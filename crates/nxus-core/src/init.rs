use std::fs::{self, create_dir_all};
use std::path::Path;
use std::process::Command as OsCommand;

use crate::config::{
    DEFAULT_BUILD_ROOT, DEFAULT_NUTTX_APPS_REV, DEFAULT_NUTTX_APPS_SRC, DEFAULT_NUTTX_REV,
    DEFAULT_NUTTX_SRC, DEFAULT_PROJECT_DEFAULT_PROFILE, DEFAULT_SIM_ARCH, DEFAULT_SIM_BOARD,
    DEFAULT_SIM_CONFIG_BASE, DEFAULT_SIM_FAMILY, DEFAULT_TEST_ARCH, DEFAULT_TEST_BOARD,
    DEFAULT_TEST_CONFIG_BASE, DEFAULT_TEST_FAMILY, DEFAULT_WORKSPACE_ROOT, SIM_PROFILE_NAME,
    TEST_PROFILE_NAME,
};
use crate::{CoreError, CoreResult};

use chrono::Datelike;
use include_dir::{include_dir, Dir, DirEntry};
use zappy_core::builtins::{PROJECT_NAME, USER, YEAR};
use zappy_core::{
    resolve_variables, Manifest, VariableResolutionInput, VariableValue, VariableValueMap,
};
use zappy_fs::{
    build_generation_plan, materialize_generation_plan, BuildPlanInput, DiscoveredTemplate,
    MaterializationOptions, TemplateSearchPath, TemplateSearchPathKind,
};

/// Project template for `init project`.
static PROJECT_TEMPLATE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/src/project_template");

/// Initializes the current directory as a config-only Nxus project.
///
/// # Errors
/// Returns [`CoreError`] when the target directory is invalid, a generated file
/// would overwrite an existing path, or an underlying I/O operation fails.
pub fn init_project_config(project_dir: &Path) -> CoreResult<()> {
    ensure_directory(project_dir)?;

    let nxus_toml = project_dir.join("nxus.toml");
    ensure_absent(&nxus_toml)?;

    write_file(&nxus_toml, &render_nxus_toml())?;

    Ok(())
}

/// Initializes a new canonical Nxus project scaffold.
///
/// # Errors
/// Returns [`CoreError`] when the target path is not usable, a destination file
/// would be overwritten, or an underlying I/O operation fails.
pub fn init_project(project_dir: &Path) -> CoreResult<()> {
    if project_dir.exists() {
        if project_dir.is_file() {
            return Err(CoreError::PathNotDir {
                path: project_dir.to_path_buf(),
            });
        }

        return Err(CoreError::PathAlreadyExists {
            path: project_dir.to_path_buf(),
        });
    }

    let temp_dir = tempfile::TempDir::new()?;

    extract_dir(&PROJECT_TEMPLATE, temp_dir.path())?;

    let template_path = temp_dir.path().to_path_buf();
    let manifest_path = template_path.join("zappy.toml");
    let manifest = Manifest::load_from_path(&manifest_path)?;

    let project_name = project_dir.file_name().map_or_else(
        || String::from("nuttx-app"),
        |name| String::from(name.to_str().unwrap_or("nuttx-app")),
    );
    let user = user_name();
    let year = chrono::Local::now().year();

    let mut builtin_variables = VariableValueMap::new();

    builtin_variables.insert(
        String::from(PROJECT_NAME),
        VariableValue::String(project_name),
    );
    builtin_variables.insert(String::from(USER), VariableValue::String(user));
    builtin_variables.insert(String::from(YEAR), VariableValue::String(year.to_string()));

    let template = DiscoveredTemplate {
        search_path: TemplateSearchPath {
            kind: TemplateSearchPathKind::Explicit,
            path: template_path.clone(),
            required: true,
        },
        template_dir: template_path,
        manifest_path,
        manifest,
    };

    let input = VariableResolutionInput {
        explicit: VariableValueMap::new(),
        interactive: VariableValueMap::new(),
        user_defaults: VariableValueMap::new(),
        builtins: builtin_variables,
    };

    let resolved = resolve_variables(&template.manifest.variables, &input)?;
    let plan_input = BuildPlanInput {
        template_dir: &template.template_dir,
        manifest: &template.manifest,
        variables: &resolved,
        output_dir: project_dir.to_path_buf(),
        force: false,
    };

    let plan = build_generation_plan(&plan_input).map_err(|error| CoreError::ZappyFs {
        error: (*error).to_string(),
    })?;

    create_dir_all(&plan.output_dir)?;
    let summary = match materialize_generation_plan(&plan, MaterializationOptions { force: false })
    {
        Ok(summary) => summary,
        Err(error) => {
            return Err(CoreError::ZappyFs {
                error: (*error).to_string(),
            });
        }
    };

    println!(
        "Generated `{}` in {}",
        &template.manifest.template.id.as_str(),
        plan.output_dir.display()
    );

    println!(
        "Created {} directories, wrote {} text files, copied {} binary files, skipped {} \
             paths.",
        summary.directories_created,
        summary.text_files_written,
        summary.binary_files_copied,
        summary.skipped,
    );

    Ok(())
}

/// Verifies that the selected initialization root is a directory.
fn ensure_directory(path: &Path) -> CoreResult<()> {
    if path.is_dir() {
        return Ok(());
    }

    Err(CoreError::PathNotDir {
        path: path.to_path_buf(),
    })
}

/// Verifies that initialization will not overwrite an existing path.
fn ensure_absent(path: &Path) -> CoreResult<()> {
    if path.exists() {
        return Err(CoreError::PathAlreadyExists {
            path: path.to_path_buf(),
        });
    }

    Ok(())
}

/// Writes a generated file, creating parent directories as needed.
fn write_file(path: &Path, contents: &str) -> CoreResult<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, contents)?;
    Ok(())
}

/// Recursively extracts en embedded directory.
fn extract_dir(dir: &Dir<'_>, destination_root: &Path) -> CoreResult<()> {
    for entry in dir.entries() {
        match *entry {
            DirEntry::Dir(ref child_dir) => {
                let destination = destination_root.join(child_dir.path());

                fs::create_dir_all(&destination)?;
                extract_dir(child_dir, destination_root)?;
            }

            DirEntry::File(ref file) => {
                let destination = destination_root.join(file.path());

                if let Some(parent) = destination.parent() {
                    fs::create_dir_all(parent)?;
                }

                fs::write(&destination, file.contents())?;
            }
        }
    }

    Ok(())
}

/// Retrieves host username.
fn user_name() -> String {
    git_user_name()
        .or_else(env_user)
        .unwrap_or_else(|| String::from("{TODO: add username}"))
}

/// Retrieves git username.
fn git_user_name() -> Option<String> {
    let output = OsCommand::new("git")
        .args(["config", "user.name"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let value = String::from_utf8_lossy(&output.stdout).trim().to_owned();

    if value.is_empty() {
        return None;
    }

    Some(value)
}

/// Retrieves env username.
fn env_user() -> Option<String> {
    std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .ok()
        .filter(|value| !value.trim().is_empty())
}

/// Renders the minimal `nxus.toml` written by init commands.
fn render_nxus_toml() -> String {
    format!(
        r#"# Global project config.
[project]
default_profile = "{DEFAULT_PROJECT_DEFAULT_PROFILE}"

# Build dir config.
[build]
root = "{DEFAULT_BUILD_ROOT}"
link_compile_commands = true

# Project-local `NuttX` workspace config.
# Pin the `nuttx` and `nuttx_apps` revisions to the baseline you need.
[workspace]
root = "{DEFAULT_WORKSPACE_ROOT}"

[workspace.nuttx]
# src = "{DEFAULT_NUTTX_SRC}"
rev = "{DEFAULT_NUTTX_REV}"

[workspace.nuttx_apps]
# src = "{DEFAULT_NUTTX_APPS_SRC}"
rev = "{DEFAULT_NUTTX_APPS_REV}"

# Sim profile overrides.
# [profile.{SIM_PROFILE_NAME}]
# arch = "{DEFAULT_SIM_ARCH}"
# family = "{DEFAULT_SIM_FAMILY}"
# board = "{DEFAULT_SIM_BOARD}"
# config_base = "{DEFAULT_SIM_CONFIG_BASE}"

# Test profile overrides.
# [profile.{TEST_PROFILE_NAME}]
# arch = "{DEFAULT_TEST_ARCH}"
# family = "{DEFAULT_TEST_FAMILY}"
# board = "{DEFAULT_TEST_BOARD}"
# config_base = "{DEFAULT_TEST_CONFIG_BASE}"

# Custom profile definitions
# [profile.prod]
# arch = "arm"
# family = "stm32f7"
# board = "nucleo-f767zi"
# config_base = "evalos"
"#
    )
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{init_project, init_project_config, load_config, CoreError};

    #[test]
    fn init_project_config_writes_minimal_config() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");

        init_project_config(temp_dir.path()).expect("config init should succeed");

        assert!(temp_dir.path().join("nxus.toml").is_file());
        assert!(load_config(temp_dir.path()).is_ok());
    }

    #[test]
    fn init_project_config_refuses_existing_generated_files() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        fs::write(temp_dir.path().join("nxus.toml"), "existing").expect("config should exist");

        assert!(matches!(
            init_project_config(temp_dir.path()),
            Err(CoreError::PathAlreadyExists { .. })
        ));
    }

    #[test]
    fn init_project_creates_canonical_scaffold() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let project_dir = temp_dir.path().join("demo");

        init_project(&project_dir).expect("project init should succeed");

        assert!(project_dir.join("nxus.toml").is_file());
        assert!(project_dir.join(".gitignore").is_file());
        assert!(project_dir.join("app").is_dir());
        assert!(project_dir.join("app/CMakeLists.txt").is_file());
        assert!(project_dir.join("app/Kconfig").is_file());
        assert!(project_dir.join("lib").is_dir());
        assert!(project_dir.join("test").is_dir());
        assert!(project_dir.join("config/common.config").is_file());
        assert!(load_config(&project_dir).is_ok());
    }

    #[test]
    fn init_project_rejects_non_empty_directory() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let project_dir = temp_dir.path().join("demo");
        fs::create_dir_all(&project_dir).expect("project dir should be created");
        fs::write(project_dir.join("README.md"), "existing").expect("file should be created");

        assert!(matches!(
            init_project(&project_dir),
            Err(CoreError::PathAlreadyExists { .. })
        ));
    }

    #[test]
    fn init_project_rejects_file_destination() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let project_file = temp_dir.path().join("demo");
        fs::write(&project_file, "file").expect("file should be created");

        assert!(matches!(
            init_project(&project_file),
            Err(CoreError::PathNotDir { .. })
        ));
    }
}

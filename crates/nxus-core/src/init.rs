use std::fs;
use std::path::Path;

use crate::{CoreError, CoreResult};

/// Default pinned revision written for the `nuttx` workspace dependency.
const DEFAULT_NUTTX_REV: &str = "master";
/// Default pinned revision written for the `nuttx-apps` workspace dependency.
const DEFAULT_NUTTX_APPS_REV: &str = "master";

/// Initializes the current directory as a config-only Nxus project.
///
/// # Errors
/// Returns [`CoreError`] when the target directory is invalid, a generated file
/// would overwrite an existing path, or an underlying I/O operation fails.
pub fn init_project_config(project_dir: &Path) -> CoreResult<()> {
    ensure_directory(project_dir)?;

    let nxus_toml = project_dir.join("nxus.toml");
    let common_config = project_dir.join("config").join("common.config");
    let sim_overlay = project_dir.join("config").join("sim.overlay");
    let test_overlay = project_dir.join("config").join("test.overlay");

    ensure_absent(&nxus_toml)?;
    ensure_absent(&common_config)?;
    ensure_absent(&sim_overlay)?;
    ensure_absent(&test_overlay)?;

    write_file(&nxus_toml, &render_nxus_toml())?;
    write_file(&common_config, COMMON_CONFIG)?;
    write_file(&sim_overlay, SIM_OVERLAY)?;
    write_file(&test_overlay, TEST_OVERLAY)?;

    Ok(())
}

/// Initializes a new canonical Nxus project scaffold.
///
/// # Errors
/// Returns [`CoreError`] when the target path is not usable, a destination file
/// would be overwritten, or an underlying I/O operation fails.
pub fn init_project(project_dir: &Path) -> CoreResult<()> {
    if project_dir.exists() {
        ensure_directory(project_dir)?;
        ensure_empty_directory(project_dir)?;
    } else {
        fs::create_dir_all(project_dir)?;
    }

    let nxus_toml = project_dir.join("nxus.toml");
    let gitignore = project_dir.join(".gitignore");
    let app_root = project_dir.join("app");
    let cmake_lists = app_root.join("CMakeLists.txt");
    let kconfig = app_root.join("Kconfig");
    let common_config = app_root.join("config").join("common.config");
    let sim_overlay = app_root.join("config").join("sim.overlay");
    let test_overlay = app_root.join("config").join("test.overlay");

    ensure_absent(&nxus_toml)?;
    ensure_absent(&gitignore)?;
    ensure_absent(&cmake_lists)?;
    ensure_absent(&kconfig)?;
    ensure_absent(&common_config)?;
    ensure_absent(&sim_overlay)?;
    ensure_absent(&test_overlay)?;

    fs::create_dir_all(app_root.join("app"))?;
    fs::create_dir_all(app_root.join("lib"))?;
    fs::create_dir_all(app_root.join("test"))?;

    write_file(&nxus_toml, &render_nxus_toml())?;
    write_file(&gitignore, GITIGNORE)?;
    write_file(&cmake_lists, CMAKE_LISTS)?;
    write_file(&kconfig, KCONFIG)?;
    write_file(&common_config, COMMON_CONFIG)?;
    write_file(&sim_overlay, SIM_OVERLAY)?;
    write_file(&test_overlay, TEST_OVERLAY)?;

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

/// Verifies that an existing directory contains no files before scaffolding.
fn ensure_empty_directory(path: &Path) -> CoreResult<()> {
    if fs::read_dir(path)?.next().is_none() {
        return Ok(());
    }

    Err(CoreError::DirectoryNotEmpty {
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

/// Renders the minimal `nxus.toml` written by init commands.
fn render_nxus_toml() -> String {
    format!(
        "# Nxus project configuration.\n# Pin these revisions to the NuttX baseline you \
         validate.\n\n[workspace.nuttx]\nrev = \
         \"{DEFAULT_NUTTX_REV}\"\n\n[workspace.nuttx_apps]\nrev = \"{DEFAULT_NUTTX_APPS_REV}\"\n"
    )
}

/// Default project `.gitignore` content for Nxus-managed outputs.
const GITIGNORE: &str = "build/\nworkspace/\n";
/// Minimal application root `CMakeLists.txt` scaffold.
const CMAKE_LISTS: &str = "# Nxus-managed NuttX application root.\n";
/// Minimal application root `Kconfig` scaffold.
const KCONFIG: &str = "menu \"Nxus Project\"\nendmenu\n";
/// Shared profile config overlay scaffold.
const COMMON_CONFIG: &str = "# Shared configuration applied to every Nxus profile.\n";
/// Default simulator overlay scaffold.
const SIM_OVERLAY: &str = "# Extra settings for the default simulator profile.\n";
/// Default test overlay scaffold.
const TEST_OVERLAY: &str = "# Extra settings for the default test profile.\n";

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::{CoreError, init_project, init_project_config, load_config};

    #[test]
    fn init_project_config_writes_minimal_files() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");

        init_project_config(temp_dir.path()).expect("config init should succeed");

        assert!(temp_dir.path().join("nxus.toml").is_file());
        assert!(temp_dir.path().join("config/common.config").is_file());
        assert!(temp_dir.path().join("config/sim.overlay").is_file());
        assert!(temp_dir.path().join("config/test.overlay").is_file());
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
        assert!(project_dir.join("app/CMakeLists.txt").is_file());
        assert!(project_dir.join("app/Kconfig").is_file());
        assert!(project_dir.join("app/app").is_dir());
        assert!(project_dir.join("app/lib").is_dir());
        assert!(project_dir.join("app/test").is_dir());
        assert!(project_dir.join("app/config/common.config").is_file());
        assert!(load_config(&project_dir).is_ok());
    }

    #[test]
    fn init_project_allows_existing_empty_directory() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let project_dir = temp_dir.path().join("demo");
        fs::create_dir_all(&project_dir).expect("project dir should be created");

        init_project(&project_dir).expect("project init should succeed");

        assert!(project_dir.join("app").is_dir());
    }

    #[test]
    fn init_project_rejects_non_empty_directory() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let project_dir = temp_dir.path().join("demo");
        fs::create_dir_all(&project_dir).expect("project dir should be created");
        fs::write(project_dir.join("README.md"), "existing").expect("file should be created");

        assert!(matches!(
            init_project(&project_dir),
            Err(CoreError::DirectoryNotEmpty { .. })
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

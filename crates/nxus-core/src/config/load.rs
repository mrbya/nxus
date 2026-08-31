use std::fs;
use std::path::Path;

use crate::config::NxusConfig;
use crate::{CoreError, CoreResult};

/// Loads `nxus.toml` configuration for a project.
///
/// # Errors
/// Returns [`CoreError::Io`] on I/O operation failures and
/// [`CoreError::ParseConfig`] if fails to parse toml config.
pub fn load_config(project_dir: &Path) -> CoreResult<NxusConfig> {
    let path = project_dir.join("nxus.toml");
    let config_str = fs::read_to_string(&path).map_err(CoreError::Io)?;
    let parsed = toml::from_str::<NxusConfig>(&config_str)
        .map_err(|source| CoreError::ParseConfig { path, source })?;

    let config = NxusConfig::new().overlay(parsed);

    Ok(config)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::{Path, PathBuf};

    use crate::config::DEFAULT_PROJECT_DEFAULT_PROFILE;
    use crate::{CommandConfig, CoreError, load_config};

    fn write_config(dir: &Path) -> PathBuf {
        let file_path = dir.join("nxus.toml");
        fs::write(
            &file_path,
            r#"[project]
default_profile = "sim"

[build]
root = "build"
link_compile_commands = true

[workspace]
root = "workspace"

[workspace.nuttx]

[workspace.nuttx_apps]

[profile.prod]
arch = "arm"
family = "stm32f7"
board = "nucleo-f767zi"
config_base = "evalos"

[profile.test]
arch = "sim"
family = "kung"
board = "foo"
config_base = "bar"
"#,
        )
        .expect("config file should be created");
        file_path
    }

    #[test]
    fn load_config_success() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let _ = write_config(temp_dir.path());

        assert_eq!(
            load_config(temp_dir.path())
                .expect("config should load")
                .project
                .default_profile,
            Some(String::from(DEFAULT_PROJECT_DEFAULT_PROFILE))
        );
    }

    #[test]
    fn load_io_error_on_missing_file() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");

        assert!(matches!(
            load_config(temp_dir.path()),
            Err(CoreError::Io(_))
        ));
    }

    #[test]
    fn load_parse_error_on_invalid_toml() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        fs::write(temp_dir.path().join("nxus.toml"), "not = [")
            .expect("config file should be created");

        assert!(matches!(
            load_config(temp_dir.path()),
            Err(CoreError::ParseConfig { .. })
        ));
    }

    #[test]
    fn load_config_parses_structured_flash_command() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        fs::write(
            temp_dir.path().join("nxus.toml"),
            r#"[workspace.nuttx]
rev = "master"

[workspace.nuttx_apps]
rev = "master"

[profile.prod]
arch = "arm"
family = "stm32f7"
board = "nucleo-f767zi"
config_base = "evalos"

[profile.prod.flash]
command = "openocd"
args = ["-c", "program {elf} verify reset exit"]
"#,
        )
        .expect("config file should be created");

        let flash = load_config(temp_dir.path())
            .expect("config should load")
            .profiles
            .get("prod")
            .and_then(|profile| profile.flash.clone());

        assert_eq!(
            flash,
            Some(CommandConfig {
                command: String::from("openocd"),
                args: vec![
                    String::from("-c"),
                    String::from("program {elf} verify reset exit")
                ],
            })
        );
    }
}

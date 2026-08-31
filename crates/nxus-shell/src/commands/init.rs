use std::env;
use std::process::ExitCode;

use nxus_core::{CoreError, init_project, init_project_config};

use crate::cli::{InitArgs, InitCommand};

/// Nxus command: init.
pub fn init(args: &InitArgs) -> ExitCode {
    let cwd = match env::current_dir() {
        Ok(cwd) => cwd,
        Err(error) => {
            eprintln!("{}", CoreError::Io(error));
            return ExitCode::FAILURE;
        }
    };

    let result = match args.command.clone() {
        InitCommand::Config => init_project_config(&cwd),
        InitCommand::Project { path } => {
            let target = path
                .as_ref()
                .map_or_else(|| cwd.clone(), |path| cwd.join(path));
            init_project(&target)
        }
    };

    if let Err(error) = result {
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use std::process::ExitCode;
    use std::{env, fs};

    use crate::cli::{InitArgs, InitCommand};
    use crate::commands::init;

    #[test]
    fn init_config_creates_expected_files_in_current_directory() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let prev_dir = env::current_dir().expect("cwd should be available");
        env::set_current_dir(temp_dir.path()).expect("cwd should change");

        let result = init(&InitArgs {
            command: InitCommand::Config,
        });

        env::set_current_dir(prev_dir).expect("cwd should restore");

        assert_eq!(result, ExitCode::SUCCESS);
        assert!(temp_dir.path().join("nxus.toml").is_file());
        assert!(temp_dir.path().join("config/common.config").is_file());
    }

    #[test]
    fn init_project_uses_relative_destination_path() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let prev_dir = env::current_dir().expect("cwd should be available");
        env::set_current_dir(temp_dir.path()).expect("cwd should change");

        let result = init(&InitArgs {
            command: InitCommand::Project {
                path: Some("demo".into()),
            },
        });

        env::set_current_dir(prev_dir).expect("cwd should restore");

        assert_eq!(result, ExitCode::SUCCESS);
        assert!(temp_dir.path().join("demo/app").is_dir());
    }

    #[test]
    fn init_project_fails_for_non_empty_current_directory() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let prev_dir = env::current_dir().expect("cwd should be available");
        fs::write(temp_dir.path().join("README.md"), "existing").expect("file should exist");
        env::set_current_dir(temp_dir.path()).expect("cwd should change");

        let result = init(&InitArgs {
            command: InitCommand::Project { path: None },
        });

        env::set_current_dir(prev_dir).expect("cwd should restore");

        assert_eq!(result, ExitCode::FAILURE);
    }
}

use std::process::ExitCode;

use nxus_core::{Cmd, CoreError, ResolvedConfig, Runner, paths};

use crate::commands::build;

/// Nxus command: run.
pub fn run_binary(cfg: &ResolvedConfig) -> ExitCode {
    let build_dir = paths::build_dir(cfg, &cfg.profile);
    let buid_dir_present = build_dir.exists();

    if buid_dir_present && !build_dir.is_dir() {
        eprintln!("{}", CoreError::PathNotDir { path: build_dir });
        return ExitCode::FAILURE;
    }

    if !buid_dir_present && build(cfg) == ExitCode::FAILURE {
        return ExitCode::FAILURE;
    }

    let cmd = Cmd::new(build_dir.join("nuttx"));
    let runner = Runner {
        verbose: 3,
        dry_run: cfg.runner.dry_run,
    };

    if let Err(error) = runner.run(&cmd, "") {
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::ExitCode;

    use crate::commands::run_binary;
    use crate::tests::resolved_config;

    #[test]
    fn run_binary_fails_when_build_dir_is_a_file() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        fs::create_dir_all(cfg.build_dir.parent().expect("build parent should exist"))
            .expect("build parent should be created");
        fs::write(&cfg.build_dir, "file").expect("build path file should be created");

        assert_eq!(run_binary(&cfg), ExitCode::FAILURE);
    }

    #[test]
    fn run_binary_succeeds_in_dry_run_when_build_dir_exists() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        fs::create_dir_all(&cfg.build_dir).expect("build dir should be created");

        assert_eq!(run_binary(&cfg), ExitCode::SUCCESS);
    }
}

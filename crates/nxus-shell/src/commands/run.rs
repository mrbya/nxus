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

use std::process::ExitCode;

use nxus_core::{Cmd, CoreError, ResolvedConfig, Runner, paths};

use crate::commands::config;

/// Nxus command: menuconfig.
pub fn menuconfig(cfg: &ResolvedConfig) -> ExitCode {
    let build_dir = paths::build_dir(cfg, &cfg.profile);
    let build_dir_present = build_dir.exists();

    if build_dir_present && !build_dir.is_dir() {
        eprintln!("{}", CoreError::PathNotDir { path: build_dir });
        return ExitCode::FAILURE;
    }

    if !build_dir_present && config(cfg) == ExitCode::FAILURE {
        return ExitCode::FAILURE;
    }

    let cmd = Cmd::new("ninja").arg("-C").arg(build_dir).arg("menuconfig");
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

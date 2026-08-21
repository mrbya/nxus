use std::process::ExitCode;

use nxus_core::ResolvedConfig;

use crate::commands::run_binary;

/// Nxus command: sim
pub fn sim(cfg: &ResolvedConfig) -> ExitCode {
    let config = cfg.with_profile("sim");

    if run_binary(&config) == ExitCode::FAILURE {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

use std::process::ExitCode;

use nxus_core::ResolvedConfig;

use crate::commands::run_binary;

/// Nxus command: test
pub fn test(cfg: &ResolvedConfig) -> ExitCode {
    let config = cfg.with_profile("test");

    if run_binary(&config) == ExitCode::FAILURE {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

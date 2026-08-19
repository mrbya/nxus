use std::{fs, process::ExitCode};

use nxus_core::ResolvedConfig;

/// Nxus command: clean
pub fn clean(cfg: ResolvedConfig, profile: Option<&String>) -> ExitCode {
    if profile.is_some() {
        match fs::remove_dir_all(cfg.build_dir) {
            Ok(()) => return ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("{error}");
                return ExitCode::FAILURE;
            }
        }
    }
    match fs::remove_dir_all(cfg.build_root) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}

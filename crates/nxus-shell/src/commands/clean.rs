use std::fs;
use std::process::ExitCode;

use nxus_core::{ResolvedConfig, unlink_app, unlink_config};

/// Nxus command: clean
pub fn clean(cfg: &ResolvedConfig) -> ExitCode {
    let mut err = false;

    if cfg.profile_selected {
        if let Err(error) = unlink_config(cfg, &cfg.profile, cfg.profiles.get(&cfg.profile)) {
            eprintln!("{error}");
            err = true;
        }

        if cfg.build_dir.exists() {
            if let Err(error) = fs::remove_dir_all(&cfg.build_dir) {
                eprintln!("{error}");
                err = true;
            }
        }
    } else {
        if let Err(error) = unlink_app(cfg) {
            eprintln!("{error}");
            err = true;
        }

        for (profile_name, profile) in &cfg.profiles {
            if let Err(error) = unlink_config(cfg, profile_name, Some(profile)) {
                eprintln!("{error}");
                err = true;
            }
        }

        if cfg.build_root.exists() {
            if let Err(error) = fs::remove_dir_all(&cfg.build_root) {
                eprintln!("{error}");
                err = true;
            }
        }
    }

    if err {
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::ExitCode;

    use nxus_core::paths;

    use crate::commands::clean;
    use crate::tests::resolved_config;

    #[test]
    fn clean_removes_selected_profile_build_dir() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let mut cfg = resolved_config(temp_dir.path());
        cfg.profile_selected = true;

        fs::create_dir_all(&cfg.build_dir).expect("build dir should be created");
        fs::create_dir_all(paths::board_config_root(&cfg)).expect("board config root should exist");

        assert_eq!(clean(&cfg), ExitCode::SUCCESS);
        assert!(!cfg.build_dir.exists());
    }

    #[test]
    fn clean_removes_build_root_when_no_profile_is_selected() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        fs::create_dir_all(&cfg.build_root).expect("build root should be created");

        assert_eq!(clean(&cfg), ExitCode::SUCCESS);
        assert!(!cfg.build_root.exists());
    }
}

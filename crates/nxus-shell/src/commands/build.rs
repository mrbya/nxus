use std::process::ExitCode;

use nxus_core::{Cmd, CoreError, ResolvedConfig, link_compile_commands, paths};

use crate::commands::config;

/// Nxus command: build.
pub fn build(cfg: &ResolvedConfig) -> ExitCode {
    let build_dir = paths::build_dir(cfg, &cfg.profile);
    let build_dir_present = build_dir.exists();

    if build_dir_present && !build_dir.is_dir() {
        eprintln!("{}", CoreError::PathNotDir { path: build_dir });
        return ExitCode::FAILURE;
    }

    let ran_config = if build_dir_present {
        false
    } else {
        if config(cfg) == ExitCode::FAILURE {
            return ExitCode::FAILURE;
        }
        true
    };

    let cmd = Cmd::new("ninja").arg("-C").arg(build_dir);

    if let Err(error) = cfg
        .runner
        .run(&cmd, &format!("Building project for `{}`", cfg.profile))
    {
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }

    if cfg.link_compile_commands
        && !ran_config
        && let Err(error) = link_compile_commands(cfg, &cfg.profile)
    {
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::ExitCode;

    use nxus_core::paths;

    use crate::commands::build;
    use crate::tests::resolved_config;

    #[test]
    fn build_fails_when_build_dir_is_a_file() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        fs::create_dir_all(cfg.build_dir.parent().expect("build parent should exist"))
            .expect("build parent should be created");
        fs::write(&cfg.build_dir, "file").expect("build dir placeholder should be created");

        assert_eq!(build(&cfg), ExitCode::FAILURE);
    }

    #[test]
    fn build_succeeds_in_dry_run_when_build_dir_exists() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        fs::create_dir_all(&cfg.build_dir).expect("build dir should be created");
        fs::write(
            paths::build_dir(&cfg, &cfg.profile).join("compile_commands.json"),
            "{}",
        )
        .expect("file should be written");

        assert_eq!(build(&cfg), ExitCode::SUCCESS);
    }
}

use std::process::ExitCode;

use nxus_core::{paths, resolve_flash_command, CoreError, ResolvedConfig};

use crate::commands::build;

/// Nxus command: flash.
pub fn flash(cfg: &ResolvedConfig) -> ExitCode {
    let build_dir = paths::build_dir(cfg, &cfg.profile);
    let binary = paths::firmware_elf(cfg, &cfg.profile);
    let build_dir_present = build_dir.exists();
    let binary_present = binary.exists();

    if build_dir_present && !build_dir.is_dir() {
        eprintln!("{}", CoreError::PathNotDir { path: build_dir });
        return ExitCode::FAILURE;
    }

    if binary_present && !binary.is_file() {
        eprintln!("{}", CoreError::PathNotDir { path: binary });
        return ExitCode::FAILURE;
    }

    if !binary_present && build(cfg) == ExitCode::FAILURE {
        return ExitCode::FAILURE;
    }

    if binary_present && cfg.rebuild && build(cfg) == ExitCode::FAILURE {
        return ExitCode::FAILURE;
    }

    let cmd = match resolve_flash_command(cfg) {
        Ok(cmd) => cmd,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };

    if let Err(error) = cfg
        .runner
        .run(&cmd, &format!("Flashing project for `{}`", cfg.profile))
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

    use crate::commands::flash;
    use crate::tests::{flash_command, resolved_config};

    #[test]
    fn flash_fails_when_build_dir_is_a_file() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        fs::create_dir_all(cfg.build_dir.parent().expect("build parent should exist"))
            .expect("build parent should be created");
        fs::write(&cfg.build_dir, "file").expect("build path file should be created");

        assert_eq!(flash(&cfg), ExitCode::FAILURE);
    }

    #[test]
    fn flash_dry_run_prints_resolved_programmer_command() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let mut cfg = resolved_config(temp_dir.path());
        cfg.runner.verbose = 3;
        fs::create_dir_all(&cfg.build_dir).expect("build dir should be created");
        fs::write(paths::firmware_elf(&cfg, &cfg.profile), "elf").expect("elf should exist");
        cfg.flash = Some(flash_command(
            "openocd",
            &["-c", "program {elf} verify reset exit"],
        ));

        assert_eq!(flash(&cfg), ExitCode::SUCCESS);
    }

    #[test]
    fn flash_propagates_runner_failure_after_successful_build_step() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let mut cfg = resolved_config(temp_dir.path());
        cfg.runner.dry_run = false;
        cfg.runner.verbose = 0;
        fs::create_dir_all(&cfg.build_dir).expect("build dir should be created");
        fs::write(
            cfg.build_dir.join("build.ninja"),
            "rule noop\nbuild all: phony\ndefault all\n",
        )
        .expect("build.ninja should exist");
        fs::write(paths::firmware_elf(&cfg, &cfg.profile), "elf").expect("elf should exist");
        cfg.flash = Some(flash_command("definitely-does-not-exist", &["{elf}"]));

        assert_eq!(flash(&cfg), ExitCode::FAILURE);
    }
}

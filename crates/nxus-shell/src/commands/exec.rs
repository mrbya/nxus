use std::process::ExitCode;

use nxus_core::{CoreError, ResolvedConfig, resolve_command};

use crate::cli::ExecArgs;

/// Nxus command: exec.
pub fn exec(cfg: &ResolvedConfig, args: &ExecArgs) -> ExitCode {
    let Some(command) = cfg.commands.get(&args.name) else {
        eprintln!(
            "{}",
            CoreError::UnknownProjectCommand {
                command: args.name.clone(),
            }
        );
        return ExitCode::FAILURE;
    };

    let mut cmd = match resolve_command(cfg, command) {
        Ok(cmd) => cmd,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };

    cmd.args.extend(args.args.iter().cloned());

    if let Err(error) = cfg.runner.run(
        &cmd,
        &format!(
            "Executing project command `{}` for `{}`",
            args.name, cfg.profile
        ),
    ) {
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use std::ffi::OsString;
    use std::fs;
    use std::process::ExitCode;

    use crate::commands::exec;
    use crate::tests::{flash_command, resolved_config};

    #[test]
    fn exec_fails_when_command_is_not_configured() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        assert_eq!(
            exec(
                &cfg,
                &crate::cli::ExecArgs {
                    name: String::from("missing"),
                    args: vec![],
                },
            ),
            ExitCode::FAILURE
        );
    }

    #[test]
    fn exec_appends_runtime_args_after_configured_args() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let mut cfg = resolved_config(temp_dir.path());
        fs::create_dir_all(&cfg.build_dir).expect("build dir should be created");
        cfg.commands.insert(
            String::from("foo"),
            flash_command("tool", &["configured", "{profile}"]),
        );

        assert_eq!(
            exec(
                &cfg,
                &crate::cli::ExecArgs {
                    name: String::from("foo"),
                    args: vec![OsString::from("runtime"), OsString::from("--flag")],
                },
            ),
            ExitCode::SUCCESS
        );
    }
}

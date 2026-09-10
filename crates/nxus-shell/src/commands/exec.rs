use std::process::ExitCode;

use nxus_core::{CoreError, ResolvedConfig, command_info, resolve_command};

use crate::cli::ExecArgs;

/// Reserved `exec` pseudo-command used for project command discovery.
const LIST_COMMAND_NAME: &str = "list";

/// Nxus command: exec.
pub fn exec(cfg: &ResolvedConfig, args: &ExecArgs) -> ExitCode {
    if args.name == LIST_COMMAND_NAME {
        return list(cfg);
    }

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
    let mut runner = cfg.runner;
    runner.verbose = if runner.verbose < 2 { 1 } else { 3 };

    if let Err(error) = runner.run(
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

/// Lists configured project commands without executing them.
fn list(cfg: &ResolvedConfig) -> ExitCode {
    let commands = match command_info(&cfg.ctx.project_dir, &cfg.commands) {
        Ok(commands) => commands,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };

    if commands.is_empty() {
        println!("No project commands configured.");
        return ExitCode::SUCCESS;
    }

    println!("Available commands:");

    let width = commands
        .iter()
        .map(|command| command.name.len())
        .max()
        .unwrap_or(0);

    for command in commands {
        match command.description {
            Some(description) => {
                println!("    {:width$}  {description}", command.name, width = width);
            }
            None => println!("    {}", command.name),
        }
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

    #[test]
    fn exec_list_succeeds_when_no_project_commands_are_configured() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        fs::write(
            temp_dir.path().join("nxus.toml"),
            "[project]\ndefault_profile = \"sim\"\n",
        )
        .expect("config should be written");
        let cfg = resolved_config(temp_dir.path());

        assert_eq!(
            exec(
                &cfg,
                &crate::cli::ExecArgs {
                    name: String::from("list"),
                    args: vec![],
                },
            ),
            ExitCode::SUCCESS
        );
    }

    #[test]
    fn exec_list_is_reserved_over_configured_project_command() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        fs::write(
            temp_dir.path().join("nxus.toml"),
            r#"# Reserved command.
[command.list]
command = "tool"

[command.docs]
command = "doxide"
"#,
        )
        .expect("config should be written");
        let mut cfg = resolved_config(temp_dir.path());
        cfg.commands
            .insert(String::from("list"), flash_command("tool", &[]));
        cfg.commands
            .insert(String::from("docs"), flash_command("doxide", &[]));

        assert_eq!(
            exec(
                &cfg,
                &crate::cli::ExecArgs {
                    name: String::from("list"),
                    args: vec![OsString::from("--ignored")],
                },
            ),
            ExitCode::SUCCESS
        );
    }
}

use std::path::PathBuf;
use std::process::ExitCode;

use clap::{Args, Parser, Subcommand};
use nxus_core::{discover_config, load_config, ResolvedConfig};

use crate::commands::{
    build, clean, config, flash, init, menuconfig, profiles, run_binary, sim, test, workspace,
};

/// `nxus` CLI parser.
#[derive(Debug, Parser)]
#[command(
    name = "nxus",
    about = "CLI build system companion for opinionated NuttX projects",
    propagate_version = true,
    after_help = r#"Examples:
    nxus init config
    nxus init project demo
    nxus build
    nxus -p prod flash
    nxus menuconfig
    nxus sim
    nxus test

For more info, see "https://gitlab.com/byacrates/nxus"
"#
)]
#[command(version, about)]
pub struct Cli {
    /// Pre-clean build dir for the given profile.
    #[arg(short = 'c', long)]
    pub clean: bool,

    /// Rebuild binary for selected profile before running/flashing.
    #[arg(short = 'r', long)]
    pub rebuild: bool,

    /// Verbosity (repeatable).
    #[arg(short = 'v', action = clap::ArgAction::Count, default_value_t = 2)]
    pub verbose: u8,

    /// Dry run command?
    #[arg(short = 'd', long = "dry-run")]
    pub dry_run: bool,

    /// Profile to run command for.
    #[arg(short = 'p', long)]
    pub profile: Option<String>,

    /// Passed in command.
    #[command(subcommand)]
    pub command: Command,
}

/// `nxus` commands.
#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Cleans build artifacts and workspace.
    #[command(alias = "c")]
    Clean,

    /// Configures `NuttX` for a specific profile.
    #[command(alias = "cf")]
    Config,

    /// Builds project for a specific profile.
    #[command(alias = "b")]
    Build,

    /// Opens Kconfig config TUI for a specific profile.
    #[command(alias = "m")]
    Menuconfig,

    /// Runs binary built for a specific profile.
    #[command(alias = "r")]
    Run,

    /// Flashes binary for a specific profile.
    #[command(alias = "f")]
    Flash,

    /// Runs simulation in default simulation profile.
    #[command(alias = "s")]
    Sim,

    /// Runs test suite for the default test profile.
    #[command(alias = "t")]
    Test,

    /// Project-local `NuttX` workspace management.
    #[command(alias = "ws")]
    Workspace(WsArgs),

    /// Initializes `NuttX` project with `nxus.toml`.
    #[command(alias = "i")]
    Init(InitArgs),

    /// Lists available profiles.
    #[command(alias = "p")]
    Profiles,
}

/// Workspace command args.
#[derive(Clone, Args, Debug)]
pub struct WsArgs {
    /// Workspace management subcommand.
    #[command(subcommand)]
    pub command: WsCommand,
}

/// Workspace subcommands.
#[derive(Clone, Debug, Subcommand)]
pub enum WsCommand {
    /// Clean workspace.
    #[command(alias = "c")]
    Clean,

    /// Initialize workspace.
    #[command(alias = "i")]
    Init,

    /// Prune workspace.
    #[command(alias = "p")]
    Prune,
}

/// Init command args.
#[derive(Clone, Args, Debug)]
pub struct InitArgs {
    /// Initialization subcommand.
    #[command(subcommand)]
    pub command: InitCommand,
}

/// Init subcommands.
#[derive(Clone, Debug, Subcommand)]
pub enum InitCommand {
    /// Initialize Nxus config files in the current directory.
    Config,

    /// Scaffold a new Nxus project at the given path.
    Project {
        /// Optional destination path. Defaults to the current directory.
        path: Option<PathBuf>,
    },
}

/// Runs `nxus` CLI.
#[must_use]
pub fn run() -> ExitCode {
    let Cli {
        clean: clean_requested,
        rebuild,
        verbose,
        dry_run,
        profile,
        command,
    } = Cli::parse();

    if let Command::Init(args) = command.clone() {
        return init(&args);
    }

    let ctx = match discover_config(None) {
        Ok(ctx) => ctx,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };

    let cfg = match load_config(&ctx.project_dir) {
        Ok(cfg) => cfg,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };

    let resolved = match ResolvedConfig::resolve(
        clean_requested,
        rebuild,
        verbose,
        dry_run,
        &ctx,
        profile.as_ref(),
        &cfg,
    ) {
        Ok(resolved) => resolved,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    };

    if resolved.clean && resolved.profile_selected && clean(&resolved) == ExitCode::FAILURE {
        return ExitCode::FAILURE;
    }

    match command {
        Command::Profiles => profiles(&resolved),
        Command::Clean => clean(&resolved),
        Command::Config => config(&resolved),
        Command::Build => build(&resolved),
        Command::Menuconfig => menuconfig(&resolved),
        Command::Run => run_binary(&resolved),
        Command::Flash => flash(&resolved),
        Command::Sim => sim(&resolved),
        Command::Test => test(&resolved),
        Command::Workspace(args) => workspace(&resolved, &args),
        Command::Init(_) => ExitCode::FAILURE,
    }
}

#[cfg(test)]
mod tests {
    use clap::Parser;

    use crate::cli::{Cli, Command, InitArgs, InitCommand, WsCommand};

    #[test]
    fn parse_build_command_with_global_flags() {
        let cli = Cli::try_parse_from(["nxus", "-c", "-vv", "-d", "-p", "prod", "build"])
            .expect("cli should parse");

        assert!(cli.clean);
        assert_eq!(cli.verbose, 2);
        assert!(cli.dry_run);
        assert_eq!(cli.profile, Some(String::from("prod")));
        assert!(matches!(cli.command, Command::Build));
    }

    #[test]
    fn parse_workspace_alias_and_subcommand_alias() {
        let cli = Cli::try_parse_from(["nxus", "ws", "p"]).expect("cli should parse");

        assert!(matches!(
            cli.command,
            Command::Workspace(crate::cli::WsArgs {
                command: WsCommand::Prune
            })
        ));
    }

    #[test]
    fn parse_profiles_alias() {
        let cli = Cli::try_parse_from(["nxus", "p"]).expect("cli should parse");

        assert!(matches!(cli.command, Command::Profiles));
    }

    #[test]
    fn parse_init_config_subcommand() {
        let cli = Cli::try_parse_from(["nxus", "init", "config"]).expect("cli should parse");

        assert!(matches!(
            cli.command,
            Command::Init(InitArgs {
                command: InitCommand::Config
            })
        ));
    }

    #[test]
    fn parse_init_project_with_optional_path() {
        let cli = Cli::try_parse_from(["nxus", "i", "project", "demo"]).expect("cli should parse");

        assert!(matches!(
            cli.command,
            Command::Init(InitArgs {
                command: InitCommand::Project { path: Some(_) }
            })
        ));
    }

    #[test]
    fn parse_init_project_without_path() {
        let cli = Cli::try_parse_from(["nxus", "init", "project"]).expect("cli should parse");

        assert!(matches!(
            cli.command,
            Command::Init(InitArgs {
                command: InitCommand::Project { path: None }
            })
        ));
    }
}

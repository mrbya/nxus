use std::process::ExitCode;

use clap::{Parser, Subcommand};
use nxus_core::{discover_config, load_config, ResolvedConfig};

use crate::commands::{clean, profiles};

/// `nxus` CLI parser.
#[derive(Debug, Parser)]
#[command(
    name = "nxus",
    about = "CLI build system companion for opinionated NuttX projects",
    propagate_version = true,
    after_help = r#"Examples:
    nxus build
    nxus menuconfig
    nxus run sim
    nxus test

For more info, see "https://gitlab.com/byacrates/nxus"
"#
)]
#[command(version, about)]
pub struct Cli {
    /// Pre-clean build dir for the given profile.
    #[arg(short = 'c', long)]
    pub clean: bool,

    /// Verbosity (repeatable).
    #[arg(short = 'v', long = "verbose", action = clap::ArgAction::Count, default_value_t = 2)]
    pub verbose: u8,

    /// Profile to run command for.
    #[arg(short = 'p', long)]
    pub profile: Option<String>,

    /// Passed in command.
    #[command(subcommand)]
    pub command: Command,
}

/// `nxus` commands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Cleans build artifacts and workspace.
    #[command(alias = "c")]
    Clean,

    /// Configures `NuttX` for a specific profile.
    #[command(alias = "cf")]
    Conf,

    /// Builds project for a specific profile.
    #[command(alias = "b")]
    Build,

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
    Workspace,

    /// Initializes `NuttX` project with `nxus.toml`.
    #[command(alias = "i")]
    Init,

    /// Lists available profiles.
    #[command(alias = "p")]
    Profiles,
}

/// Runs `nxus` CLI.
#[must_use]
pub fn run() -> ExitCode {
    let cli = Cli::parse();

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

    let resolved =
        match ResolvedConfig::resolve(cli.clean, cli.verbose, &ctx, cli.profile.as_ref(), &cfg) {
            Ok(resolved) => resolved,
            Err(error) => {
                eprintln!("{error}");
                return ExitCode::FAILURE;
            }
        };

    match cli.command {
        Command::Profiles => profiles(resolved),
        Command::Clean => clean(resolved),
        _ => ExitCode::FAILURE,
    }
}

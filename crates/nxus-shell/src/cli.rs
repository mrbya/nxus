use std::process::ExitCode;

use clap::{Parser, Subcommand};

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
}

/// Runs `nxus` CLI.
#[must_use]
pub fn run() -> ExitCode {
    let _ = Cli::parse();

    //match cli.command {
    //    Command::Greet(args) => commands::greet(args),
    //    Command::NewCommand => {
    //        eprintln!("Command unknown or not implemented yet");
    //        ExitCode::FAILURE
    //    }
    //    _ => ExitCode::FAILURE,
    //}

    ExitCode::FAILURE
}

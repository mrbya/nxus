//! `nxus` CLI commands.
//!
//! Implements `nxus`'s command handlers and re-exports them for the shell
//! crate.

/// Builds project for a specific profile.
pub mod build;
/// Cleans up all build artifacts and workspace.
pub mod clean;
/// Configures `NuttX` for a specific profile.
pub mod config;
/// Executes a configured project command.
pub mod exec;
/// Flashes project binary built for a profile.
pub mod flash;
/// Initializes a Nxus project or config layout.
pub mod init;
/// Opens menuconfig for a specific profile.
pub mod menuconfig;
/// Liests profiles configured for project.
pub mod profiles;
/// Runs project binary built for a specific profile.
pub mod run;
/// Runs project simulation.
pub mod sim;
/// Runs project tests.
pub mod test;
/// Manages project-local nuttx workspace .
pub mod workspace;

// Command re-exports for cli parser,
pub use build::build;
pub use clean::clean;
pub use config::config;
pub use exec::exec;
pub use flash::flash;
pub use init::init;
pub use menuconfig::menuconfig;
pub use profiles::profiles;
pub use run::run_binary;
pub use sim::sim;
pub use test::test;
pub use workspace::workspace;

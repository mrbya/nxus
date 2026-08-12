//! `nxus` CLI commands.
//!
//! Implements `nxus`'s command handlers and re-exports them for the shell
//! crate.

/// Builds project for a specific profile.
pub mod build;
/// Cleans up all build artifacts and workspace.
pub mod clean;
/// Configures `NuttX` for a specific profile.
pub mod conf;
/// Flashes project binary built for a profile.
pub mod flash;
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

// Command re-exports for cli parser,

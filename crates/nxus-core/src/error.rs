use std::path::PathBuf;

use thiserror::Error;

use crate::ExecError;

/// Result type alias used by `nxus-core`.
pub type CoreResult<T> = std::result::Result<T, CoreError>;

/// Core-domain errors.
#[derive(Debug, Error)]
pub enum CoreError {
    /// Generic nxus execution error.
    #[error("nxus encountered an error: {msg}")]
    Generic {
        /// User-facing diagnostic error message.
        msg: String,
    },

    /// Io error.
    #[error(transparent)]
    Io(#[from] std::io::Error),

    /// Command execution error.
    #[error(transparent)]
    Exec(#[from] ExecError),

    /// Failed to find config file.
    #[error("`nxus.toml` config not found, search started in: {start}")]
    ConfigNotFound {
        /// Directory where the upward search started.
        start: PathBuf,
    },

    /// Failed to parse toml config file.
    #[error("failed to parse config file `{path}`: {source}")]
    ParseConfig {
        /// Path to config file being parsed.
        path: PathBuf,

        /// Underlying toml parsing error.
        source: toml::de::Error,
    },

    /// Unknown profile selected.
    #[error("unknown profile: `{profile}`; use `--profiles` to list available profiles")]
    UnknownProfile {
        /// Unknown profile name.
        profile: String,
    },

    /// Failed to resolve a config value.
    #[error("failed to resolve `{value}`, make sure `nxus.toml` sets it up correctly")]
    Resolve {
        /// Config value that failed to be resolved.
        value: String,
    },
}

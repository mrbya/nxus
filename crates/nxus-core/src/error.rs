use std::path::PathBuf;
use std::process::ExitStatus;

use thiserror::Error;

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
    #[error("command `{cmd}` failed with status: {status}")]
    CommandFailed {
        /// Command that failed.
        cmd: String,
        /// Command exit status.
        status: ExitStatus,
    },

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

    /// Selected path is not a directory.
    #[error("`{path}` already exists and is not a directory")]
    PathNotDir {
        /// Selected path.
        path: PathBuf,
    },

    /// Selected path is not a symlink.
    #[error("`{path}` already exists and is not a symlink")]
    PathNotSymlink {
        /// Selected path.
        path: PathBuf,
    },

    /// Selected path is not a file.
    #[error("`{path}` is not a file")]
    PathNotFile {
        /// Selected path.
        path: PathBuf,
    },

    /// Path missing.
    #[error("`{path}` does not exist")]
    PathMissing {
        /// Selected path.
        path: PathBuf,
    },

    /// Selected path is not a git repository.
    #[error("`{path}` is not a git repository")]
    PathNotRepo {
        /// Selected path.
        path: PathBuf,
    },

    /// `nuttx` repo clone is missing in workspace.
    #[error(
        "`{name}` clone missing in `{workspace_root}`, make sure you have initialized nxus \
         workspace"
    )]
    WorkspaceRepoMissing {
        /// Workspace clone repo name.
        name: String,
        /// Nxus project-local workspace root.
        workspace_root: PathBuf,
    },

    /// project-local workspace not initialized.
    #[error("workspace at `{workspace_root}` not initialized")]
    WorkspaceNotInitialized {
        /// Workspace root dir.
        workspace_root: PathBuf,
    },

    /// Required config base for the selected board not found.
    #[error("config `{config_base}` for board `{board}` not found")]
    ConfigBaseNotFound {
        /// Board the config base belongs to.
        board: String,
        /// Config base.
        config_base: String,
    },

    /// Refusing to overwrite an existing path during initialization.
    #[error("`{path}` already exists; refusing to overwrite it")]
    PathAlreadyExists {
        /// Conflicting existing path.
        path: PathBuf,
    },

    /// Initialization target directory must be empty.
    #[error("`{path}` must be empty before initialization")]
    DirectoryNotEmpty {
        /// Non-empty directory path.
        path: PathBuf,
    },

    /// Selected profile has no flash command configured.
    #[error("profile `{profile}` does not define a flash command")]
    FlashNotConfigured {
        /// Profile name.
        profile: String,
    },

    /// Encountered an unsupported template placeholder.
    #[error("unknown flash placeholder `{{{placeholder}}}`")]
    UnknownPlaceholder {
        /// Placeholder name without braces.
        placeholder: String,
    },

    /// A flash template referenced a required artifact that is missing.
    #[error("required flash artifact `{artifact}` not found at `{path}`")]
    FlashArtifactMissing {
        /// Artifact placeholder name.
        artifact: String,
        /// Resolved artifact path.
        path: PathBuf,
    },

    /// Zappy-core error.
    #[error(transparent)]
    ZappyCore(#[from] zappy_core::CoreError),

    /// Zappy-fs error.
    #[error("{error}")]
    ZappyFs {
        /// Zappy fs error string.
        error: String,
    },
}

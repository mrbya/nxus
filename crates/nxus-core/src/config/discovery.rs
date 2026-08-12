use std::path::{Path, PathBuf};

use crate::{CoreError, CoreResult};

/// Configuration discovery context.
#[derive(Debug, Clone)]
pub struct ConfigContext {
    /// Resolved project root containing nxus.toml.
    pub project_dir: PathBuf,

    /// Current working directory @ invocation time.
    pub cwd: PathBuf,
}

/**
 * Discovers project configuration context.
 *
 * # Errors
 * Returns [`CoreError::Io`] for filesystem failures and
 * [`CoreError::ConfigNotFound`] when nxus.toml cannot be located.
 */
pub fn discover_config(project_override: Option<&Path>) -> CoreResult<ConfigContext> {
    let cwd = std::env::current_dir()?;
    let start = project_override.unwrap_or(&cwd);

    let project_dir =
        find_upwards(start, "nxus.toml").ok_or_else(|| CoreError::ConfigNotFound {
            start: start.to_path_buf(),
        })?;

    Ok(ConfigContext { project_dir, cwd })
}

/**
 * Walks upward from `start` looking for a marker file, returning the directory
 * that contains it.
 *
 * # Arguments
 * - `start`: Directory to begin the upward search from.
 * - `marker`: Marker filename to locate.
 *
 * # Returns
 * Directory containing the marker file, or `None` if not found.
 */
#[must_use]
fn find_upwards(start: &Path, marker: &str) -> Option<PathBuf> {
    let mut cur = start;
    loop {
        if cur.join(marker).is_file() {
            return Some(cur.to_path_buf());
        }
        cur = cur.parent()?;
    }
}

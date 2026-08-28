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

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::config::discovery::find_upwards;
    use crate::discover_config;

    #[test]
    fn find_upwards_finds_file() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let dir_root = temp_dir.path().join("dir1");
        let dir1 = dir_root.join("dir11");
        let dir2 = dir_root.join("dir12");
        let file1 = dir_root.join("file.marker");
        let file2 = dir2.join("file.marker");

        fs::create_dir_all(&dir1).expect("dir11 should be created");
        fs::create_dir_all(&dir2).expect("dir12 should be created");
        fs::write(&file1, "marker").expect("file1 should be created");
        fs::write(&file2, "marker").expect("file1 should be created");

        let found = find_upwards(&dir1, "file.marker");
        assert_eq!(found, Some(dir_root));
    }

    #[test]
    fn find_upwards_returns_none_on_missing_file() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        assert!(find_upwards(temp_dir.path(), "file.marker").is_none());
    }

    #[test]
    fn discover_config_finds_config() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let dir = temp_dir.path().join("dir1");
        let config_file = temp_dir.path().join("nxus.toml");

        fs::create_dir_all(&dir).expect("dir should be created");
        fs::write(&config_file, "config").expect("config file should be created");

        assert_eq!(
            discover_config(Some(temp_dir.path()))
                .expect("should find config")
                .project_dir,
            temp_dir.path()
        );

        assert_eq!(
            discover_config(Some(&dir))
                .expect("should find config")
                .project_dir,
            temp_dir.path()
        );
    }
}

#[test]
fn discover_config_errs_when_no_config_found() {
    let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
    discover_config(Some(temp_dir.path())).expect_err("config discovery should err");
}

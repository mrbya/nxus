use std::fs;
use std::path::Path;

use crate::config::NxusConfig;
use crate::{CoreError, CoreResult};

/// Loads `nxus.toml` configuration for a project.
///
/// # Errors
/// Returns [`CoreError::Io`] on I/O operation failures and
/// [`CoreError::ParseConfig`] if fails to parse toml config.
pub fn load_config(project_dir: &Path) -> CoreResult<NxusConfig> {
    let path = project_dir.join("nxus.toml");
    let config_str = fs::read_to_string(&path).map_err(CoreError::Io)?;
    let parsed = toml::from_str::<NxusConfig>(&config_str)
        .map_err(|source| CoreError::ParseConfig { path, source })?;

    let config = NxusConfig::new().overlay(parsed);

    Ok(config)
}

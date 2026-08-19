/// Config discovery.
pub mod discovery;
/// Config file load.
pub mod load;
/// Config resolution.
pub mod resolution;
/// Config file toml schema.
pub mod schema;

// Re-exports.
pub use discovery::{discover_config, ConfigContext};
pub use load::load_config;
pub use resolution::ResolvedConfig;
pub use schema::{
    NxusConfig, ProfileConfig, DEFAULT_BUILD_ROOT, DEFAULT_NUTTX_APPS_SRC, DEFAULT_NUTTX_SRC,
    DEFAULT_OVERLAY_ROOT, DEFAULT_PROJECT_DEFAULT_PROFILE, DEFAULT_WORKSPACE_ROOT,
};

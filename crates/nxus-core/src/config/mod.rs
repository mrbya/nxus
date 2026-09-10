/// Config discovery.
pub mod discovery;
/// Config file load.
pub mod load;
/// Config resolution.
pub mod resolution;
/// Config file toml schema.
pub mod schema;

// Re-exports.
pub use discovery::{ConfigContext, discover_config};
pub use load::load_config;
pub use resolution::{ProfileSelection, ResolvedConfig};
pub use schema::{
    CommandConfig, DEFAULT_BUILD_ROOT, DEFAULT_NUTTX_APPS_REV, DEFAULT_NUTTX_APPS_SRC,
    DEFAULT_NUTTX_REV, DEFAULT_NUTTX_SRC, DEFAULT_OVERLAY_ROOT, DEFAULT_PROJECT_DEFAULT_PROFILE,
    DEFAULT_SIM_ARCH, DEFAULT_SIM_BOARD, DEFAULT_SIM_CONFIG_BASE, DEFAULT_SIM_FAMILY,
    DEFAULT_TEST_ARCH, DEFAULT_TEST_BOARD, DEFAULT_TEST_CONFIG_BASE, DEFAULT_TEST_FAMILY,
    DEFAULT_WORKSPACE_ROOT, NxusConfig, ProfileConfig, SIM_PROFILE_NAME, TEST_PROFILE_NAME,
};

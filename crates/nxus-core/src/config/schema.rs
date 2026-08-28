use indexmap::IndexMap;
use serde::Deserialize;

/// Nxus top-lefel configuration file schema.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct NxusConfig {
    /// Project configuration settings.
    #[serde(default)]
    pub project: ProjectConfig,

    /// Build dir configuration settings.
    #[serde(default)]
    pub build: BuildConfig,

    /// Project-local `NuttX` workspace configuration settings.
    #[serde(default)]
    pub workspace: WorkspaceConfig,

    /// Build profile configs.
    #[serde(default, alias = "profile")]
    pub profiles: IndexMap<String, ProfileConfig>,
}

/// Default project config `default_rpofile` value.
pub const DEFAULT_PROJECT_DEFAULT_PROFILE: &str = "sim";
/// Default project `overlay_root` value.
pub const DEFAULT_OVERLAY_ROOT: &str = "config";
/// Default build `root` value.
pub const DEFAULT_BUILD_ROOT: &str = "build";
/// Default workspace `root` value.
pub const DEFAULT_WORKSPACE_ROOT: &str = "workspace";
/// Sim profile name.
const SIM_PROFILE_NAME: &str = "sim";
/// Test profile name.
const TEST_PROFILE_NAME: &str = "test";
/// Default nuttx clone source repository.
pub const DEFAULT_NUTTX_SRC: &str = "https://github.com/apache/nuttx.git";
/// Default nuttx apps clone source repository.
pub const DEFAULT_NUTTX_APPS_SRC: &str = "https://github.com/apache/nuttx-apps.git";

impl NxusConfig {
    /// Constructs new default nxus config.
    #[must_use]
    pub fn new() -> Self {
        let mut config = Self::default();

        config.project.default_profile = Some(String::from(DEFAULT_PROJECT_DEFAULT_PROFILE));
        config.project.overlay_root = Some(String::from(DEFAULT_OVERLAY_ROOT));
        config.build.root = Some(String::from(DEFAULT_BUILD_ROOT));
        config.build.link_compile_commands = Some(true);
        config.workspace.root = Some(String::from(DEFAULT_WORKSPACE_ROOT));
        config.workspace.nuttx = WorkspaceComponentConfig::new_nuttx();
        config.workspace.nuttx_apps = WorkspaceComponentConfig::new_nuttx_apps();

        config
            .profiles
            .insert(String::from(SIM_PROFILE_NAME), ProfileConfig::new_sim());

        config
            .profiles
            .insert(String::from(TEST_PROFILE_NAME), ProfileConfig::new_test());

        config
    }

    /// Overlays another nxus configuration on top of this one.
    ///
    /// # Returns
    /// Merged nxus configuration with RSH values taking precedence.
    #[must_use]
    pub fn overlay(mut self, rhs: Self) -> Self {
        self.project = self.project.overlay(rhs.project);
        self.build = self.build.overlay(rhs.build);
        self.workspace = self.workspace.overlay(rhs.workspace);

        for (name, prof_rhs) in rhs.profiles {
            match self.profiles.get_mut(&name) {
                Some(prof_lhs) => {
                    *prof_lhs = prof_lhs.clone().overlay(prof_rhs);
                }

                None => {
                    self.profiles.insert(name, prof_rhs);
                }
            }
        }

        self
    }
}

/// Project-level configuration values.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ProjectConfig {
    /// Default profile name.
    pub default_profile: Option<String>,

    /// Config overlay root.
    pub overlay_root: Option<String>,
}

impl ProjectConfig {
    /// Overlays another project config on top of this one.
    ///
    /// # Returns
    /// Merged project configuration with RHS values taking precedence.
    #[must_use]
    pub fn overlay(mut self, rhs: Self) -> Self {
        if rhs.default_profile.is_some() {
            self.default_profile = rhs.default_profile;
        }

        if rhs.overlay_root.is_some() {
            self.overlay_root = rhs.overlay_root;
        }

        self
    }
}

/// Build dir config values.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct BuildConfig {
    /// Build output root directory relative to project root.
    pub root: Option<String>,

    /// Whether to ling `compile_commands.json`.
    pub link_compile_commands: Option<bool>,
}

impl BuildConfig {
    /// Overlays another build dir config on top of this one.
    ///
    /// # Return
    /// Merged build dir config with RHS values taking precedence.
    pub fn overlay(mut self, rhs: Self) -> Self {
        if rhs.root.is_some() {
            self.root = rhs.root;
        }

        if rhs.link_compile_commands.is_some() {
            self.link_compile_commands = rhs.link_compile_commands;
        }

        self
    }
}

/// Project-local `NuttX` workspace config values.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct WorkspaceConfig {
    /// Project-local `NuttX` workspace root relative to project root.
    pub root: Option<String>,

    /// Project-local workspace `NuttX` clone configuration.
    pub nuttx: WorkspaceComponentConfig,

    /// Project-local workspace `nuttx_apps` clone configuration.
    pub nuttx_apps: WorkspaceComponentConfig,
}

impl WorkspaceConfig {
    /// Overlays another workspace config on top of this one.
    ///
    /// # Return
    /// Merged workspace config with RHS values taking precedence.
    pub fn overlay(mut self, rhs: Self) -> Self {
        if rhs.root.is_some() {
            self.root = rhs.root;
        }

        if rhs.nuttx.src.is_some() {
            self.nuttx.src = rhs.nuttx.src;
        }
        if rhs.nuttx.rev.is_some() {
            self.nuttx.rev = rhs.nuttx.rev;
        }

        if rhs.nuttx_apps.src.is_some() {
            self.nuttx_apps.src = rhs.nuttx_apps.src;
        }
        if rhs.nuttx_apps.rev.is_some() {
            self.nuttx_apps.rev = rhs.nuttx_apps.rev;
        }

        self
    }
}

/// `NuttX` workspace component config.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct WorkspaceComponentConfig {
    /// Source repository.
    pub src: Option<String>,

    /// Component revision to check out.
    pub rev: Option<String>,
}

impl WorkspaceComponentConfig {
    /// Constructs default workspace nuttx clone config.
    pub fn new_nuttx() -> Self {
        Self {
            src: Some(String::from(DEFAULT_NUTTX_SRC)),
            rev: None,
        }
    }

    /// Constructs default workspace nuttx apps clone config.
    pub fn new_nuttx_apps() -> Self {
        Self {
            src: Some(String::from(DEFAULT_NUTTX_APPS_SRC)),
            rev: None,
        }
    }

    /// Overlays another workspace component config on top of this one.
    ///
    /// # Return
    /// Merged workspace component config with RHS values taking precedence.
    pub fn overlay(mut self, rhs: Self) -> Self {
        if rhs.src.is_some() {
            self.src = rhs.src;
        }

        if rhs.rev.is_some() {
            self.rev = rhs.rev;
        }

        self
    }
}

/// Profile config values.
#[derive(Debug, Clone, Deserialize)]
pub struct ProfileConfig {
    /// Profile target architecture.
    pub arch: String,

    /// Profile target family.
    pub family: String,

    /// Profile target board.
    pub board: String,

    /// Profile target config base.
    pub config_base: String,
}

/// Default sim profile architecture.
const DEFAULT_SIM_ARCH: &str = "sim";
/// Default sim profile family.
const DEFAULT_SIM_FAMILY: &str = "sim";
/// Default sim profile board.
const DEFAULT_SIM_BOARD: &str = "sim";
/// Default sim profile config base.
const DEFAULT_SIM_CONFIG_BASE: &str = "nsh";

/// Default test profile architecture.
const DEFAULT_TEST_ARCH: &str = "sim";
/// Default test profile family.
const DEFAULT_TEST_FAMILY: &str = "sim";
/// Default test profile board.
const DEFAULT_TEST_BOARD: &str = "sim";
/// Default test profile config base.
const DEFAULT_TEST_CONFIG_BASE: &str = "nsh";

impl ProfileConfig {
    /// Constructs default sim profile config.
    #[must_use]
    pub fn new_sim() -> Self {
        Self {
            arch: DEFAULT_SIM_ARCH.into(),
            family: DEFAULT_SIM_FAMILY.into(),
            board: DEFAULT_SIM_BOARD.into(),
            config_base: DEFAULT_SIM_CONFIG_BASE.into(),
        }
    }

    /// Constructs default test profile config.
    #[must_use]
    pub fn new_test() -> Self {
        Self {
            arch: DEFAULT_TEST_ARCH.into(),
            family: DEFAULT_TEST_FAMILY.into(),
            board: DEFAULT_TEST_BOARD.into(),
            config_base: DEFAULT_TEST_CONFIG_BASE.into(),
        }
    }

    /// Overlays another profile config on top of this one.
    ///
    /// # Return
    /// Merged profile config with RHS values taking precedence.
    #[must_use]
    pub fn overlay(mut self, rhs: Self) -> Self {
        self = rhs;
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::config::schema::{
        BuildConfig, ProjectConfig, WorkspaceComponentConfig, WorkspaceConfig,
    };
    use crate::config::{
        DEFAULT_BUILD_ROOT, DEFAULT_NUTTX_SRC, DEFAULT_PROJECT_DEFAULT_PROFILE,
        DEFAULT_WORKSPACE_ROOT,
    };
    use crate::{NxusConfig, ProfileConfig};

    #[test]
    fn default_config_contains_default_values() {
        let cfg = NxusConfig::new();
        assert_eq!(
            cfg.project.default_profile,
            Some(String::from(DEFAULT_PROJECT_DEFAULT_PROFILE))
        );
        assert_eq!(cfg.build.root, Some(String::from(DEFAULT_BUILD_ROOT)));
        assert_eq!(
            cfg.workspace.root,
            Some(String::from(DEFAULT_WORKSPACE_ROOT))
        );
        assert_eq!(
            cfg.workspace.nuttx.src,
            Some(String::from(DEFAULT_NUTTX_SRC))
        );

        assert_eq!(
            cfg.profiles
                .get("sim")
                .expect("cfg should contain sim profile")
                .board,
            String::from("sim")
        );

        assert_eq!(
            cfg.profiles
                .get("test")
                .expect("cfg should contain sim profile")
                .board,
            String::from("sim")
        );
    }

    #[test]
    fn nxus_cfg_overlay_prefers_rhs() {
        let lhs = NxusConfig::new();
        let mut rhs = NxusConfig {
            project: ProjectConfig {
                default_profile: Some(String::from("right")),
                overlay_root: None,
            },
            build: BuildConfig {
                root: Some(String::from("right")),
                link_compile_commands: Some(false),
            },
            workspace: WorkspaceConfig {
                root: Some(String::from("right")),
                nuttx: WorkspaceComponentConfig {
                    src: Some(String::from("right")),
                    rev: Some(String::from("v0.0.0")),
                },
                nuttx_apps: WorkspaceComponentConfig {
                    src: None,
                    rev: None,
                },
            },
            ..Default::default()
        };

        rhs.profiles.insert(
            String::from("sim"),
            ProfileConfig {
                arch: String::from("arch"),
                family: String::from("family"),
                board: String::from("board"),
                config_base: String::from("base"),
            },
        );

        let merged = lhs.overlay(rhs);
        assert_eq!(merged.project.default_profile, Some(String::from("right")));
        assert_eq!(merged.build.root, Some(String::from("right")));
        assert_eq!(merged.workspace.nuttx.src, Some(String::from("right")));
        assert_eq!(merged.workspace.nuttx.rev, Some(String::from("v0.0.0")));

        let merged_sim = merged
            .profiles
            .get("sim")
            .expect("should contain sim profile");
        assert_eq!(merged_sim.arch, String::from("arch"));
        assert_eq!(merged_sim.family, String::from("family"));
        assert_eq!(merged_sim.board, String::from("board"));
        assert_eq!(merged_sim.config_base, String::from("base"));
    }
}

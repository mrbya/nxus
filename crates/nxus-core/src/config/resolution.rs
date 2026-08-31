use std::path::PathBuf;

use indexmap::IndexMap;

use crate::config::{
    ConfigContext, NxusConfig, DEFAULT_BUILD_ROOT, DEFAULT_NUTTX_APPS_SRC, DEFAULT_NUTTX_SRC,
    DEFAULT_OVERLAY_ROOT, DEFAULT_PROJECT_DEFAULT_PROFILE, DEFAULT_WORKSPACE_ROOT,
};
use crate::{CommandConfig, CoreError, CoreResult, ProfileConfig, Runner};

/// Resolved nxus configuration after parsing and resolving profile.
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    /// General config values
    /// Current working dir.
    pub cwd: PathBuf,
    /// Pre-celan profile build dir?
    pub clean: bool,
    /// Rebuild project for selected profile?
    pub rebuild: bool,
    /// Runner with verbosity and dry-run config.
    pub runner: Runner,
    /// Config discovery context.
    pub ctx: ConfigContext,
    /// Profile selected?
    pub profile_selected: bool,
    /// Selected profile name, if any.
    pub profile: String,
    /// Profiles.
    pub profiles: IndexMap<String, ProfileConfig>,

    /// Build dir related config values.
    /// Project build root path.
    pub build_root: PathBuf,
    /// Build directory for the selected profile.
    pub build_dir: PathBuf,
    /// Whether to link `compile_commands.json`.
    pub link_compile_commands: bool,

    /// Project-local `NuttX` workspace related config values.
    pub workspace_root: PathBuf,
    /// Workspace nuttx clone source repository.
    pub nuttx_src: String,
    /// Workspace nuttx clone revision.
    pub nuttx_rev: Option<String>,
    /// Workspace nuttx apps source repository.
    pub nuttx_apps_src: String,
    /// Workspace nuttx apps revision.
    pub nuttx_apps_rev: Option<String>,

    /// Resolved profile config values.
    /// Profile target architecture.
    pub arch: String,
    /// Profile target family.
    pub family: String,
    /// Profile target board.
    pub board: String,
    /// Profile target config base.
    pub config_base: String,
    /// Selected profile flash command configuration.
    pub flash: Option<CommandConfig>,
    /// Config overlay available for selected profile?
    pub config_overlay: PathBuf,
}

impl ResolvedConfig {
    /// Resolves configuration for a selected profile.
    ///
    /// # Errors
    /// Returns [`CoreError::UnknownProfile`] when trying to resolve config for an unknown profile.
    pub fn resolve(
        clean: bool,
        rebuild: bool,
        verbose: u8,
        dry_run: bool,
        ctx: &ConfigContext,
        profile: Option<&String>,
        cfg: &NxusConfig,
    ) -> CoreResult<Self> {
        let selected = select_profile(profile.cloned(), cfg)?;

        let build_root = ctx.project_dir.join(
            cfg.build
                .root
                .clone()
                .unwrap_or_else(|| String::from(DEFAULT_BUILD_ROOT)),
        );
        let build_dir = build_root.join(&selected);
        let link_compile_commands = cfg.build.link_compile_commands.unwrap_or(true);

        let workspace_root = ctx.project_dir.join(
            cfg.workspace
                .root
                .clone()
                .unwrap_or_else(|| String::from(DEFAULT_WORKSPACE_ROOT)),
        );

        let nuttx_src = cfg
            .workspace
            .nuttx
            .src
            .clone()
            .unwrap_or_else(|| String::from(DEFAULT_NUTTX_SRC));
        let nuttx_rev = cfg.workspace.nuttx.rev.clone();

        let nuttx_apps_src = cfg
            .workspace
            .nuttx_apps
            .src
            .clone()
            .unwrap_or_else(|| String::from(DEFAULT_NUTTX_APPS_SRC));
        let nuttx_apps_rev = cfg.workspace.nuttx_apps.rev.clone();

        let Some(profile_cfg) = cfg.profiles.get(&selected) else {
            return Err(CoreError::UnknownProfile { profile: selected });
        };

        let arch = profile_cfg.arch.clone();
        let family = profile_cfg.family.clone();
        let board = profile_cfg.board.clone();
        let config_base = profile_cfg.config_base.clone();
        let flash = profile_cfg.flash.clone();
        let config_overlay = ctx
            .project_dir
            .join(
                cfg.project
                    .overlay_root
                    .clone()
                    .unwrap_or_else(|| String::from(DEFAULT_OVERLAY_ROOT)),
            )
            .join(format!("{selected}.overlay"));

        Ok(Self {
            cwd: ctx.cwd.clone(),
            clean,
            rebuild,
            runner: Runner { verbose, dry_run },
            ctx: ctx.clone(),
            profile_selected: profile.is_some(),
            profile: selected,
            profiles: cfg.profiles.clone(),
            build_root,
            build_dir,
            link_compile_commands,
            workspace_root,
            nuttx_src,
            nuttx_rev,
            nuttx_apps_src,
            nuttx_apps_rev,
            arch,
            family,
            board,
            config_base,
            flash,
            config_overlay,
        })
    }

    /// Creates new resilved config and overwrites its selected profile.
    #[must_use]
    pub fn with_profile(&self, profile: &str) -> Self {
        let mut config = self.clone();
        config.profile_selected = true;
        config.profile = String::from(profile);
        config
    }
}

/// Selects the active profile based on CLI flags and available profiles.
///
/// # Errors
/// Returns [`CoreError::UnknownProfile`] when a requested profile does not exist.
fn select_profile(profile: Option<String>, cfg: &NxusConfig) -> CoreResult<String> {
    let selected = profile.unwrap_or_else(|| {
        cfg.project
            .default_profile
            .clone()
            .unwrap_or_else(|| String::from(DEFAULT_PROJECT_DEFAULT_PROFILE))
    });

    if !cfg.profiles.contains_key(&selected) {
        return Err(CoreError::UnknownProfile { profile: selected });
    }

    Ok(selected)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::config::{ConfigContext, NxusConfig, ResolvedConfig};
    use crate::{CoreError, ProfileConfig};

    fn context() -> ConfigContext {
        ConfigContext {
            project_dir: PathBuf::from("/tmp/project"),
            cwd: PathBuf::from("/tmp/project/app"),
        }
    }

    #[test]
    fn resolve_uses_selected_profile_and_overrides() {
        let mut cfg = NxusConfig::new();
        cfg.project.default_profile = Some(String::from("prod"));
        cfg.project.overlay_root = Some(String::from("overlays"));
        cfg.build.root = Some(String::from("out"));
        cfg.build.link_compile_commands = Some(false);
        cfg.workspace.root = Some(String::from("ws"));
        cfg.workspace.nuttx.src = Some(String::from("nuttx-src"));
        cfg.workspace.nuttx.rev = Some(String::from("nuttx-rev"));
        cfg.workspace.nuttx_apps.src = Some(String::from("apps-src"));
        cfg.workspace.nuttx_apps.rev = Some(String::from("apps-rev"));
        cfg.profiles.insert(
            String::from("prod"),
            ProfileConfig {
                arch: String::from("arm"),
                family: String::from("stm32"),
                board: String::from("nucleo"),
                config_base: String::from("release"),
                flash: None,
            },
        );

        let profile = String::from("prod");
        let resolved = ResolvedConfig::resolve(true, 4, true, &context(), Some(&profile), &cfg)
            .expect("config should resolve");

        assert!(resolved.clean);
        assert_eq!(resolved.runner.verbose, 4);
        assert!(resolved.runner.dry_run);
        assert!(resolved.profile_selected);
        assert_eq!(resolved.profile, profile);
        assert_eq!(resolved.build_root, PathBuf::from("/tmp/project/out"));
        assert_eq!(resolved.build_dir, PathBuf::from("/tmp/project/out/prod"));
        assert!(!resolved.link_compile_commands);
        assert_eq!(resolved.workspace_root, PathBuf::from("/tmp/project/ws"));
        assert_eq!(resolved.nuttx_src, String::from("nuttx-src"));
        assert_eq!(resolved.nuttx_rev, Some(String::from("nuttx-rev")));
        assert_eq!(resolved.nuttx_apps_src, String::from("apps-src"));
        assert_eq!(resolved.nuttx_apps_rev, Some(String::from("apps-rev")));
        assert_eq!(resolved.arch, String::from("arm"));
        assert_eq!(resolved.family, String::from("stm32"));
        assert_eq!(resolved.board, String::from("nucleo"));
        assert_eq!(resolved.config_base, String::from("release"));
        assert_eq!(resolved.flash, None);
        assert_eq!(
            resolved.config_overlay,
            PathBuf::from("/tmp/project/overlays/prod.overlay")
        );
    }

    #[test]
    fn resolve_uses_default_profile_when_not_selected() {
        let cfg = NxusConfig::new();

        let resolved = ResolvedConfig::resolve(false, 2, false, &context(), None, &cfg)
            .expect("default config should resolve");

        assert!(!resolved.profile_selected);
        assert_eq!(resolved.profile, String::from("sim"));
        assert_eq!(resolved.build_dir, PathBuf::from("/tmp/project/build/sim"));
    }

    #[test]
    fn resolve_errors_for_unknown_profile() {
        let cfg = NxusConfig::new();
        let requested_profile = String::from("missing");

        let error =
            ResolvedConfig::resolve(false, 0, false, &context(), Some(&requested_profile), &cfg)
                .expect_err("unknown profile should fail");

        assert!(matches!(
            error,
            CoreError::UnknownProfile { profile } if profile == "missing"
        ));
    }

    #[test]
    fn with_profile_marks_profile_as_selected() {
        let resolved =
            ResolvedConfig::resolve(false, 1, true, &context(), None, &NxusConfig::new())
                .expect("default config should resolve");

        let selected = resolved.with_profile("test");

        assert!(selected.profile_selected);
        assert_eq!(selected.profile, String::from("test"));
        assert_eq!(selected.build_dir, resolved.build_dir);
        assert_eq!(selected.workspace_root, resolved.workspace_root);
    }
}

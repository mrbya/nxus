use std::path::PathBuf;

use indexmap::IndexMap;

use crate::config::{
    ConfigContext, NxusConfig, DEFAULT_BUILD_ROOT, DEFAULT_NUTTX_APPS_SRC, DEFAULT_NUTTX_SRC,
    DEFAULT_OVERLAY_ROOT, DEFAULT_PROJECT_DEFAULT_PROFILE, DEFAULT_WORKSPACE_ROOT,
};
use crate::{CoreError, CoreResult, ProfileConfig, Runner};

/// Resolved nxus configuration after parsing and resolving profile.
#[derive(Debug, Clone)]
pub struct ResolvedConfig {
    /// General config values
    /// Current working dir.
    pub cwd: PathBuf,
    /// Pre-celan profile build dir?
    pub clean: bool,
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
            cwd: std::env::current_dir()?,
            clean,
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
            config_overlay,
        })
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

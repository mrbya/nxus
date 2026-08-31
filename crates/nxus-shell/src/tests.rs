use std::path::{Path, PathBuf};

use indexmap::IndexMap;
use nxus_core::{
    CommandConfig, ConfigContext, ProfileConfig, ProfileSelection, ResolvedConfig, Runner,
};

pub fn profile_config(arch: &str, family: &str, board: &str, config_base: &str) -> ProfileConfig {
    ProfileConfig {
        arch: String::from(arch),
        family: String::from(family),
        board: String::from(board),
        config_base: String::from(config_base),
        flash: None,
    }
}

pub fn flash_command(command: &str, args: &[&str]) -> CommandConfig {
    CommandConfig {
        command: String::from(command),
        args: args.iter().map(|arg| String::from(*arg)).collect(),
    }
}

pub fn resolved_config(project_dir: &Path) -> ResolvedConfig {
    let cwd = project_dir.join("app");
    let build_root = project_dir.join("build-root");
    let workspace_root = project_dir.join("workspace-root");
    let profile_name = String::from("sim");
    let profile = profile_config("arch", "family", "board", "base");
    let mut profiles = IndexMap::new();
    profiles.insert(profile_name.clone(), profile.clone());
    profiles.insert(
        String::from("test"),
        profile_config("sim", "sim", "sim", "nsh"),
    );

    ResolvedConfig {
        cwd,
        clean: false,
        rebuild: false,
        runner: Runner {
            verbose: 1,
            dry_run: true,
        },
        ctx: ConfigContext {
            project_dir: PathBuf::from(project_dir),
            cwd: PathBuf::from(project_dir),
        },
        profile_selection: ProfileSelection::Default,
        profile: profile_name,
        profiles,
        build_root: build_root.clone(),
        build_dir: build_root.join("sim"),
        link_compile_commands: true,
        workspace_root,
        nuttx_src: String::from("https://example.invalid/nuttx.git"),
        nuttx_rev: None,
        nuttx_apps_src: String::from("https://example.invalid/nuttx-apps.git"),
        nuttx_apps_rev: None,
        arch: profile.arch,
        family: profile.family,
        board: profile.board,
        config_base: profile.config_base,
        flash: None,
        config_overlay: project_dir.join("config").join("sim.overlay"),
    }
}

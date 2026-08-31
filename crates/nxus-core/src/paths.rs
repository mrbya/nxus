use std::path::PathBuf;

use crate::{ProfileConfig, ResolvedConfig};

/// Returns path to project-local workspace nuttx repo clone.
#[must_use]
pub fn nuttx(cfg: &ResolvedConfig) -> PathBuf {
    cfg.workspace_root.join("nuttx")
}

/// Returns path to project-local workspace nuttx apps repo clone.
#[must_use]
pub fn nuttx_apps(cfg: &ResolvedConfig) -> PathBuf {
    cfg.workspace_root.join("nuttx-apps")
}

/// Returns path to project-local workspace nuttx app project link.
#[must_use]
pub fn app_link(cfg: &ResolvedConfig) -> PathBuf {
    nuttx_apps(cfg).join("external")
}

/// Returns path to build root dir.
#[must_use]
pub fn build_root(cfg: &ResolvedConfig) -> PathBuf {
    cfg.cwd.join(&cfg.build_root)
}

/// Returns path to build dir of the selected profile.
#[must_use]
pub fn build_dir(cfg: &ResolvedConfig, profile: &str) -> PathBuf {
    build_root(cfg).join(profile)
}

/// Returns path to the standard `NuttX` executable output.
#[must_use]
pub fn firmware_elf(cfg: &ResolvedConfig, profile: &str) -> PathBuf {
    build_dir(cfg, profile).join("nuttx")
}

/// Returns path to the standard `NuttX` binary output.
#[must_use]
pub fn firmware_bin(cfg: &ResolvedConfig, profile: &str) -> PathBuf {
    build_dir(cfg, profile).join("nuttx.bin")
}

/// Returns path to the standard `NuttX` Intel HEX output.
#[must_use]
pub fn firmware_hex(cfg: &ResolvedConfig, profile: &str) -> PathBuf {
    build_dir(cfg, profile).join("nuttx.hex")
}

/// Returns path to generated configs root dir.
#[must_use]
pub fn generated_configs_root(cfg: &ResolvedConfig) -> PathBuf {
    cfg.workspace_root.join("config")
}

/// Returns path to generated config dir for selected profile.
#[must_use]
pub fn generated_config_dir(cfg: &ResolvedConfig, profile: &str) -> PathBuf {
    generated_configs_root(cfg).join(profile)
}

/// Returns path to generated defconfig file for selected profile.
#[must_use]
pub fn generated_config_file(cfg: &ResolvedConfig, profile: &str) -> PathBuf {
    generated_config_dir(cfg, profile).join("defconfig")
}

/// Returns path to nxus project config root dir.
#[must_use]
pub fn config_root(cfg: &ResolvedConfig) -> PathBuf {
    cfg.cwd.join("config")
}

/// Returns path to nxus project common config.
#[must_use]
pub fn common_config(cfg: &ResolvedConfig) -> PathBuf {
    config_root(cfg).join("common.config")
}

/// Returns path to nxus project config overlay for selected profile.
#[must_use]
pub fn config_overlay(cfg: &ResolvedConfig, profile: &str) -> PathBuf {
    config_root(cfg).join(format!("{profile}.overlay"))
}

/// Returns path to board config root dir.
#[must_use]
pub fn board_config_root(cfg: &ResolvedConfig) -> PathBuf {
    nuttx(cfg)
        .join("boards")
        .join(&cfg.arch)
        .join(&cfg.family)
        .join(&cfg.board)
        .join("configs")
}

/// Returns path to target board config base selected by profile.
#[must_use]
pub fn board_config_base(cfg: &ResolvedConfig) -> PathBuf {
    board_config_root(cfg)
        .join(&cfg.config_base)
        .join("defconfig")
}

/// Returns path to target board config dir link.
#[must_use]
pub fn board_config_link(cfg: &ResolvedConfig, profile: &str) -> PathBuf {
    board_config_root(cfg).join(profile)
}

/// Returns path to board config root dir for a specific profile config.
#[must_use]
pub fn board_config_root_for_profile(cfg: &ResolvedConfig, profile: &ProfileConfig) -> PathBuf {
    nuttx(cfg)
        .join("boards")
        .join(&profile.arch)
        .join(&profile.family)
        .join(&profile.board)
        .join("configs")
}

/// Returns path to target board config dir link for a specific profile config.
#[must_use]
pub fn board_config_link_for_profile(
    cfg: &ResolvedConfig,
    profile_name: &str,
    profile: &ProfileConfig,
) -> PathBuf {
    board_config_root_for_profile(cfg, profile).join(profile_name)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::paths;
    use crate::tests::{profile_config, resolved_config};

    #[test]
    fn path_helpers_resolve_expected_locations() {
        let cfg = resolved_config(PathBuf::from("/tmp/project").as_path());
        let prod = profile_config("rv32", "esp32", "devkit", "release");

        assert_eq!(
            paths::nuttx(&cfg),
            PathBuf::from("/tmp/project/workspace-root/nuttx")
        );
        assert_eq!(
            paths::nuttx_apps(&cfg),
            PathBuf::from("/tmp/project/workspace-root/nuttx-apps")
        );
        assert_eq!(
            paths::app_link(&cfg),
            PathBuf::from("/tmp/project/workspace-root/nuttx-apps/external")
        );
        assert_eq!(
            paths::build_root(&cfg),
            PathBuf::from("/tmp/project/build-root")
        );
        assert_eq!(
            paths::build_dir(&cfg, "sim"),
            PathBuf::from("/tmp/project/build-root/sim")
        );
        assert_eq!(
            paths::firmware_elf(&cfg, "sim"),
            PathBuf::from("/tmp/project/build-root/sim/nuttx")
        );
        assert_eq!(
            paths::firmware_bin(&cfg, "sim"),
            PathBuf::from("/tmp/project/build-root/sim/nuttx.bin")
        );
        assert_eq!(
            paths::firmware_hex(&cfg, "sim"),
            PathBuf::from("/tmp/project/build-root/sim/nuttx.hex")
        );
        assert_eq!(
            paths::generated_configs_root(&cfg),
            PathBuf::from("/tmp/project/workspace-root/config")
        );
        assert_eq!(
            paths::generated_config_dir(&cfg, "sim"),
            PathBuf::from("/tmp/project/workspace-root/config/sim")
        );
        assert_eq!(
            paths::generated_config_file(&cfg, "sim"),
            PathBuf::from("/tmp/project/workspace-root/config/sim/defconfig")
        );
        assert_eq!(
            paths::config_root(&cfg),
            PathBuf::from("/tmp/project/app/config")
        );
        assert_eq!(
            paths::common_config(&cfg),
            PathBuf::from("/tmp/project/app/config/common.config")
        );
        assert_eq!(
            paths::config_overlay(&cfg, "sim"),
            PathBuf::from("/tmp/project/app/config/sim.overlay")
        );
        assert_eq!(
            paths::board_config_root(&cfg),
            PathBuf::from("/tmp/project/workspace-root/nuttx/boards/arch/family/board/configs")
        );
        assert_eq!(
            paths::board_config_base(&cfg),
            PathBuf::from(
                "/tmp/project/workspace-root/nuttx/boards/arch/family/board/configs/base/defconfig"
            )
        );
        assert_eq!(
            paths::board_config_link(&cfg, "sim"),
            PathBuf::from("/tmp/project/workspace-root/nuttx/boards/arch/family/board/configs/sim")
        );
        assert_eq!(
            paths::board_config_root_for_profile(&cfg, &prod),
            PathBuf::from("/tmp/project/workspace-root/nuttx/boards/rv32/esp32/devkit/configs")
        );
        assert_eq!(
            paths::board_config_link_for_profile(&cfg, "prod", &prod),
            PathBuf::from(
                "/tmp/project/workspace-root/nuttx/boards/rv32/esp32/devkit/configs/prod"
            )
        );
    }
}

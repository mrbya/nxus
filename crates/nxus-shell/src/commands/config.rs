use std::fs;
use std::process::ExitCode;

use nxus_core::{
    Cmd, CoreError, ResolvedConfig, ensure_workspace, generate_config, link_app, link_config, paths,
};

/// Nxus command: config.
pub fn config(cfg: &ResolvedConfig) -> ExitCode {
    if let Err(error) = ensure_workspace(cfg) {
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }

    if let Err(error) = link_app(cfg) {
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }

    if let Err(error) = generate_config(cfg) {
        println!("gen");
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }

    if let Err(error) = link_config(cfg) {
        println!("link");
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }

    let build_dir = paths::build_dir(cfg, &cfg.profile);
    let build_dir_present = build_dir.exists();

    if build_dir_present && !build_dir.is_dir() {
        eprintln!("{}", CoreError::PathNotDir { path: build_dir });
        return ExitCode::FAILURE;
    }

    if !build_dir_present && let Err(error) = fs::create_dir_all(&build_dir) {
        eprintln!("{}", CoreError::Io(error));
        return ExitCode::FAILURE;
    }

    let cmd = Cmd::new("cmake")
        .arg("-S")
        .arg(cfg.workspace_root.join("nuttx"))
        .arg("-B")
        .arg(&build_dir)
        .arg("-GNinja")
        .arg(format!("-DBOARD_CONFIG={}:{}", cfg.board, cfg.profile))
        .arg(format!(
            "-DNUTTX_APPS_DIR={}",
            cfg.workspace_root.join("nuttx-apps").display()
        ));

    if let Err(error) = cfg
        .runner
        .run(&cmd, &format!("Configuring project for `{}`", cfg.profile))
    {
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::ExitCode;

    use nxus_core::paths;

    use crate::commands::config;
    use crate::tests::resolved_config;

    #[test]
    fn config_fails_when_build_dir_is_a_file() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        fs::create_dir_all(paths::nuttx(&cfg)).expect("nuttx dir should be created");
        fs::create_dir_all(paths::nuttx_apps(&cfg)).expect("nuttx-apps dir should be created");
        fs::create_dir_all(
            paths::board_config_base(&cfg)
                .parent()
                .expect("board parent should exist"),
        )
        .expect("board config dir should be created");
        fs::write(paths::board_config_base(&cfg), "CONFIG_BASE=y\n")
            .expect("base config should be created");
        fs::create_dir_all(cfg.build_dir.parent().expect("build parent should exist"))
            .expect("build parent should be created");
        fs::write(&cfg.build_dir, "file").expect("build path file should be created");

        assert_eq!(config(&cfg), ExitCode::FAILURE);
    }

    #[test]
    fn config_sets_up_workspace_links_and_build_dir_in_dry_run() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        fs::create_dir_all(&cfg.cwd).expect("cwd should be created");
        fs::create_dir_all(paths::nuttx(&cfg)).expect("nuttx dir should be created");
        fs::create_dir_all(paths::nuttx_apps(&cfg)).expect("nuttx-apps dir should be created");
        fs::create_dir_all(
            paths::board_config_base(&cfg)
                .parent()
                .expect("board parent should exist"),
        )
        .expect("board config dir should be created");
        fs::write(paths::board_config_base(&cfg), "CONFIG_BASE=y\n")
            .expect("base config should be created");

        assert_eq!(config(&cfg), ExitCode::SUCCESS);
        assert!(cfg.build_dir.exists());
        assert!(paths::app_link(&cfg).is_symlink());
        assert!(paths::board_config_link(&cfg, &cfg.profile).is_symlink());
        assert!(paths::generated_config_file(&cfg, &cfg.profile).is_file());
    }
}

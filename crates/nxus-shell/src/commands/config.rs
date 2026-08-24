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

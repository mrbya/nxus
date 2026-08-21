use std::process::ExitCode;

use nxus_core::{ensure_workspace, generate_config, link_app, link_config, Cmd, ResolvedConfig};

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

    let cmd = Cmd::new("cmake")
        .arg("-S")
        .arg(cfg.workspace_root.join("nuttx"))
        .arg("-B")
        .arg(&cfg.build_dir)
        .arg("-GNinja")
        .arg(format!("-DBOARD_CONFIG=\"{}:{}\"", cfg.board, cfg.profile))
        .arg(format!(
            "-DNUTTX_APPS_DIR=\"{}\"",
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

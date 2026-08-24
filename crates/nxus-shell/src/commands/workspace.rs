use std::fs;
use std::process::ExitCode;

use nxus_core::{
    Cmd, CoreError, ResolvedConfig, ensure_workspace, paths, unlink_app, unlink_config,
};

use crate::cli::{WsArgs, WsCommand};

/// Nxus command: workspace.
pub fn workspace(cfg: &ResolvedConfig, args: &WsArgs) -> ExitCode {
    match args.command {
        WsCommand::Clean => clean(cfg),
        WsCommand::Init => init(cfg),
        WsCommand::Prune => prune(cfg),
    }
}

/// Nxus subcommand: workspace - clean.
fn clean(cfg: &ResolvedConfig) -> ExitCode {
    let workspace_dir = &cfg.workspace_root;
    let workspace_present = workspace_dir.exists();

    if workspace_present && !workspace_dir.is_dir() {
        eprintln!(
            "{}",
            CoreError::PathNotDir {
                path: workspace_dir.clone()
            }
        );
        return ExitCode::FAILURE;
    }

    if workspace_present {
        if let Err(error) = fs::remove_dir_all(workspace_dir) {
            eprintln!("{error}");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}

/// Nxus subcommand: workspace - init.
fn init(cfg: &ResolvedConfig) -> ExitCode {
    let workspace_dir = &cfg.workspace_root;
    let workspace_present = workspace_dir.exists();

    if workspace_present && !workspace_dir.is_dir() {
        eprintln!(
            "{}",
            CoreError::PathNotDir {
                path: workspace_dir.clone()
            }
        );
        return ExitCode::FAILURE;
    }

    if let Err(error) = ensure_workspace(cfg) {
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

/// Nxus subcommand: workspace - prune.
fn prune(cfg: &ResolvedConfig) -> ExitCode {
    let workspace_dir = &cfg.workspace_root;
    let workspace_present = workspace_dir.exists();

    if workspace_present && !workspace_dir.is_dir() {
        eprintln!(
            "{}",
            CoreError::PathNotDir {
                path: workspace_dir.clone()
            }
        );
        return ExitCode::FAILURE;
    }

    if !workspace_present {
        eprintln!(
            "{}",
            CoreError::WorkspaceNotInitialized {
                workspace_root: workspace_dir.clone()
            }
        );
        return ExitCode::FAILURE;
    }

    let nuttx_dir = paths::nuttx(cfg);
    let nuttx_git = nuttx_dir.join(".git");
    let nuttx_present = nuttx_dir.exists();

    if nuttx_present && !nuttx_dir.is_dir() {
        eprintln!("{}", CoreError::PathNotDir { path: nuttx_dir });
        return ExitCode::FAILURE;
    }

    if nuttx_present && !nuttx_git.exists() {
        eprintln!("{}", CoreError::PathNotRepo { path: nuttx_dir });
        return ExitCode::FAILURE;
    }

    if nuttx_present && nuttx_git.exists() && !nuttx_git.is_dir() {
        eprintln!("{}", CoreError::PathNotRepo { path: nuttx_dir });
        return ExitCode::FAILURE;
    }

    let nuttx_stash = Cmd::new("git").arg("-C").arg(&nuttx_dir).arg("stash");

    if let Err(error) = cfg.runner.run(
        &nuttx_stash,
        &format!("Stashing changes in {}", nuttx_dir.display()),
    ) {
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }

    let nuttx_apps_dir = paths::nuttx_apps(cfg);
    let nuttx_apps_git = nuttx_apps_dir.join(".git");
    let nuttx_apps_present = nuttx_apps_dir.exists();

    if nuttx_apps_present && !nuttx_apps_dir.is_dir() {
        eprintln!(
            "{}",
            CoreError::PathNotDir {
                path: nuttx_apps_dir
            }
        );
        return ExitCode::FAILURE;
    }

    if nuttx_apps_present && !nuttx_apps_git.exists() {
        eprintln!(
            "{}",
            CoreError::PathNotRepo {
                path: nuttx_apps_dir
            }
        );
        return ExitCode::FAILURE;
    }

    if nuttx_apps_present && nuttx_apps_git.exists() && !nuttx_apps_git.is_dir() {
        eprintln!(
            "{}",
            CoreError::PathNotRepo {
                path: nuttx_apps_dir
            }
        );
        return ExitCode::FAILURE;
    }

    let nuttx_apps_stash = Cmd::new("git").arg("-C").arg(&nuttx_apps_dir).arg("stash");

    if let Err(error) = cfg.runner.run(
        &nuttx_apps_stash,
        &format!("Stashing changes in {}", nuttx_apps_dir.display()),
    ) {
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }

    if let Err(error) = unlink_app(cfg) {
        eprintln!("{error}");
        return ExitCode::FAILURE;
    }

    let mut err = false;

    for (profile_name, profile) in &cfg.profiles {
        if let Err(error) = unlink_config(cfg, profile_name, Some(profile)) {
            eprintln!("{error}");
            err = true;
        }
    }

    if err {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

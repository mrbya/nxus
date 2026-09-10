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

#[cfg(test)]
mod tests {
    use std::fs;
    use std::os::unix::fs as unix_fs;
    use std::process::ExitCode;

    use nxus_core::paths;

    use crate::cli::{WsArgs, WsCommand};
    use crate::commands::workspace;
    use crate::tests::resolved_config;

    fn ws_args(command: WsCommand) -> WsArgs {
        WsArgs { command }
    }

    #[test]
    fn workspace_clean_removes_workspace_directory() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        fs::create_dir_all(&cfg.workspace_root).expect("workspace should be created");

        assert_eq!(
            workspace(&cfg, &ws_args(WsCommand::Clean)),
            ExitCode::SUCCESS
        );
        assert!(!cfg.workspace_root.exists());
    }

    #[test]
    fn workspace_clean_fails_when_workspace_path_is_file() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        fs::write(&cfg.workspace_root, "file").expect("workspace file should be created");

        assert_eq!(
            workspace(&cfg, &ws_args(WsCommand::Clean)),
            ExitCode::FAILURE
        );
    }

    #[test]
    fn workspace_init_succeeds_with_existing_repo_directories() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        fs::create_dir_all(&cfg.workspace_root).expect("workspace should be created");
        fs::create_dir_all(paths::nuttx(&cfg)).expect("nuttx dir should be created");
        fs::create_dir_all(paths::nuttx_apps(&cfg)).expect("nuttx-apps dir should be created");

        assert_eq!(
            workspace(&cfg, &ws_args(WsCommand::Init)),
            ExitCode::SUCCESS
        );
    }

    #[test]
    fn workspace_prune_fails_when_workspace_is_missing() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        assert_eq!(
            workspace(&cfg, &ws_args(WsCommand::Prune)),
            ExitCode::FAILURE
        );
    }

    #[test]
    fn workspace_prune_fails_when_nuttx_is_not_a_repository() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        fs::create_dir_all(paths::nuttx(&cfg)).expect("nuttx dir should be created");
        fs::create_dir_all(paths::nuttx_apps(&cfg)).expect("nuttx-apps dir should be created");
        fs::create_dir_all(&cfg.workspace_root).expect("workspace should be created");

        assert_eq!(
            workspace(&cfg, &ws_args(WsCommand::Prune)),
            ExitCode::FAILURE
        );
    }

    #[test]
    fn workspace_prune_stashes_and_unlinks_in_dry_run() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());
        let app_target = temp_dir.path().join("linked-app");

        fs::create_dir_all(&cfg.cwd).expect("cwd should be created");
        fs::create_dir_all(&cfg.workspace_root).expect("workspace should be created");
        fs::create_dir_all(paths::nuttx(&cfg).join(".git")).expect("nuttx git dir should exist");
        fs::create_dir_all(paths::nuttx_apps(&cfg).join(".git"))
            .expect("nuttx-apps git dir should exist");
        fs::create_dir_all(&app_target).expect("app target should exist");
        fs::create_dir_all(
            paths::app_link(&cfg)
                .parent()
                .expect("app link parent should exist"),
        )
        .expect("app link parent should be created");
        unix_fs::symlink(&app_target, paths::app_link(&cfg)).expect("app link should be created");

        for (profile_name, profile) in &cfg.profiles {
            let link_path = paths::board_config_link_for_profile(&cfg, profile_name, profile);
            let target = temp_dir.path().join(format!("{profile_name}-generated"));
            fs::create_dir_all(&target).expect("profile target should exist");
            fs::create_dir_all(link_path.parent().expect("link parent should exist"))
                .expect("link parent should be created");
            unix_fs::symlink(&target, &link_path).expect("profile link should be created");
        }

        assert_eq!(
            workspace(&cfg, &ws_args(WsCommand::Prune)),
            ExitCode::SUCCESS
        );
        assert!(!paths::app_link(&cfg).exists());
        for (profile_name, profile) in &cfg.profiles {
            assert!(!paths::board_config_link_for_profile(&cfg, profile_name, profile).exists());
        }
    }
}

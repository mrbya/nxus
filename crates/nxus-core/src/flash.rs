use crate::{Cmd, CoreError, CoreResult, ResolvedConfig, resolve_command};

/// Resolves the selected profile's configured flash command into an executable command.
///
/// # Errors
/// Returns [`CoreError`] when flash is not configured, a placeholder is
/// unknown, or a required artifact is missing.
pub fn resolve_flash_command(cfg: &ResolvedConfig) -> CoreResult<Cmd> {
    let Some(flash) = cfg.flash.as_ref() else {
        return Err(CoreError::FlashNotConfigured {
            profile: cfg.profile.clone(),
        });
    };

    resolve_command(cfg, flash)
}

#[cfg(test)]
mod tests {
    use crate::tests::{flash_command, resolved_config};
    use crate::{CoreError, resolve_flash_command};

    #[test]
    fn resolve_flash_command_errors_when_not_configured() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        assert!(matches!(
            resolve_flash_command(&cfg),
            Err(CoreError::FlashNotConfigured { .. })
        ));
    }

    #[test]
    fn resolve_flash_command_errors_on_unknown_placeholder() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let mut cfg = resolved_config(temp_dir.path());
        cfg.flash = Some(flash_command("tool", &["{unknown}"]));

        assert!(matches!(
            resolve_flash_command(&cfg),
            Err(CoreError::UnknownCommandPlaceholder { .. })
        ));
    }

    #[test]
    fn resolve_flash_command_errors_on_missing_artifact() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let mut cfg = resolved_config(temp_dir.path());
        cfg.flash = Some(flash_command("tool", &["{hex}"]));

        assert!(matches!(
            resolve_flash_command(&cfg),
            Err(CoreError::RequiredCommandArtifactMissing { artifact, .. }) if artifact == "hex"
        ));
    }
}

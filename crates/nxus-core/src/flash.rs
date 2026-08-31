use std::path::Path;

use crate::{Cmd, CoreError, CoreResult, ResolvedConfig, paths};

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

    let program = expand_template(cfg, &flash.command)?;
    let args = flash
        .args
        .iter()
        .map(|arg| expand_template(cfg, arg))
        .collect::<CoreResult<Vec<_>>>()?;

    Ok(Cmd::new(program).args(args))
}

/// Expands supported `{placeholder}` segments inside a configured flash string.
fn expand_template(cfg: &ResolvedConfig, template: &str) -> CoreResult<String> {
    let mut output = String::new();
    let mut rest = template;

    while let Some((prefix, placeholder)) = rest.split_once('{') {
        output.push_str(prefix);
        let Some((name, suffix)) = placeholder.split_once('}') else {
            return Err(CoreError::UnknownPlaceholder {
                placeholder: placeholder.to_owned(),
            });
        };

        output.push_str(&placeholder_value(cfg, name)?);
        rest = suffix;
    }

    output.push_str(rest);
    Ok(output)
}

/// Resolves a single supported flash placeholder value.
fn placeholder_value(cfg: &ResolvedConfig, placeholder: &str) -> CoreResult<String> {
    match placeholder {
        "project_dir" => Ok(path_string(&cfg.ctx.project_dir)),
        "workspace_dir" => Ok(path_string(&cfg.workspace_root)),
        "build_dir" => Ok(path_string(&paths::build_dir(cfg, &cfg.profile))),
        "profile" => Ok(cfg.profile.clone()),
        "elf" => required_artifact("elf", &paths::firmware_elf(cfg, &cfg.profile)),
        "bin" => required_artifact("bin", &paths::firmware_bin(cfg, &cfg.profile)),
        "hex" => required_artifact("hex", &paths::firmware_hex(cfg, &cfg.profile)),
        _ => Err(CoreError::UnknownPlaceholder {
            placeholder: placeholder.to_owned(),
        }),
    }
}

/// Returns a required artifact path as a string, erroring when it is missing.
fn required_artifact(artifact: &str, path: &Path) -> CoreResult<String> {
    if path.is_file() {
        return Ok(path_string(path));
    }

    Err(CoreError::FlashArtifactMissing {
        artifact: artifact.to_owned(),
        path: path.to_path_buf(),
    })
}

/// Converts a path to its display string form.
fn path_string(path: &Path) -> String {
    path.display().to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::tests::{flash_command, resolved_config};
    use crate::{CoreError, paths, resolve_flash_command};

    #[test]
    fn resolve_flash_command_preserves_program_and_argument_boundaries() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let mut cfg = resolved_config(temp_dir.path());
        fs::create_dir_all(paths::build_dir(&cfg, &cfg.profile)).expect("build dir should exist");
        fs::write(paths::firmware_elf(&cfg, &cfg.profile), "elf").expect("elf should exist");
        cfg.flash = Some(flash_command(
            "openocd",
            &[
                "-f",
                "board/st.cfg",
                "-c",
                "program {elf} verify reset exit",
            ],
        ));

        let cmd = resolve_flash_command(&cfg).expect("flash command should resolve");

        assert_eq!(cmd.program.to_string_lossy(), "openocd");
        assert_eq!(
            cmd.args
                .first()
                .expect("first arg should exist")
                .to_string_lossy(),
            "-f"
        );
        assert_eq!(
            cmd.args
                .get(1)
                .expect("second arg should exist")
                .to_string_lossy(),
            "board/st.cfg"
        );
        assert!(
            cmd.args
                .get(3)
                .expect("program arg should exist")
                .to_string_lossy()
                .contains("nuttx verify reset exit")
        );
    }

    #[test]
    fn resolve_flash_command_expands_path_placeholders() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let mut cfg = resolved_config(temp_dir.path());
        fs::create_dir_all(paths::build_dir(&cfg, &cfg.profile)).expect("build dir should exist");
        fs::write(paths::firmware_elf(&cfg, &cfg.profile), "elf").expect("elf should exist");
        fs::write(paths::firmware_bin(&cfg, &cfg.profile), "bin").expect("bin should exist");
        fs::write(paths::firmware_hex(&cfg, &cfg.profile), "hex").expect("hex should exist");
        cfg.flash = Some(flash_command(
            "tool",
            &[
                "{project_dir}",
                "{workspace_dir}",
                "{build_dir}",
                "{profile}",
                "{elf}",
                "{bin}",
                "{hex}",
            ],
        ));

        let cmd = resolve_flash_command(&cfg).expect("flash command should resolve");

        assert_eq!(
            cmd.args
                .first()
                .expect("first arg should exist")
                .to_string_lossy(),
            cfg.ctx.project_dir.display().to_string()
        );
        assert_eq!(
            cmd.args
                .get(1)
                .expect("second arg should exist")
                .to_string_lossy(),
            cfg.workspace_root.display().to_string()
        );
        assert_eq!(
            cmd.args
                .get(2)
                .expect("third arg should exist")
                .to_string_lossy(),
            paths::build_dir(&cfg, &cfg.profile).display().to_string()
        );
        assert_eq!(
            cmd.args
                .get(3)
                .expect("profile arg should exist")
                .to_string_lossy(),
            cfg.profile
        );
        assert_eq!(
            cmd.args
                .get(4)
                .expect("elf arg should exist")
                .to_string_lossy(),
            paths::firmware_elf(&cfg, &cfg.profile)
                .display()
                .to_string()
        );
        assert_eq!(
            cmd.args
                .get(5)
                .expect("bin arg should exist")
                .to_string_lossy(),
            paths::firmware_bin(&cfg, &cfg.profile)
                .display()
                .to_string()
        );
        assert_eq!(
            cmd.args
                .get(6)
                .expect("hex arg should exist")
                .to_string_lossy(),
            paths::firmware_hex(&cfg, &cfg.profile)
                .display()
                .to_string()
        );
    }

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
            Err(CoreError::UnknownPlaceholder { .. })
        ));
    }

    #[test]
    fn resolve_flash_command_errors_on_missing_artifact() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let mut cfg = resolved_config(temp_dir.path());
        cfg.flash = Some(flash_command("tool", &["{hex}"]));

        assert!(matches!(
            resolve_flash_command(&cfg),
            Err(CoreError::FlashArtifactMissing { artifact, .. }) if artifact == "hex"
        ));
    }
}

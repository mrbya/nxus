use std::process::ExitCode;

use nxus_core::ResolvedConfig;

/// Nxus command: profiles.
pub fn profiles(cfg: &ResolvedConfig) -> ExitCode {
    if cfg.profiles.is_empty() {
        eprintln!("akafuka");
        return ExitCode::FAILURE;
    }

    println!(
        "{:<12} {:<12} {:<12} {:<36}",
        "Profile", "Family", "Arch", "Target"
    );
    println!("{:-<12} {:-<12} {:-<12} {:-<36}", "", "", "", "");

    for (profile, config) in &cfg.profiles {
        println!(
            "{:<12} {:<12} {:<12} {}:{}",
            profile, config.arch, config.family, config.board, config.config_base
        );
    }

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use std::process::ExitCode;

    use crate::commands::profiles;
    use crate::tests::resolved_config;

    #[test]
    fn profiles_fails_when_no_profiles_are_configured() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let mut cfg = resolved_config(temp_dir.path());
        cfg.profiles.clear();

        assert_eq!(profiles(&cfg), ExitCode::FAILURE);
    }

    #[test]
    fn profiles_succeeds_when_profiles_are_present() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        assert_eq!(profiles(&cfg), ExitCode::SUCCESS);
    }
}

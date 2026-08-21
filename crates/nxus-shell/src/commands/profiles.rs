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

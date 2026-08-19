use std::process::ExitCode;

use indexmap::IndexMap;
use nxus_core::{ProfileConfig, ResolvedConfig};

/// Nxus command: profiles.
pub fn profiles(_: ResolvedConfig, profiles: IndexMap<String, ProfileConfig>) -> ExitCode {
    if profiles.is_empty() {
        eprintln!("akafuka");
        return ExitCode::FAILURE;
    }

    println!(
        "{:<12} {:<12} {:<12} {:<24} {:<12}",
        "Profile", "Family", "Arch", "Board", "Config base"
    );
    println!(
        "{:-<12} {:-<12} {:-<12} {:-<24} {:-<12}",
        "", "", "", "", ""
    );

    for (profile, config) in profiles {
        println!(
            "{:<12} {:<12} {:<12} {:<24} {:<12}",
            profile, config.arch, config.family, config.board, config.config_base
        );
    }

    ExitCode::SUCCESS
}

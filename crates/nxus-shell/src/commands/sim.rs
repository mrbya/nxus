use std::process::ExitCode;

use nxus_core::ResolvedConfig;

use crate::commands::run_binary;

/// Nxus command: sim
pub fn sim(cfg: &ResolvedConfig) -> ExitCode {
    let config = cfg.with_profile("sim");

    if run_binary(&config) == ExitCode::FAILURE {
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::process::ExitCode;

    use nxus_core::paths;

    use crate::commands::sim;
    use crate::tests::resolved_config;

    #[test]
    fn sim_runs_using_sim_profile() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        fs::create_dir_all(paths::build_dir(&cfg, "sim")).expect("sim build dir should exist");

        assert_eq!(sim(&cfg), ExitCode::SUCCESS);
    }
}

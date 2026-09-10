use std::process::ExitCode;

use nxus_core::ResolvedConfig;

use crate::commands::{clean, run_binary};

/// Nxus command: test
pub fn test(cfg: &ResolvedConfig) -> ExitCode {
    let config = cfg.with_profile("test");

    if config.clean && clean(&config) == ExitCode::FAILURE {
        return ExitCode::FAILURE;
    }

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

    use crate::commands::test;
    use crate::tests::resolved_config;

    #[test]
    fn test_runs_using_test_profile() {
        let temp_dir = tempfile::TempDir::new().expect("tempdir should be created");
        let cfg = resolved_config(temp_dir.path());

        fs::create_dir_all(paths::build_dir(&cfg, "test")).expect("test build dir should exist");
        fs::write(
            paths::build_dir(&cfg, "test").join("compile_commands.json"),
            "{}",
        )
        .expect("file should be written");

        assert_eq!(test(&cfg), ExitCode::SUCCESS);
    }
}

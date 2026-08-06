use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn runs_cli_and_parses_greet() {
    Command::cargo_bin("nxus")
        .expect("binary should exist")
        .args(["greet", "--name", "BruceLee"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Hello from"))
        .stdout(predicate::str::contains("BruceLee"));
}

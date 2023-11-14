use std::process::Command;
use assert_cmd::prelude::*;
use predicates::prelude::*; // Used for writing assertions

#[test]
fn run_with_defaults() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .assert()
        .success()
        .stdout(predicate::str::contains("Meow!"));

    Ok(())
}

#[test]
fn fail_on_non_existing_file() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin("catsay")
        .expect("binary exists")
        .args(&["--file", "non-existing-file"])
        .assert()
        .failure();

    Ok(())
}
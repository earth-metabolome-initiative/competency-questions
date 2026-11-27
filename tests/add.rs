#![cfg(unix)]

use std::process::Command;

#[test]
fn independent_checkouts_use_the_registry_and_keep_existing_questions() {
    let status = Command::new("sh")
        .args(["-eu", "-c", include_str!("fixtures/add.sh")])
        .env("CATALOGUE", env!("CARGO_BIN_EXE_competency-questions"))
        .status()
        .unwrap();
    assert!(status.success());
}

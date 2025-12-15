//! Integration tests for the cargo-nfpm cli.
#![allow(deprecated)]

#[test]
fn print_help() {
    let mut cmd = assert_cmd::Command::cargo_bin("cargo-nfpm").unwrap();
    cmd.arg("--help")
        .assert()
        .success()
        .stdout(predicates::str::contains("Usage:"));
}

#[test]
fn nfpm_command_exists() {
    let mut cmd = assert_cmd::Command::cargo_bin("cargo-nfpm").unwrap();
    cmd.args(["nfpm", "--help"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Usage:"));
}

#[test]
fn nfpm_package_command_exists() {
    let mut cmd = assert_cmd::Command::cargo_bin("cargo-nfpm").unwrap();
    cmd.args(["nfpm", "package", "--help"])
        .assert()
        .success()
        .stdout(predicates::str::contains("Usage:"));
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_single_project() {
    use std::path::PathBuf;

    let mut cmd = assert_cmd::Command::cargo_bin("cargo-nfpm").unwrap();
    cmd.args(["nfpm", "package", "-f", "deb", "-s", "skip"])
        .current_dir("./test-projects/single-project")
        .assert()
        .success();

    let debpath = PathBuf::from(
        "./test-projects/single-project/target/release/single-project_0.1.0-1_amd64.deb",
    );
    assert!(debpath.exists());
}

#[test]
#[cfg(target_arch = "x86_64")]
fn test_workspace_project() {
    use std::path::PathBuf;

    let mut cmd = assert_cmd::Command::cargo_bin("cargo-nfpm").unwrap();
    cmd.args(["nfpm", "package", "-f", "deb", "-s", "skip", "-p", "bin1"])
        .current_dir("./test-projects/workspace-project")
        .assert()
        .success();

    let debpath =
        PathBuf::from("./test-projects/workspace-project/target/release/bin1_1.0.0-1_amd64.deb");
    assert!(debpath.exists());
}

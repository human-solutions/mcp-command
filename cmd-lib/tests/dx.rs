#![cfg(feature = "dx-test")]
use std::process::{Command, Stdio};
use std::time::Duration;
use tempfile::tempdir;

#[test]
fn install_and_run_dx() {
    // Install the dx CLI
    let status = Command::new("cargo")
        .args(["install", "dioxus-cli"])
        .status()
        .expect("failed to install dioxus-cli");
    assert!(status.success());

    // Create a temporary project directory
    let dir = tempdir().expect("failed to create temp dir");
    let project = dir.path().join("test_app");

    // Create a new dioxus project
    let status = Command::new("dx")
        .args(["new", project.to_str().unwrap()])
        .status()
        .expect("failed to create project");
    assert!(status.success());

    // Run `dx serve` in the project directory
    let mut child = Command::new("dx")
        .arg("serve")
        .current_dir(&project)
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to run dx serve");

    std::thread::sleep(Duration::from_secs(2));
    let _ = child.kill();
    let _ = child.wait();
}

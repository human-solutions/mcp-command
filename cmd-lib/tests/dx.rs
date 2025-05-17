#![cfg(feature = "dx-test")]

use std::time::Duration;
use tempfile::tempdir;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::process::Command;

/// Installs the `dx` CLI, creates a sample project, runs `dx serve`,
/// sends an `r` to trigger a rebuild and then terminates the process.
#[tokio::test]
async fn install_and_control_dx() {
    // Install the dx CLI. Output is inherited so progress is visible.
    let status = Command::new("cargo")
        .args(["install", "dioxus-cli"])
        .status()
        .await
        .expect("failed to install dioxus-cli");
    assert!(status.success());

    // Create a temporary project directory
    let dir = tempdir().expect("failed to create temp dir");
    let project = dir.path().join("test_app");

    // Create a new dioxus project
    let status = Command::new("dx")
        .args(["new", project.to_str().unwrap()])
        .status()
        .await
        .expect("failed to create project");
    assert!(status.success());

    // Start `dx serve` and capture its stdio
    let mut child = Command::new("dx")
        .arg("serve")
        .current_dir(&project)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("failed to run dx serve");

    let mut stdout = child.stdout.take().expect("stdout");
    let mut stderr = child.stderr.take().expect("stderr");
    let mut stdin = child.stdin.take().expect("stdin");

    // Forward stdout
    let out_task = tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        let mut out = io::stdout();
        while let Ok(n) = stdout.read(&mut buf).await {
            if n == 0 {
                break;
            }
            let _ = out.write_all(&buf[..n]).await;
        }
    });

    // Forward stderr
    let err_task = tokio::spawn(async move {
        let mut buf = [0u8; 1024];
        let mut err = io::stderr();
        while let Ok(n) = stderr.read(&mut buf).await {
            if n == 0 {
                break;
            }
            let _ = err.write_all(&buf[..n]).await;
        }
    });

    // Wait briefly for the server to start then request a rebuild
    tokio::time::sleep(Duration::from_secs(2)).await;
    stdin
        .write_all(b"r\n")
        .await
        .expect("failed to send rebuild command");

    // Allow some time for the rebuild output to appear
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Terminate the process
    let _ = child.kill().await;
    let _ = child.wait().await;

    let _ = out_task.await;
    let _ = err_task.await;
}


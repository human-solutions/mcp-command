use std::process::{Command, Output};

/// Run a command and capture its output.
///
/// # Examples
///
/// ```
/// use cmd_lib::run;
///
/// let output = run("echo", &["hello"]).expect("command failed");
/// assert!(output.status.success());
/// ```
pub fn run(cmd: &str, args: &[&str]) -> std::io::Result<Output> {
    Command::new(cmd).args(args).output()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn echo_works() {
        let output = run("echo", &["test"]).expect("command failed");
        assert!(output.status.success());
        assert_eq!(String::from_utf8_lossy(&output.stdout).trim(), "test");
    }
}

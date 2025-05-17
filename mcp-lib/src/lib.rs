/// Utility functions for working with MCP commands.
pub struct Mcp;

impl Mcp {
    /// Convert raw command output into a trimmed String.
    pub fn process_output(output: &[u8]) -> String {
        String::from_utf8_lossy(output).trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_output_trims() {
        let out = Mcp::process_output(b"hello\n");
        assert_eq!(out, "hello");
    }
}

/// Utility functions for working with MCP commands.
pub struct Mcp;

impl Mcp {
    /// Convert raw command output into a trimmed String.
    pub fn process_output(output: &[u8]) -> String {
        String::from_utf8_lossy(output).trim().to_string()
    }
}

/// Start a simple "Hello World" MCP server over stdio.
///
/// This uses the `rmcp` crate's stdio server functionality.
pub fn start_hello_server() -> std::io::Result<()> {
    use rmcp::stdio::Server;

    let mut server = Server::new();
    println!("MCP stdio server ready");

    for mut connection in server.incoming() {
        let mut stream = connection?;
        stream.send(b"Hello World\n")?;
    }

    Ok(())
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

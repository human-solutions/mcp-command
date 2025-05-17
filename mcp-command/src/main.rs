use cmd_lib::run;
use mcp_lib::Mcp;

fn main() {
    let output = match run("echo", &["Hello from mcp-command"]) {
        Ok(out) => out,
        Err(err) => {
            eprintln!("failed to run command: {err}");
            std::process::exit(1);
        }
    };

    let formatted = Mcp::process_output(&output.stdout);
    println!("{formatted}");
}

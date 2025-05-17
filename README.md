# mcp-command Workspace

This repository contains a Cargo workspace with three crates:

- **cmd-lib**: a small wrapper around `std::process::Command` for running commands.
- **mcp-lib**: helpers for processing command output.
- **mcp-command**: binary that ties everything together.

Run the example binary with `cargo run -p mcp-command`.

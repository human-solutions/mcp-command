# MCP Command Server PRD

## Overview

The MCP Command server provides a way for an LLM or user to manage interactive command-line tools. It acts as an MCP (Model Context Protocol) server that launches and interacts with external processes based on configurations defined in TOML files.

## Goals

- Provide a server that can start, control, and monitor interactive CLI tools.
- Support automated interaction with tools such as databases, web servers, build tools, or custom utilities.
- Allow configuration through local or global TOML files so no code changes are required for new tools.

## Configuration

1. **Local configuration**: search for `.mcp-command.toml` starting in the current directory and then each parent directory. Use the first file found.
2. **Fallback configuration**: if no local file is found, use `~/.config/mcp-command/config.toml`.
3. **Tool sections**: each tool is defined under its own section. The section name is the command name used with `mcp-command <toolname>`.
4. Each tool entry includes:
   - Description of the tool
   - Command to start the tool
   - Operations that describe how to interact with the process
   - Expected status outcome with optional failure states and time-out values

Example snippet:
```toml
[dioxus]
description = "Dioxus CLI"
command = "dx serve -p my-app"
expected_status = "Serving"
error_states = ["Error", "Aborted"]
timeout_secs = 30
```

## Operations

An operation is a single interaction with a process. The MCP Command server should support:

- **start**: launch the configured command with the environment variables inherited from the user's shell (bash, zsh, etc).
- **send_signal**: send POSIX signals (e.g. `SIGINT` for Ctrl-C) to the process.
- **send_key**: send keystrokes such as `r` or `v` to the process's stdin.
- **get_output**: retrieve stdout/stderr output, including handling TUI output and streaming logs.
- **stop**: terminate the process gracefully and report its final status.

Operations can be chained in the configuration to express how the server should respond to different states of the command.

## Waiting for Outcome

Tools may take time to initialize. Each tool configuration allows waiting for an expected status string. The configuration may also define non-success statuses that interrupt the wait. A timeout ensures the command does not wait forever.

Example wait configuration:
```toml
[dioxus.rebuild]
description = "Trigger a rebuild for the dx tool"
keystroke = "r"

[dioxus.rebuild.wait]
for = "Status: Serving"
not = ["Status: Error", "Status: Aborted"]
timeout_secs = 60
```

## Usage

After configuring a tool, start the server with:
```
mcp-command <toolname>
```
The server reads the configuration and performs the defined operations to control the tool. For example, starting the Dioxus development server and monitoring its status messages.

## Use Cases

- Launch a database and wait for the ready message before running integration tests.
- Start a web server, watch for compile states, and send a rebuild command when requested.
- Manage long-running utilities that require periodic keystrokes or signals.


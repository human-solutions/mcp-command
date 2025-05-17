# AGENT Guidelines

This repository is a Cargo workspace with multiple crates. Follow these rules
when making changes:

1. **Verifications and formatting**. Always run before committing.
   - Run `cargo fmt --all`.
   - Run `cargo clippy`.
   - Run `cargo test --workspace` and ensure it passes.

2. **Comments and Tests**
   - Do not add comments that simply restate obvious code.
   - Only write tests that provide real value.

3. **Pull Requests**
   - Summarize code changes and include test results in the PR description.

## Rust documention

Use the ruskel tool to get latest documentation for a crate. It can be installed with `cargo install ruskel`.

Use it like:

```
# Current project
ruskel

# If we're in a workspace and we have a crate mypacakage
ruskel mypackage

# A dependency of the current project, else we fetch from crates.io 
ruskel serde

# A sub-path within a crate
ruskel serde::de::Deserialize 

# Path to a crate
ruskel /my/path

# A module within that crate
ruskel /my/path::foo

# A crate from crates.io with a specific version
ruskel serde@1.0.0
```

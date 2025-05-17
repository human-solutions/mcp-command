# AGENT Guidelines

This repository is a Cargo workspace with multiple crates. Follow these rules
when making changes:

1. **Formatting and Tests**
   - Always run `cargo fmt --all` before committing.
   - Run `cargo test --workspace` and ensure it passes.

2. **Comments and Tests**
   - Do not add comments that simply restate obvious code.
   - Only write tests that provide real value.

3. **Pull Requests**
   - Summarize code changes and include test results in the PR description.

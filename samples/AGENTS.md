# AGENTS.md

Follow [../AGENTS.md](../AGENTS.md).

## Samples

- Use the latest stable published versions of repo crates.
- Use path dependencies only to show unpublished functionality that cannot live in a crate's `examples/` directory.
- Never use path dependencies for convenience or to track unreleased versions.
- If a sample uses a local `path` crate, mirror its direct Azure SDK `path + version` dependencies for any public types that cross the boundary; do not mix published and local crate instances there.

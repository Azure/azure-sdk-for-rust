# AGENTS.md

Follow [../AGENTS.md](../AGENTS.md).

## Samples

- Use the latest stable published versions of repo crates.
- Use path dependencies only to show unpublished functionality that cannot live in a crate's `examples/` directory.
- Never use path dependencies for convenience or to track unreleased versions.
- If a sample uses a local `path` crate, mirror its direct Azure SDK `path + version` dependencies for any public types that cross the boundary; do not mix published and local crate instances there.

### Organization

- Place each sample in `samples/{service directory}/{sample}`. The service directory must match an existing service directory under `sdk/`; for example, `samples/cosmos` contains samples for `sdk/cosmos`.
- Choose the `sdk/` service directory for the predominant Azure service crate used by the sample. If the predominant service is indeterminate, ask the user which service directory to use before creating or moving the sample.
- For every `samples/{service directory}`, ensure [.github/CODEOWNERS](../.github/CODEOWNERS) has a matching `/sdk/{service directory}/` entry and a `/samples/{service directory}/` entry with the same owners.
- Apply spelling corrections to `samples/{service directory}` using the same pattern as the corresponding `sdk/{service directory}`.

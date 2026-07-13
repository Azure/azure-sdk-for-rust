# Core SDK Instructions

These rules apply in addition to the repository root [AGENTS.md](../../AGENTS.md).

## Semver Propagation

- Treat `sdk/core` as an ordered stack: `typespec -> typespec_client_core -> azure_core`.
- `typespec_client_core` re-exports public APIs from `typespec`; `azure_core` re-exports public APIs `typespec_client_core` with few exceptions.
- A semver-relevant change in a lower crate requires **at least** the same bump in each higher crate.
- A lower crate may force a higher bump, but a higher crate can still require a larger bump because of its own public API changes.

- A new public feature in `typespec` requiring `1.0.x -> 1.1.0` means `typespec_client_core` and `azure_core` must be at least `1.1.0`.
- A new public feature in `typespec_client_core` requiring `1.0.x -> 1.1.0` means `azure_core` must be at least `1.1.0`.
- A breaking change in `typespec` requiring `2.0.0` means `typespec_client_core` and `azure_core` must be at least `2.0.0`.

## Changelog Propagation

- Reflect `typespec` changelog changes in `typespec_client_core` when they affect its shipped dependency surface or release decision.
- Reflect `typespec_client_core` changelog changes in `azure_core` when they affect its shipped dependency surface or release decision.
- If a higher crate is bumped only because of a propagated dependency change, its `CHANGELOG.md` should still mention that dependency-driven update.

## Dependencies and Versioning

- Keep versions for `sdk/core` dependencies managed from the root workspace `Cargo.toml`.
- Prefer `workspace = true` in `sdk/core` crate manifests when inheriting workspace-managed dependencies.
- For unreleased local `sdk/core` dependencies, use the workspace table for the needed `path + version` entry.
- Keep crates in next-release state in their own manifests; manage cross-crate version wiring from the workspace table.

### Published vs. unpublished graphs

- Mixing published and local versions of `typespec`, `typespec_client_core`, `azure_core`, or crates that expose their types can create duplicate-type failures.
- Common breakage looks like mismatched `ClientOptions`, `TokenCredential`, or trait implementations that appear identical but come from different crate instances.
- When changing one side of a test/example graph to local source, check all directly exchanged public types in that graph and keep them on the same crate instance.

### Example and documentation stub crates

- `azure_core_examples` is a non-published helper crate for compile-only examples and docs. It intentionally depends on local `sdk/core/azure_core` by path.
- Keep stubbed example APIs small and shaped like the public APIs they stand in for, but do not add real behavior unless tests need it.
- Organize identity stubs in `azure_core_examples::identity` per credential module, mirroring `azure_identity` where practical, and re-export them from `identity/mod.rs`.
- For markdown compile tests, alias helper modules to the public crate names used in docs (for example `use azure_core_examples::identity as azure_identity;`).
- If a compile-only test needs just one or two service types and would otherwise mix published and local `azure_core` graphs, prefer a tiny local stub in that test module over broad dependency rewiring.

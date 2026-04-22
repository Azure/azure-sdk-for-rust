# Check API Superset

Verifies that `azure_core` exports all public types from `typespec_client_core`, and that `typespec_client_core` exports all public types from `typespec`. This includes types surfaced through `pub use` re-exports and glob re-exports (`pub use dep::module::*`).

Only type names and module paths are compared — struct fields, enum variants, and other definitions are not checked.

## Usage

Run from the repository root:

```sh
cargo run --manifest-path eng/tools/check_api_superset/Cargo.toml
```

The tool exits with code 0 if all types are present, or code 1 with a markdown-formatted list of missing types.

## Exemptions

To exempt specific types from the check, add their missing fully-qualified paths — one per line — to `eng/tools/check_api_superset/exemptions.txt`. Use the crate where the type is missing from:

```text
azure_core::http::REDACTED_PATTERN
typespec_client_core::http::QueryBuilder
```

Blank lines and lines starting with `#` are ignored.

## How It Works

1. Generates rustdoc JSON for `typespec`, `typespec_client_core`, and `azure_core` using a nightly toolchain.
2. Walks each crate's public module tree to collect exported type names (structs, enums, functions, traits, etc.).
3. Resolves glob and module re-exports across crate boundaries using the dependency crate's JSON.
4. Reports any types missing from the expected superset crate.

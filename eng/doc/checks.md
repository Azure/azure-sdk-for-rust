# Engineering System checks

These checks run in addition to logic tests.

## Standard checks

The Engineering System provides a set of [standard checks](https://dev.azure.com/azure-sdk/internal/_wiki/wikis/internal.wiki/1215/Source-and-Artifact-scans-and-checks).

## Rust-specific checks

### Checks included in `cargo`

These following checks are included in the Rust toolchain or are part of cargo.

#### Fmt

Documentation: [cargo-fmt](https://doc.rust-lang.org/cargo/commands/cargo-fmt.html)

Runs in: [Analyze-Code.ps1](https://github.com/Azure/azure-sdk-for-rust/blob/main/eng/scripts/Analyze-Code.ps1)

```
cargo fmt --all -- --check
```

#### Clippy

Documentation: [cargo-clippy](https://doc.rust-lang.org/cargo/commands/cargo-clippy.html)

Runs in: [Analyze-Code.ps1](https://github.com/Azure/azure-sdk-for-rust/blob/main/eng/scripts/Analyze-Code.ps1)

```
cargo clippy --target=wasm32-unknown-unknown --workspace --keep-going --no-deps
```

#### Check

> [!NOTE]  
> This only runs against `azure-core` and not against other packages in the build

Documentation: [cargo-check](https://doc.rust-lang.org/cargo/commands/cargo-check.html)

Runs in: [Analyze-Code.ps1](https://github.com/Azure/azure-sdk-for-rust/blob/main/eng/scripts/Analyze-Code.ps1)

```
cargo check --package azure_core --all-features --all-targets --keep-going
```

#### Doc

Documentation: [cargo-doc](https://doc.rust-lang.org/cargo/commands/cargo-doc.html)

Runs in: [Analyze-Code.ps1](https://github.com/Azure/azure-sdk-for-rust/blob/main/eng/scripts/Analyze-Code.ps1)

```
cargo doc --workspace --no-deps --all-features
```

#### Docs-rs

Documentation: [cargo-docs-rs](https://github.com/dtolnay/cargo-docs-rs)

Runs in: [Analyze-Code.ps1](https://github.com/Azure/azure-sdk-for-rust/blob/main/eng/scripts/Analyze-Code.ps1)

```
cargo install --locked cargo-docs-rs
cargo +nightly docs-rs --package <pkg>
```

### Checks installed as tools

Additional checks can be installed as tools by `cargo install` and run in relvant parts of the build system.

### Deny

Documentation: [cargo-deny](https://github.com/EmbarkStudios/cargo-deny)

Runs in [Analyze-Code.ps1](https://github.com/Azure/azure-sdk-for-rust/blob/main/eng/scripts/Analyze-Code.ps1)

```
cargo install cargo-deny
cargo deny --all-features check
```

### Semver

Documentation: [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks)

Runs in [Test-Semver.ps1](https://github.com/Azure/azure-sdk-for-rust/blob/main/eng/scripts/Test-Semver.ps1)

```
cargo install cargo-semver-checks
cargo semver-checks --package <pkg1> --package <pkg2> ...
```

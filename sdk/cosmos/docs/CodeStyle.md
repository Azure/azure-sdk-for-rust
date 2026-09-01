# Cosmos Rust Code Style

This file supplements the repository-wide Rust instructions. It records only
Cosmos-specific or frequently missed conventions; architecture and feature
behavior belong in [Project.md](Project.md), [Architecture.md](Architecture.md),
the relevant specification, or an ADR.

## Public APIs and types

Follow the Azure SDK Design Guidelines for Rust and the established API shape in
the crate being changed.

- Keep SDK public types independent from driver types. Convert explicitly at
  the crate boundary.
- Prefer standard traits such as `From`, `TryFrom`, `FromStr`, `Display`,
  `Default`, and `IntoIterator` over equivalent ad hoc methods.
- Use builders when construction has multiple optional values or invalid
  intermediate states. Do not add a builder around a simple constructor.
- Avoid public lifetime parameters unless borrowing materially improves the API.
- Use `#[non_exhaustive]` for public structs and enums.
- Derive `SafeDebug`, not `Debug`, for models that can contain customer data or
  operational metadata.
- Keep public Cargo features within the categories defined by the feature ADR.
  Internal-only features must retain their `__internal_` naming and visibility.

During pre-GA `0.x` releases, prefer clean breaking API changes over aliases,
deprecated forwarding methods, dual representations, or other compatibility
shims.

## Modules and imports

- Put imports at module scope.
- Import every item explicitly; never use glob imports.
- Group imports by source crate, with `std` at the top.
- In non-test code, use `crate::` for items in the current crate.
- In `#[cfg(test)]` modules, import the subject from `super`.
- Merge related imports, including standard-library imports.
- Keep clients, options, diagnostics, errors, and internal pipeline state in
  their existing dedicated modules. Do not create a general shared-model crate.

```rust
use std::{sync::Arc, time::Duration};

use crate::{
    error::Result,
    models::{CosmosOperation, OperationResponse},
};
```

Wire-format types belong with the protocol or models that serialize them.
Runtime configuration belongs in options. Operational telemetry belongs in
diagnostics. Customer item types do not belong in the driver.

## Errors

- Use `azure_data_cosmos::Result<T>` in the SDK and the driver's crate-local
  `Result<T>` alias in the driver.
- Propagate failures with `?` and preserve the Cosmos status/sub-status
  classification across layers.
- Retain request charge, activity ID, diagnostics, and retry context when they
  are available.
- Return an error for invalid user-derived input. Panic only for an internal
  invariant whose violation is a programming defect.
- Do not erase a typed Cosmos error into a string merely to cross an internal
  boundary.

```rust
fn parse_page(body: &[u8]) -> Result<Page> {
    serde_json::from_slice(body).map_err(Error::from)
}
```

If an infallible function needs to produce an error, raise this with the user to
confirm intent. Strongly prefer changing its signature to return `Result`
instead of swallowing the failure, logging and continuing, manufacturing a
default, or panicking.

## Documentation

Document every public item with a concise summary, a blank line, and details
that affect correct use.

Include only what is relevant:

- returned errors and partial-result behavior;
- partition-key or continuation-token semantics;
- request-unit or performance implications;
- feature gates, support level, or preview status;
- a short example when the usage is not evident from the signature.

Use exact intra-doc links and verify them with `cargo doc`. README examples with
placeholder credentials or endpoints use `rust no_run`.

```rust
/// Reads one item from a logical partition.
///
/// Returns an error when the item does not exist.
pub async fn read_item<T>(&self, id: &str, key: PartitionKey) -> Result<T> {
    // ...
}
```

NEVER include references to internal engineering documentation (specs, ADRs, etc.)
in doc comments. Doc comments are ALWAYS designed for the consumer of the API.
Even doc comments on internal APIs should focus on documenting the API behaviour,
not referencing the spec.

Every new Rust source file starts with:

```rust
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
```

## Generated code

Never edit a file under `generated/`. Change the TypeSpec definition, emitter,
or supported customization surface and regenerate. Do not copy generated code
into a handwritten module to bypass generation.

Handwritten adapters around generated clients are acceptable when they preserve
the generated ownership boundary and avoid duplicate exports.

## Tests

Put unit tests at the end of the module unless the existing module uses a
neighboring test file.

```rust
#[cfg(test)]
mod tests {
    use super::order_regions;

    #[test]
    fn orders_regions_by_preference() {
        let actual = order_regions(["westus", "eastus"]);
        assert_eq!(actual, ["westus", "eastus"]);
    }
}
```

- Name tests for the behavior, without a redundant `test_` prefix.
- Prefer exact assertions over partial assertions.
- Use `assert!(condition)` rather than comparing a Boolean with `true`.
- Always prefer `assert_eq!` and `assert_ne!` over manual comparisons for equality and inequality.
- Always prefer "complete" assertions over containment checks (avoid `assert!` on `contains(...)`), round-tripping of serialization/deserialization code, etc., to ensure thorough test coverage.
- Test meaningful parsing, conversion, request construction, pipeline
  decisions, failures, and regressions.
- Use existing in-memory, hosted-emulator, or legacy-emulator categories for
  deterministic integration coverage; preserve their `test_category` gates.
- Do not add tests for getters, direct field assignment, or compiler-derived
  behavior.

Follow the
[Cosmos emulator-test skill](../.github/skills/emulator-tests/SKILL.md) for
emulator testing.

## Validation

Follow the [Cosmos validation skill](../.github/skills/validate/SKILL.md).
It is the single source of truth for completion criteria and commands. Fix
warnings rather than suppressing them; when an external or FFI constraint
requires a Clippy allowance, explain the constraint next to the allowance.

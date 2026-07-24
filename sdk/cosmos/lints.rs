// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Shared lint policy for all `sdk/cosmos` crates.
//
// These lints are enforced via the `[lints.clippy]` table in each crate's
// `Cargo.toml`.  This file exists as the canonical documentation for the
// policy; if you change a lint here you must also update the corresponding
// Cargo.toml entries.
//
// Lints that need to be disabled in `#[cfg(test)]` code (so that test helpers
// can use `.unwrap()` / `.expect()` freely) are applied via an inline
// `#![cfg_attr(not(test), deny(...))]` in each crate's root module because
// Cargo.toml lints cannot be conditioned on `cfg(test)`.

// Prefer `#[expect(lint)]` over `#[allow(lint)]` so that suppressed lints are
// re-evaluated each time the compiler runs and stale suppressions are caught.
// Cargo.toml key: clippy.allow_attributes = "deny"

// Every public function that can panic must document the panic conditions.
// Cargo.toml key: clippy.missing_panics_doc = "deny"

// `#[should_panic]` tests must specify the expected message.
// Cargo.toml key: clippy.should_panic_without_expect = "deny"

// Functions that return `Result` should propagate errors rather than calling
// `.unwrap()` / `.expect()` internally.
// Cargo.toml key: clippy.unwrap_in_result = "deny"

// Prefer `.expect("reason")` over `.unwrap()` in non-test code so that panics
// include a meaningful diagnostic message.
// Applied inline as: #![cfg_attr(not(test), deny(clippy::unwrap_used))]

// Wildcard imports make it hard to trace where names come from.  `use super::*`
// inside `#[cfg(test)]` modules is explicitly allowed by clippy.
// Cargo.toml key: clippy.wildcard_imports = "deny"

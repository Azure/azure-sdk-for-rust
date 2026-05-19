# Slice 2 Brief — Gateway 2.0 Foundation (Helper + Constants + Account-Name Extraction)

> **Audience**: a sub-agent dispatched to implement this slice on branch `tvaron3/gateway-2.0-impl`.
> **Working directory**: `/Users/tomasvaron/sdks/client-engine/worktrees/wc`.
> **Authoritative spec**: `sdk/cosmos/azure_data_cosmos_driver/docs/GATEWAY_20_SPEC.md` (especially §5 Phase 2 lines 264–319).
> **Pre-existing artifact**: Slice 1 (commit `836320984`) added `azure_data_cosmos_driver::driver::transport::rntbd` (request/response/tokens/status) — **do not modify it in this slice**.

---

## 1. Slice 2 Goal

Lay down the **foundation** for Gateway 2.0 request handling without yet routing any traffic through it. Three deliverables:

1. **Eligibility helper** (R4 / AC5) — pure function `is_operation_supported_by_gateway20(resource_type, operation_type) -> bool`.
2. **Account-name extraction** (R7 / AC7 partial) — parse the global database account name once at client construction, store it as `Option<String>`. **Not yet emitted on RNTBD frames** (deferred to Slice 3).
3. **Constants relocation** (R16 / AC17) — create the driver-side `GATEWAY20_*` family with the wire-header strings, and convert the existing SDK-side `THINCLIENT_PROXY_*` constants into deprecated re-exports of the new driver constants to preserve public API.

This slice **does NOT**:

- Wire eligibility into routing (`resolve_endpoint` is unchanged).
- Wrap any request body in RNTBD.
- Inject any Gateway 2.0 HTTP/2 headers into outgoing requests.
- Touch the response unwrap path.
- Cut over `get_hashed_partition_key_string` (R5 deferred).
- Add `Patch` to the `OperationType` enum (deferred; the helper just doesn't have a Patch arm).

The next slice will do the mode-aware dispatcher work. Keep this slice **mechanical and rollback-safe**.

---

## 2. Files

### Create

- `sdk/cosmos/azure_data_cosmos_driver/src/constants.rs` — new crate-root module mirroring the SDK's pattern (`azure_data_cosmos/src/constants.rs`).
- `sdk/cosmos/azure_data_cosmos_driver/src/driver/transport/gateway20_eligibility.rs` — new module containing the eligibility helper + tests.

### Edit

- `sdk/cosmos/azure_data_cosmos_driver/src/lib.rs` — add `pub mod constants;`.
- `sdk/cosmos/azure_data_cosmos_driver/src/driver/transport/mod.rs` — add `pub(crate) mod gateway20_eligibility;` (alongside the existing `pub(crate) mod rntbd;` from Slice 1).
- `sdk/cosmos/azure_data_cosmos_driver/src/models/account_reference.rs` — add a `pub(crate) fn global_database_account_name(&self) -> Option<String>` method on `AccountEndpoint` (parses the host label) + tests.
- `sdk/cosmos/azure_data_cosmos/src/constants.rs` — replace the existing `THINCLIENT_PROXY_OPERATION_TYPE` / `THINCLIENT_PROXY_RESOURCE_TYPE` constants with `#[deprecated]` re-exports that resolve to the new driver constants. **Do not delete the SDK identifiers** (public API).
- `sdk/cosmos/azure_data_cosmos/Cargo.toml` — verify `azure_data_cosmos_driver` is already a dependency; add it if not (it is — confirm by inspection).

### Do NOT edit

- `driver/transport/rntbd/*.rs` (Slice 1 artifact)
- `driver/pipeline/operation_pipeline.rs` (no routing changes in Slice 2)
- `driver/pipeline/components.rs` (TransportMode unchanged)
- `driver/transport/cosmos_headers.rs` (no header injection in Slice 2)
- `driver/routing/endpoint.rs` (no `uses_gateway20` change)
- `models/effective_partition_key.rs` (R5 deferred)
- `azure_data_cosmos/src/handler/container_connection.rs` (R5 deferred)

---

## 3. R4 — Eligibility helper (in `gateway20_eligibility.rs`)

### API

```rust
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Gateway 2.0 operation eligibility filter.

use crate::models::{OperationType, ResourceType};

/// Returns `true` when the (resource_type, operation_type) pair is eligible
/// to be routed through Gateway 2.0.
///
/// Only `ResourceType::Document` is currently eligible (matches Java's
/// `ThinClientStoreModel`). Stored-procedure execution is explicitly out of
/// scope for Rust SDK GA; every non-Document resource type falls back to
/// standard Gateway via the eligibility-fallback path.
///
/// `OperationType::Patch` is not currently a variant on the Rust enum and is
/// therefore not handled here. When the variant is added in a future slice,
/// this match must be updated.
pub(crate) fn is_operation_supported_by_gateway20(
    resource_type: ResourceType,
    operation_type: OperationType,
) -> bool {
    match resource_type {
        ResourceType::Document => matches!(
            operation_type,
            OperationType::Create
                | OperationType::Read
                | OperationType::Replace
                | OperationType::Upsert
                | OperationType::Delete
                | OperationType::Query
                | OperationType::SqlQuery
                | OperationType::QueryPlan
                | OperationType::ReadFeed
                | OperationType::Batch
        ),
        ResourceType::DatabaseAccount
        | ResourceType::Database
        | ResourceType::DocumentCollection
        | ResourceType::StoredProcedure
        | ResourceType::Trigger
        | ResourceType::UserDefinedFunction
        | ResourceType::PartitionKeyRange
        | ResourceType::Offer => false,
    }
}
```

**Critical**: the inner `match` on `resource_type` MUST be exhaustive (no wildcard arm) so adding a new `ResourceType` variant in the future is a compile-time error that forces a re-evaluation. Same for the inner `matches!` for OperationType — but since `matches!` already enumerates explicit variants, you instead need a follow-up `#[cfg(test)]` exhaustiveness test (see below).

### Tests (AC5 — exhaustive matrix)

In a `#[cfg(test)] mod tests` block at the bottom of the same file:

1. **Helper exhaustiveness** test — iterate every `(ResourceType, OperationType)` Cartesian product (use `[ResourceType::Document, ResourceType::Database, ...]` and `[OperationType::Create, OperationType::Read, ...]` arrays; if a future enum variant is added, the test array must update, which keeps the helper consistent). For each pair, call the helper and assert the expected bool.

   - Eligible: `Document × {Create, Read, Replace, Upsert, Delete, Query, SqlQuery, QueryPlan, ReadFeed, Batch}` → `true`.
   - Ineligible: `Document × {Head, HeadFeed, Execute}` → `false`.
   - Ineligible: every non-Document resource type × every operation → `false`.

2. **Stored-procedure exclusion is explicit** test:
   - `is_operation_supported_by_gateway20(StoredProcedure, Execute) == false`.
   - `is_operation_supported_by_gateway20(Document, Execute) == false`.

3. **Document Bulk note**: `Bulk` is not on the `OperationType` enum (Bulk is SDK-side fan-out per the spec). Don't test for it.

---

## 4. R7 — Account-name extraction (in `account_reference.rs`)

### Add a method on `AccountEndpoint`

The host string today is `"myaccount.documents.azure.com"`, `"localhost"`, `"127.0.0.1"`, etc. Extract the **first label** of the hostname when the hostname has the shape `<label>.documents.azure.<tld>` or similar Cosmos endpoint suffix. For everything else, return `None`.

```rust
impl AccountEndpoint {
    /// Returns the global database account name parsed from the endpoint
    /// hostname's first label, or `None` for emulator/IP/custom-domain hosts.
    ///
    /// Used as the value of the RNTBD `GlobalDatabaseAccountName` (0x00CE)
    /// metadata token on Gateway 2.0 requests. When `None`, Gateway 2.0
    /// requests fall back to standard Gateway for that account (the proxy
    /// requires a parseable account name).
    pub(crate) fn global_database_account_name(&self) -> Option<String> {
        let host = self.host();
        if host.is_empty() {
            return None;
        }
        // Reject IP literals (IPv4 dotted quads always start with a digit;
        // IPv6 contains ':'). url::Host could be used here for stronger
        // validation, but a cheap prefix check is sufficient for Cosmos URLs.
        if host.starts_with(|c: char| c.is_ascii_digit()) || host.contains(':') {
            return None;
        }
        // Require a multi-label hostname (e.g., `myacct.documents.azure.com`).
        let (label, suffix) = host.split_once('.')?;
        if label.is_empty() || suffix.is_empty() {
            return None;
        }
        // Only accept hosts whose suffix contains "documents." — this
        // intentionally excludes `localhost`, custom domains, and bare
        // single-label hostnames. Future expansion to support custom
        // domains will need explicit allow-listing.
        if !suffix.starts_with("documents.") {
            return None;
        }
        Some(label.to_owned())
    }
}
```

### Tests (AC7 partial — extractor only)

Table-driven tests in the existing `#[cfg(test)] mod tests` block of `account_reference.rs`:

| Endpoint URL | Expected `global_database_account_name()` |
| --- | --- |
| `https://myaccount.documents.azure.com/` | `Some("myaccount")` |
| `https://my-account-123.documents.azure.com/` | `Some("my-account-123")` |
| `https://myacct.documents.azure.us/` | `Some("myacct")` (Gov cloud) |
| `https://myacct.documents.azure.cn:443/` | `Some("myacct")` (China cloud) |
| `https://localhost:8081/` | `None` (emulator) |
| `https://127.0.0.1:8081/` | `None` (IPv4) |
| `https://[::1]:8081/` | `None` (IPv6) |
| `https://my.custom.domain/` | `None` (no `documents.` suffix) |
| `https://example.com/` | `None` (no `documents.` suffix) |

A single `#[test]` function with a `for (url, expected) in cases` loop is fine — keep it tight.

### Plumbing

This slice **stops at adding the method**. Do NOT thread the value into any caller. Slice 3 will read it off `AccountEndpoint` at the dispatch site and emit it as the RNTBD token.

---

## 5. R16 — Constants relocation (in `azure_data_cosmos_driver/src/constants.rs`)

### New module

```rust
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// Don't spell-check header names (which should start with 'x-').
// cSpell:disable

//! Driver-level Cosmos DB constants.
//!
//! This module owns the canonical wire-name strings for the Gateway 2.0
//! HTTP/2 outer headers. The wire strings retain the historical
//! `x-ms-thinclient-*` form because the proxy is server-defined; only the
//! Rust identifier follows the `GATEWAY20_*` naming convention.

use azure_core::http::headers::HeaderName;

/// Gateway 2.0 proxy operation-type header (numeric op-type, every
/// Gateway 2.0 request).
pub const GATEWAY20_OPERATION_TYPE: HeaderName =
    HeaderName::from_static("x-ms-thinclient-proxy-operation-type");

/// Gateway 2.0 proxy resource-type header (numeric resource-type, every
/// Gateway 2.0 request).
pub const GATEWAY20_RESOURCE_TYPE: HeaderName =
    HeaderName::from_static("x-ms-thinclient-proxy-resource-type");

/// Effective Partition Key header (point Document operations only).
pub const EFFECTIVE_PARTITION_KEY: HeaderName =
    HeaderName::from_static("x-ms-effective-partition-key");

/// Lower bound of the EPK range (feed / cross-partition operations only).
pub const GATEWAY20_RANGE_MIN: HeaderName =
    HeaderName::from_static("x-ms-thinclient-range-min");

/// Upper bound of the EPK range (feed / cross-partition operations only).
pub const GATEWAY20_RANGE_MAX: HeaderName =
    HeaderName::from_static("x-ms-thinclient-range-max");

/// Account-metadata fetch hint instructing the response to advertise
/// thin-client endpoints.
pub const GATEWAY20_USE_THINCLIENT: HeaderName =
    HeaderName::from_static("x-ms-cosmos-use-thinclient");
```

### Edit `azure_data_cosmos/src/constants.rs`

The existing `cosmos_headers!` macro at line ~150-200 generates `THINCLIENT_PROXY_OPERATION_TYPE` and `THINCLIENT_PROXY_RESOURCE_TYPE`. Two-step change:

1. **Remove** those two entries from the `cosmos_headers!` macro invocation.
2. **Add** below the macro (still inside `azure_data_cosmos::constants`) explicit `#[deprecated]` re-exports so existing callers don't break:

```rust
#[deprecated(
    since = "0.27.0",
    note = "Use `azure_data_cosmos_driver::constants::GATEWAY20_OPERATION_TYPE` instead."
)]
pub const THINCLIENT_PROXY_OPERATION_TYPE: HeaderName =
    azure_data_cosmos_driver::constants::GATEWAY20_OPERATION_TYPE;

#[deprecated(
    since = "0.27.0",
    note = "Use `azure_data_cosmos_driver::constants::GATEWAY20_RESOURCE_TYPE` instead."
)]
pub const THINCLIENT_PROXY_RESOURCE_TYPE: HeaderName =
    azure_data_cosmos_driver::constants::GATEWAY20_RESOURCE_TYPE;
```

(Pick whichever `since` value matches the unreleased crate version; check `azure_data_cosmos/Cargo.toml`'s `version = "..."` and use that.)

3. The `COSMOS_ALLOWED_HEADERS` array (auto-generated by the macro) will lose the two THINCLIENT entries since they no longer come from the macro. **Add them back manually** to the `COSMOS_ALLOWED_HEADERS` slice (or a sibling slice) so logging behavior is unchanged. If the macro emits the slice as `pub const COSMOS_ALLOWED_HEADERS: &[&HeaderName] = &[...];`, you may need to define a separate const slice that concatenates the macro output with the deprecated re-exports — keep this change tight; if you can't find a non-disruptive way, fall back to leaving the two THINCLIENT entries in the macro AND re-export them, accepting the duplication and letting the deprecation lint guide future migration.

### Tests

In `azure_data_cosmos_driver/src/constants.rs`, add a `#[cfg(test)] mod tests` block with smoke tests:

1. Each constant exists and equals its expected wire string (use `.as_str()` or compare against `HeaderName::from_static("...")`).
2. No two constants share the same wire string.

You do NOT need to test the deprecated SDK re-exports beyond verifying the crate still compiles. The compiler's deprecation warnings during `cargo clippy` will trigger if any internal code still references the old names — fix those callsites by updating them to the new driver constants.

---

## 6. Cargo.toml

`azure_data_cosmos` already depends on `azure_data_cosmos_driver` (verify by checking `sdk/cosmos/azure_data_cosmos/Cargo.toml`). If by some chance it doesn't, ADD it as a workspace dependency under `[dependencies]`:

```toml
azure_data_cosmos_driver = { workspace = true }
```

Don't add new third-party dependencies. The `url` crate is already pulled in transitively.

---

## 7. Validation order

After all edits are complete, run **in this exact order** and fix any issues:

```bash
cargo fmt -p azure_data_cosmos_driver
cargo fmt -p azure_data_cosmos
cargo build -p azure_data_cosmos_driver --all-features
cargo build -p azure_data_cosmos --all-features
cargo clippy -p azure_data_cosmos_driver --all-features --all-targets -- -D warnings
cargo clippy -p azure_data_cosmos --all-features --all-targets -- -D warnings
cargo doc -p azure_data_cosmos_driver --all-features --no-deps
cargo doc -p azure_data_cosmos --all-features --no-deps
cargo test -p azure_data_cosmos_driver --all-features
cargo test -p azure_data_cosmos --all-features
```

Expected: 0 warnings, 0 errors, all tests pass. New tests added in this slice should appear and pass.

If clippy flags `deprecated` usage of the old SDK constants from inside the SDK or driver itself, **fix the internal callers** to use the new driver constants — those callers have to migrate as part of this slice (the deprecation is for *external* consumers).

---

## 8. Conventions & guardrails (copy from AGENTS.md)

- All new `.rs` files start with the 2-line copyright header — **no preceding blank line**.
- Use `pub(crate)` visibility for the eligibility helper and the new account-name accessor. Use `pub` only for things explicitly meant for external consumers (the constants under `pub mod constants` are `pub`).
- Tests go in a `#[cfg(test)] mod tests` block at the bottom of the same module, importing from `super::*`.
- Test function names should NOT begin with `test_` — use descriptive names (e.g., `document_create_is_eligible_for_gateway20`).
- Use idiomatic `match` exhaustiveness for both enums; do NOT use wildcard `_` arms in the helper.
- Use `azure_core::Result` (NOT `std::result::Result` or `Result<_, Box<dyn Error>>`) only if a function returns a `Result`. The eligibility helper does NOT return a Result; the account-name extractor returns `Option<String>`.
- No `unwrap()` / `expect()` / `panic!` in non-test code.
- Doc-comments (`///`) on every `pub(crate)` and `pub` item with a one-sentence summary + a blank line + details.

---

## 9. DO-NOT-DO list (lessons from rubber-duck critique)

- **Do NOT modify `resolve_endpoint` or any routing logic.** The eligibility helper is added but NOT yet called from the pipeline. This is intentional — Slice 3 will wire it in once mode-aware dispatch lands.
- **Do NOT wrap any HTTP request body in RNTBD.** Slice 1 added the codecs; Slice 2 does not invoke them.
- **Do NOT inject any `x-ms-thinclient-*` headers into outgoing requests.** The constants exist; nothing emits them.
- **Do NOT touch the response unwrap path.** Per rubber-duck, response handling needs a mode-specific dispatcher; defer entirely to Slice 3.
- **Do NOT cut over `get_hashed_partition_key_string`.** Leave `azure_data_cosmos::handler::container_connection` alone.
- **Do NOT add `Patch` to the `OperationType` enum** in this slice — it's a separate decision (and would propagate through 100+ usages). Document the gap in the helper's doc comment.
- **Do NOT delete the SDK's `THINCLIENT_PROXY_*` constants.** Public API; deprecate + re-export only.
- **Do NOT change `is_gateway20_allowed` → `gateway20_disabled`.** That's R15 / Phase 5 / Slice 5.
- **Do NOT add or remove crate dependencies.** Use only what's already in `Cargo.toml`.
- **Do NOT touch `cosmos_headers.rs` or the capabilities bitmask.** Slice 1 set it to "9".

---

## 10. Acceptance check

Before declaring done, confirm:

- [ ] `is_operation_supported_by_gateway20` returns `true` for the 10 listed Document ops, `false` for everything else (covered by exhaustive test matrix).
- [ ] `AccountEndpoint::global_database_account_name()` returns `Some("myaccount")` for the canonical Cosmos URL, `None` for emulator and custom domains.
- [ ] All six new `GATEWAY20_*` / `EFFECTIVE_PARTITION_KEY` constants exist in `azure_data_cosmos_driver::constants` with the exact wire strings documented in §5.
- [ ] The two SDK constants `THINCLIENT_PROXY_OPERATION_TYPE` / `THINCLIENT_PROXY_RESOURCE_TYPE` still resolve at the same fully-qualified path, but emit a deprecation warning when used.
- [ ] No file under `driver/transport/rntbd/`, `driver/pipeline/`, `driver/routing/`, or `driver/transport/cosmos_headers.rs` is touched.
- [ ] `cargo fmt`, `cargo clippy --all-features --all-targets -- -D warnings`, `cargo doc`, and `cargo test --all-features` all pass cleanly for both `azure_data_cosmos` and `azure_data_cosmos_driver`.

When done, **report back**: a one-paragraph summary of what changed, the test count delta, and a list of files touched. Do NOT commit — the orchestrator will review the diff and dispatch a code-review agent before any commit.

# Slice 3a Brief — Routing Eligibility + endpoint_key fix + Account-name plumb

> Stack base: `tvaron3/gateway-2.0-impl` at HEAD (`802e4795b`, Slice 2 committed).
> Read this brief end-to-end. Do not start before reading the **Surface** and **DO NOT** sections. Sub-slicing rationale lives in `.coding-harness/slice-3-scoping.md` §6 + §7.

---

## Goal

Three changes to the operation pipeline, landing as one commit:

1. **Eligibility gate**: `resolve_endpoint` must select `TransportMode::Gateway20` only when the operation is supported by Gateway 2.0 (per `is_operation_supported_by_gateway20` from Slice 2) AND the account name is parseable from the endpoint host.
2. **`endpoint_key` correctness**: when `TransportMode::Gateway20` is selected, the connection-pool key passed downstream MUST be derived from the gateway20 URL's authority — NOT the gateway1 URL. Today `routing.endpoint.endpoint_key()` returns a value cached from the gateway1 URL, so Gateway 2.0 traffic shards by the wrong key.
3. **Account-name plumb**: parse `global_database_account_name()` once at the top of `execute_operation_pipeline` and (a) use its `Some`/`None` to decide eligibility, (b) carry the parsed value forward in a way that Slice 3b/c can consume.

After this slice: no observable wire-level change (Gateway 2.0 path is still broken end-to-end because wrap/unwrap don't exist), but routing decisions are correct and ready for 3b/c to land.

---

## Files to edit

1. `sdk/cosmos/azure_data_cosmos_driver/src/driver/pipeline/components.rs` — extend `RoutingDecision` with an `endpoint_key: EndpointKey` field. Update `Display` impl if needed (probably not — it only prints region + URL).
2. `sdk/cosmos/azure_data_cosmos_driver/src/driver/pipeline/operation_pipeline.rs`
   - Add `account_name: Option<String>` derivation at the top of `execute_operation_pipeline` (line ~63 area, immediately after `let mut diagnostics = diagnostics;`). Compute via `account_endpoint.global_database_account_name()`.
   - Pass `account_name.is_some()` (as a `bool`) into `resolve_endpoint`.
   - Inside `resolve_endpoint`, compute the eligibility gate (see "Eligibility logic" below) and populate `endpoint_key` from `selected.selected_url(use_gateway20)`.
   - At line 202, replace `endpoint_key: routing.endpoint.endpoint_key()` with `endpoint_key: routing.endpoint_key.clone()`.
   - Update the existing `resolve_endpoint` test cases (`resolve_endpoint_prefers_gateway20_for_dataplane_reads`, `resolve_endpoint_skips_unavailable_region_when_gateway20_is_present`) to pass the new `account_name_present` arg as `true`.
   - Add 3 new tests (see "Tests" below).

That's it. **Do not** modify `endpoint.rs`. **Do not** introduce a new module. **Do not** touch the wrap/unwrap path (Slice 3b/c).

---

## Surface (already verified by orchestrator)

### `resolve_endpoint` today (operation_pipeline.rs:335)

```rust
fn resolve_endpoint(
    operation: &CosmosOperation,
    retry_state: &OperationRetryState,
    location: &LocationSnapshot,
    prefer_gateway20: bool,
    endpoint_unavailability_ttl: Duration,
) -> RoutingDecision {
    // ... selects `selected: CosmosEndpoint` ...
    let use_gateway20 = selected.uses_gateway20(prefer_gateway20);
    let transport_mode = if use_gateway20 {
        TransportMode::Gateway20
    } else {
        TransportMode::Gateway
    };

    RoutingDecision {
        selected_url: selected.selected_url(use_gateway20).clone(),
        endpoint: selected,
        transport_mode,
    }
}
```

### `RoutingDecision` today (components.rs:44)

```rust
pub(crate) struct RoutingDecision {
    pub endpoint: CosmosEndpoint,
    pub selected_url: Url,
    pub transport_mode: TransportMode,
}
```

### Caller of resolve_endpoint (operation_pipeline.rs:115)

```rust
let routing = resolve_endpoint(
    operation,
    &retry_state,
    &location,
    pipeline_type == PipelineType::DataPlane,
    location_state_store.endpoint_unavailability_ttl(),
);
```

### endpoint_key consumer (operation_pipeline.rs:202)

```rust
endpoint_key: routing.endpoint.endpoint_key(),  // ← bug: cached from gateway1 URL
```

### EndpointKey constructor

`EndpointKey::try_from(&url)` returns `Result<EndpointKey, _>` (look at `sharded_transport.rs:239`). Existing constructors in `endpoint.rs` use `.expect("CosmosEndpoint URL must have a valid host and port")`. Mirror that pattern.

### Slice 2 helper (gateway20_eligibility.rs:20)

```rust
pub(crate) fn is_operation_supported_by_gateway20(
    resource_type: ResourceType,
    operation_type: OperationType,
) -> bool
```

### Slice 2 account-name helper (`AccountEndpoint::global_database_account_name`)

Returns `Option<String>`. Located on `AccountEndpoint`. The arg `account_endpoint: &AccountEndpoint` is already in scope inside `execute_operation_pipeline` (line 52).

---

## Eligibility logic

Inside `resolve_endpoint`, after `let selected = ...;`:

```rust
let use_gateway20 = selected.uses_gateway20(prefer_gateway20)
    && account_name_present
    && is_operation_supported_by_gateway20(
        operation.resource_type(),
        operation.operation_type(),
    );
```

Note the order: keep `selected.uses_gateway20(prefer_gateway20)` first because it's the cheapest gate, then `account_name_present` (cheap bool), then the eligibility check (a match statement).

You will need to add `is_operation_supported_by_gateway20` to the imports at the top of `operation_pipeline.rs`. It's reachable as `crate::driver::transport::gateway20_eligibility::is_operation_supported_by_gateway20`. Verify by grepping for the `pub(crate)` exports of the module.

---

## endpoint_key derivation

Inside `resolve_endpoint`, after `use_gateway20` is computed and BEFORE the `RoutingDecision { ... }` literal:

```rust
let selected_url = selected.selected_url(use_gateway20).clone();
let endpoint_key = EndpointKey::try_from(&selected_url)
    .expect("selected URL must have a valid host and port");
```

Then in the `RoutingDecision` literal, set `endpoint_key`. Drop the duplicate `selected.selected_url(use_gateway20).clone()` call — reuse the local `selected_url`.

You will need to add `EndpointKey` to the imports at the top of `operation_pipeline.rs`. It's already exported as `crate::driver::transport::EndpointKey` (`transport/mod.rs:31`).

**Do NOT** delete or modify `CosmosEndpoint::endpoint_key()` — it's still used elsewhere (e.g., in tests at `transport_pipeline.rs:920`). Just stop using it from the operation pipeline.

---

## Account-name derivation

At the top of `execute_operation_pipeline`, immediately after the existing `let mut diagnostics = diagnostics;` (line ~63):

```rust
let account_name: Option<String> = account_endpoint.global_database_account_name();
```

Reasoning: `global_database_account_name()` parses the endpoint host. Cheap, but doing it once per operation (rather than once per attempt) avoids redundant work in the retry loop. The parsed value is unused by 3a beyond its `is_some()` discriminator, but later slices (3b/c) will need the actual `Option<String>` value to build the wrap function's `account_name` argument and `x-ms-thinclient-account-name` header.

Pass `account_name.is_some()` into `resolve_endpoint` as a new `bool` arg named `account_name_present`. The full `Option<String>` stays in `execute_operation_pipeline` for 3b/c to consume.

**Visibility / dead-code**: the `account_name` binding will be unused in 3a (only `account_name.is_some()` is read). Add `#[allow(dead_code)]` on the binding **or** suppress with `let _account_name = ...;` until 3b/c consumes it. Either is acceptable; pick whichever is conventional in this crate. Do NOT remove the binding — keeping it documents the intent and makes 3b/c a smaller diff.

Actually: don't `let _ = ...` and don't bind it. Just inline `account_endpoint.global_database_account_name().is_some()` at the call site for 3a, and let 3b/c add the binding when it's actually used. That keeps 3a's diff minimal and avoids a dead `let`.

---

## resolve_endpoint signature change

```rust
fn resolve_endpoint(
    operation: &CosmosOperation,
    retry_state: &OperationRetryState,
    location: &LocationSnapshot,
    prefer_gateway20: bool,
    account_name_present: bool,    // NEW
    endpoint_unavailability_ttl: Duration,
) -> RoutingDecision
```

Caller updates: line 115 (production) and lines 871, 921, 969, 1026, 1039, 1182, 1240 (tests). All test callers should pass `true` for `account_name_present` UNLESS the test specifically targets the unparseable-name fallback (the new test).

---

## Tests

Add these in the existing `mod tests` of `operation_pipeline.rs` (find the section near line 839 / 1153 / 1198 — there's already a section for `resolve_endpoint` tests).

### Test 1: ineligible operation falls back to Gateway

```rust
#[test]
fn resolve_endpoint_falls_back_to_gateway_when_op_ineligible_for_gateway20() {
    // Build an account_endpoint state with at least one regional endpoint
    // that has a gateway20_url. Use an operation that is NOT supported
    // by Gateway 2.0 (consult is_operation_supported_by_gateway20's tests
    // in gateway20_eligibility.rs for an ineligible (resource, op) pair).
    //
    // Call resolve_endpoint with prefer_gateway20=true, account_name_present=true.
    //
    // Assert: routing.transport_mode == TransportMode::Gateway (NOT Gateway20),
    // and routing.selected_url == the gateway1 URL of the selected endpoint.
}
```

### Test 2: unparseable account name falls back to Gateway

```rust
#[test]
fn resolve_endpoint_falls_back_to_gateway_when_account_name_unparseable() {
    // Build the same setup as Test 1, but with an ELIGIBLE operation
    // (e.g., Document Read).
    //
    // Call resolve_endpoint with prefer_gateway20=true, account_name_present=false.
    //
    // Assert: routing.transport_mode == TransportMode::Gateway (NOT Gateway20),
    // and routing.selected_url == the gateway1 URL of the selected endpoint.
}
```

### Test 3: gateway20 endpoint_key uses gateway20 authority

```rust
#[test]
fn resolve_endpoint_uses_gateway20_authority_for_endpoint_key() {
    // Build an account_endpoint state with at least one regional endpoint
    // whose gateway1 host differs from its gateway20 host (e.g.,
    // "central.documents.azure.com" vs "central.thinclient.azure.com").
    //
    // Call resolve_endpoint with an ELIGIBLE operation,
    // prefer_gateway20=true, account_name_present=true.
    //
    // Assert: routing.transport_mode == TransportMode::Gateway20,
    // routing.selected_url's host matches the gateway20 host,
    // AND routing.endpoint_key (when converted back via Display or
    // exposed via a test-accessor on EndpointKey) reflects the gateway20
    // authority, NOT the gateway1 authority.
}
```

For Test 3: `EndpointKey` wraps `Arc<str>` (`sharded_transport.rs:237`). Look for an existing way to extract the inner string for assertion (Display, AsRef, or `.0` if visible from the test module via `pub(crate)`). If no such accessor exists in test scope, build a reference key via `EndpointKey::try_from(&gateway20_url).unwrap()` and assert equality.

### Update existing tests

The two existing gateway20 tests pass `account_name_present=true`:
- `resolve_endpoint_prefers_gateway20_for_dataplane_reads` (line 1153)
- `resolve_endpoint_skips_unavailable_region_when_gateway20_is_present` (line 1198)

The four non-gateway20 tests pass `account_name_present=true` as well (it's irrelevant to their assertions but the signature requires it):
- `resolve_endpoint_uses_write_region_for_single_write_session_retry` (line 839)
- `resolve_endpoint_falls_back_to_default_when_all_unavailable` (line 882)
- `resolve_endpoint_ignores_write_forbidden_for_reads` (line 932)
- (and the unnamed one near line 1026/1039 if it exists)

---

## Imports to add

In `operation_pipeline.rs`:

```rust
use crate::driver::transport::{
    gateway20_eligibility::is_operation_supported_by_gateway20,
    EndpointKey,
    // ... existing items
};
```

Verify the path of `is_operation_supported_by_gateway20` — it may need a re-export from `transport/mod.rs` if it's not already pub-exported. If it's not, add the re-export.

---

## Validation order (per AGENTS.md)

Run each of these from repo root, fix any failures before moving to the next:

1. `cargo fmt -p azure_data_cosmos_driver` (formatting first per AGENTS.md)
2. `cargo build -p azure_data_cosmos_driver --all-features`
3. `cargo clippy -p azure_data_cosmos_driver --all-features --all-targets -- -D warnings`
4. `cargo test -p azure_data_cosmos_driver --all-features`
5. `cargo doc -p azure_data_cosmos_driver --no-deps`

The same five for `azure_data_cosmos` (sanity check that nothing leaked through).

Tests should be 721 (current) + 3 (new) = 724 total. If a count differs, investigate.

---

## DO NOT

- Do NOT rename or remove `CosmosEndpoint::endpoint_key()`. Other consumers still use it.
- Do NOT modify `endpoint.rs` (the bug is at the consumer, not the source).
- Do NOT introduce wrap/unwrap helpers, RNTBD encoding, or `gateway20_dispatch` modules. That's Slice 3b/c.
- Do NOT modify `transport_pipeline.rs`, `request_signing.rs`, `adaptive_transport.rs`, or anything in `rntbd/`.
- Do NOT add a new public API; everything is `pub(crate)` or private.
- Do NOT touch `azure_data_cosmos` (the SDK side); 3a is driver-only.
- Do NOT mark anything `#[deprecated]` in this slice.
- Do NOT update the GATEWAY_20_SPEC.md or any spec docs in this slice.

---

## Acceptance

This slice is complete when:

- All five validation commands above pass cleanly with `-D warnings`.
- The three new tests exist and pass.
- The four existing tests still pass with the new arg.
- `git diff --stat` shows changes ONLY in: `components.rs`, `operation_pipeline.rs`, and (if the eligibility helper needs re-exporting) `driver/transport/mod.rs`. No other files.
- Test count increased from 721 to 724 in the driver crate.

---

## Open question to surface back

If you discover that `is_operation_supported_by_gateway20` is NOT pub-re-exported from `transport/mod.rs` and you need to add the re-export, that's fine — do it and call it out in your summary. If you discover a tighter visibility (e.g., it's `pub(super)` only), mention it and we'll discuss.

If you discover a hidden assumption that breaks the plan (e.g., `account_endpoint.global_database_account_name()` is more expensive than expected and SHOULD be cached, or `EndpointKey::try_from` is fallible in a non-trivial way), STOP and surface it before continuing.

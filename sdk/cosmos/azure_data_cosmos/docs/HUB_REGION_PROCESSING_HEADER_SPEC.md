# Hub-Region Processing-Only Header Spec for `azure_data_cosmos`

**Status:** Draft / Iterating
**Date:** 2026-04-29
**Authors:** (team)
**Crate:** `azure_data_cosmos`
**Tracks:** [#4303](https://github.com/Azure/azure-sdk-for-rust/issues/4303)
**Mirrors:** [Azure/azure-cosmos-dotnet-v3#5447](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5447)

---

## Table of Contents

1. [Goals & Motivation](#1-goals--motivation)
2. [Architectural Overview](#2-architectural-overview)
3. [Behavior Specification](#3-behavior-specification)
4. [Code Changes](#4-code-changes)
5. [Side Effects & Risk](#5-side-effects--risk)
6. [Alternatives Considered](#6-alternatives-considered)
7. [Testing Strategy](#7-testing-strategy)
8. [Open Questions & Future Work](#8-open-questions--future-work)

---

## 1. Goals & Motivation

### Problem Statement

On a **single-master** Cosmos DB account using **Session** consistency, a region failover or
failback opens a catch-up window during which a satellite read region may not yet have applied
the latest writes. A read against the lagging region returns `404` with sub-status
`1002` (`READ_SESSION_NOT_AVAILABLE`). The current Rust `ClientRetryPolicy` retries the request
on the next preferred region but cannot ask the backend to *guarantee* it serves the request
from the hub (write) region — so the retry is liable to land on another lagging satellite and
fail again, eventually surfacing a hard error to the caller.

The .NET SDK ([azure-cosmos-dotnet-v3#5447](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5447))
shipped a fix: when the second-or-later 404/1002 fires inside a single-master operation, the SDK
attaches a new request header — `x-ms-cosmos-hub-region-processing-only: true` — to that retry
and every subsequent retry within the same operation. The backend honors the header by serving
the request from the hub (write) region, which is guaranteed to be current. Issue
[#4303](https://github.com/Azure/azure-sdk-for-rust/issues/4303) tracks bringing the Rust SDK to
parity.

### Goals

1. **Functional parity with .NET.** Mirror the observable behavior of
   [PR #5447](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5447) exactly: same trigger,
   same gate (single-master only), same persistence semantics (latched once, applied on every
   subsequent retry until the operation completes).
2. **Surgical, additive change.** No public-API change, no error-type change, no new options
   surfaced to callers. The feature is fully automatic.
3. **Forward-compatible on the wire.** Older backend builds that do not yet honor the header
   ignore it; behavior degrades gracefully to today's baseline.
4. **Contained blast radius.** All logic lives inside `ClientRetryPolicy`; existing retry
   budgets, delays, and status-code routing are unchanged.

### Non-Goals (This Spec)

- The 403/3 (`NotWriteRegion`) **skip-set rotation** flow described in the issue text. The
  .NET PR did not implement it; mirroring .NET is the explicit constraint of this work item.
  See [§6 Alternatives Considered](#6-alternatives-considered) for why this is intentional.
- A user-facing toggle to disable the header. The feature mirrors .NET's "always on for
  single-master" contract.
- Backend rollout coordination. The header is forward-compatible; older backends ignore it.
- Multi-master account behavior. Multi-master accounts never emit the header.

### Primary Target

Single-master Session-consistency reads during a region failover or failback catch-up window.
Specifically: any `read_item` / `query` / point-read on a single-master account where a
satellite region briefly lags behind the hub.

---

## 2. Architectural Overview

The Rust pipeline is shaped almost identically to .NET. The two relevant hooks already exist on
`ClientRetryPolicy`:

| .NET (`ClientRetryPolicy.cs`)         | Rust (`client_retry_policy.rs`)                      | Role |
|---------------------------------------|------------------------------------------------------|------|
| `OnBeforeSendRequest`                 | `before_send_request` (line ~150)                    | Sole pre-flight mutation hook for outbound headers / endpoint routing. |
| `ShouldRetryInternalAsync` — 404/1002 | `should_retry_on_session_not_available` (line ~267)  | Decides whether to retry a session-not-available failure and how many regions remain. |

The change introduces a single boolean field on `ClientRetryPolicy` — the *latch* — that is set
inside `should_retry_on_session_not_available` and consumed inside `before_send_request`. The
retry handler (`BackOffRetryHandler`) re-enters `before_send_request` for every attempt, which
is the property that lets the latched flag persist onto subsequent retries automatically.

```text
┌──────────────────────────────────────────────────────────────────────────────┐
│                       BackOffRetryHandler::send (loop)                        │
│                                                                              │
│  ┌────────────────────────────────────────┐                                  │
│  │  ClientRetryPolicy::before_send_request │   ◀── runs on EVERY attempt    │
│  │  ──────────────────────────────────────  │                                  │
│  │  if self.add_hub_region_processing_only_│                                  │
│  │      header { request.headers.insert(.) }│                                  │
│  └────────────────────────────────────────┘                                  │
│                       │                                                      │
│                       ▼                                                      │
│            (HTTP send → response)                                            │
│                       │                                                      │
│                       ▼                                                      │
│  ┌────────────────────────────────────────┐                                  │
│  │  ClientRetryPolicy::should_retry_on_*   │                                  │
│  │  ──────────────────────────────────────  │                                  │
│  │  on 404/1002 + !multi_master + count≥2: │                                  │
│  │     self.add_hub_region_..._header=true │   ◀── latches the flag          │
│  └────────────────────────────────────────┘                                  │
└──────────────────────────────────────────────────────────────────────────────┘
```

`ClientRetryPolicy` is constructed once per top-level operation by
`retry_handler::retry_policy_for_request`. The latch's lifetime is therefore naturally bounded
to a single user-visible operation — see [§5 Side Effects & Risk](#5-side-effects--risk) for the
verification step required in implementation.

---

## 3. Behavior Specification

### 3.1 Trigger

The latch flips to `true` when **all** of the following hold inside a single
`ClientRetryPolicy` instance:

1. `should_retry_on_session_not_available` is invoked (i.e., the response was `404` with
   sub-status `1002 / READ_SESSION_NOT_AVAILABLE`).
2. `enable_endpoint_discovery` is `true`. (When discovery is disabled, the policy returns
   `DoNotRetry` and the latch is not touched.)
3. `!can_use_multiple_write_locations` — the account is single-master.
4. `self.session_token_retry_count >= 2` after the existing increment at the top of
   `should_retry_on_session_not_available`. This is the post-increment equivalent of .NET's
   pre-increment check `sessionTokenRetryCount >= 1` (i.e., "second-or-later 404/1002 in
   this operation").

Once latched, the flag is **never reset** within the policy. .NET uses a single `volatile bool`
field with the same semantics.

### 3.2 Outbound effect

On every subsequent attempt within the same operation, `before_send_request` checks the latch.
When set, it adds:

```text
x-ms-cosmos-hub-region-processing-only: true
```

to the outgoing `CosmosRequest`. The header rides along on retries triggered by **any** status
code — 404/1002, 503, 410/1022, connection failures — until the operation terminates.

### 3.3 What stays unchanged

- Retry budgets, delays, and status-code routing in `should_retry_on_*`.
- Client-side region selection in `RetryContext`. The header is a *backend* routing hint;
  Rust continues to drive its own region rotation through the existing failover ladder.
- Session-token propagation, partition-key handling, consistency-level handling.
- The existing inline `request.headers.insert(constants::ALLOW_TENTATIVE_WRITES, "true")` on
  `before_send_request` — that line stays exactly where it is and is the precedent the new
  header follows.

### 3.4 Worked example

Single-master account, Session consistency, preferred regions `[East US 2, Central US]`,
write region `East US 2`.

| Attempt | Targeted region | Server response | Header on **this** attempt | Notes |
|---|---|---|---|---|
| 1 | East US 2 | `404` / `1002` | _absent_ | First 404/1002. Counter goes 0→1 (first retry). Latch unchanged. |
| 2 | Central US | `404` / `1002` | _absent_ | Counter goes 1→2 (second retry). Latch flips ON inside this call. The retry decision still routes to the next region per existing logic; the latch will take effect on the *next* attempt. |
| 3 | East US 2 (next location) | `503` | **`true`** | `before_send_request` reads the latch and emits the header. Backend honors hub-region-only processing. |
| 4 | East US 2 | `200` ✓ | **`true`** | Header still present (latch never resets). Operation completes. |

This matches the .NET reference implementation exactly.

---

## 4. Code Changes

### 4.1 `sdk/cosmos/azure_data_cosmos/src/constants.rs`

Add **one** entry to the existing `cosmos_headers!` macro invocation, adjacent to
`ALLOW_TENTATIVE_WRITES`:

```rust
cosmos_headers! {
    // ... existing entries ...
    ALLOW_TENTATIVE_WRITES => "x-ms-cosmos-allow-tentative-writes",
    SHOULD_PROCESS_ONLY_IN_HUB_REGION => "x-ms-cosmos-hub-region-processing-only",
    // ... existing entries ...
}
```

The macro auto-defines the constant via `HeaderName::from_static` *and* auto-registers it in
`COSMOS_ALLOWED_HEADERS` (used by the default logging policy). Do **not** define the constant
outside the macro and do **not** hand-edit `COSMOS_ALLOWED_HEADERS`.

### 4.2 `sdk/cosmos/azure_data_cosmos/src/retry_policies/client_retry_policy.rs`

**(a) Add the latch field.** Insert into the `ClientRetryPolicy` struct alongside the other
retry-state counters:

```rust
/// Latched flag indicating that subsequent attempts in this operation must carry
/// the `x-ms-cosmos-hub-region-processing-only` header. Set in
/// `should_retry_on_session_not_available` for single-master accounts after the
/// second 404/1002 within the operation; never reset within the policy. Mirrors
/// the `addHubRegionProcessingOnlyHeader` field added in .NET PR #5447.
add_hub_region_processing_only_header: bool,
```

Initialize to `false` in `ClientRetryPolicy::new`.

**(b) Apply the header.** In `before_send_request`, after the existing routing logic and
**immediately adjacent** to the existing `ALLOW_TENTATIVE_WRITES` line (currently around
line 168), add:

```rust
if self.add_hub_region_processing_only_header {
    request
        .headers
        .insert(constants::SHOULD_PROCESS_ONLY_IN_HUB_REGION, "true");
}
```

The `"true"` string-literal form (lowercase, no allocation) matches the precedent on the
adjacent `ALLOW_TENTATIVE_WRITES` line. **Do not** use `"True"` (.NET style) or
`"true".to_string()`.

**(c) Set the latch.** In `should_retry_on_session_not_available`, inside the existing
`!can_use_multiple_write_locations` branch, after the counter increment and before the
return:

```rust
if !self.add_hub_region_processing_only_header
    && self.session_token_retry_count >= 2
{
    self.add_hub_region_processing_only_header = true;
    debug!(
        "latched x-ms-cosmos-hub-region-processing-only for single-master \
         session retry"
    );
}
```

Extend the existing `use tracing::error;` import to `use tracing::{debug, error};`.

**(d) Doc-comment hygiene (opportunistic).** The surrounding doc comments on
`should_retry_on_session_not_available` and `should_retry_on_http_status` cite "404.1022" for
`READ_SESSION_NOT_AVAILABLE`; the actual sub-status is `1002`
(`PARTITION_KEY_RANGE_GONE`). Correct in the same edit.

### 4.3 `sdk/cosmos/azure_data_cosmos/CHANGELOG.md`

The most recent shipped block is `## 0.31.0 (2026-02-25)`. Add a new in-flight block at the
top under `# Release History`:

```markdown
## 0.32.0 (Unreleased)

### Features Added

- Added `x-ms-cosmos-hub-region-processing-only` request header on retries after
  `404`/`1002` for single-master accounts, opting the retry into hub-region processing
  during failover/failback windows. ([#NNNN](https://github.com/Azure/azure-sdk-for-rust/pull/NNNN))
```

The `(#NNNN)` suffix is mandatory; replace with the PR number once opened. Confirm with
maintainers that 0.32.0 is the right target (or amend an open 0.31.x block if one exists).

### 4.4 No changes required

- No `Cargo.toml` change.
- No public API, options, or builder change.
- No change to `request_options.additional_headers`, `COSMOS_ALLOWED_HEADERS` (the macro
  handles it), or `cosmos_pipeline.rs`.
- No integration-test fixture change.

---

## 5. Side Effects & Risk

| ID     | Severity        | Description | Mitigation |
|--------|-----------------|-------------|------------|
| SE-001 | 🟡 Potential    | Backend rollout asymmetry — older regions ignore the header; behavior reverts to today's baseline (no recovery during failback). | Forward-compatible by design. Document graceful degradation in CHANGELOG. No code mitigation required. |
| SE-002 | 🟡 Potential    | Future backend mis-routing if header were honored on multi-master accounts. | Latch is set ONLY inside the existing `!can_use_multiple_write_locations` branch. Asserted in unit test (see [§7.1](#71-unit-tests) AC-4). |
| SE-003 | 🟡 Potential    | Latch correctness depends on `ClientRetryPolicy` being constructed fresh per top-level operation. If the policy is ever pooled or reused across operations, the latch leaks and the header attaches to unrelated requests. | Phase 3 implementer must verify in `BackOffRetryHandler::retry_policy_for_request`. Current evidence (`retry_handler.rs`) says yes. Add a unit test asserting the latch survives intervening 503/410 retries within an operation. If the assumption is ever broken, add an explicit reset hook at the operation boundary. |
| SE-004 | 🟢 Minor        | Spec deviation: the issue text describes a 403/3 "skip-set rotation" flow. The .NET PR did not implement it; we mirror .NET. | Documented in [§6](#6-alternatives-considered) (ALT-4) and [§8](#8-open-questions--future-work) (OQ-1). Existing `should_retry_on_endpoint_failure` already rotates on 403/3 and the latched header rides along. |
| SE-005 | 🟢 Minor        | Pre-existing inline doc-comment typo `404.1022 → 404.1002`. | Fixed opportunistically in §4.2(d). |

### Cross-cutting concerns touched

- **Retry logic & policies** — direct change site; additive only.
- **Telemetry / tracing** — one new `debug!` event when the latch flips. Low cost, high
  diagnostic value during failover events.
- **Wire format** — one additive forward-compatible outbound header. No request body or
  response handling change.
- **Consistency level handling** — 404/1002 is a Session-consistency symptom; backend
  honors the header for consistency `< Strong`. We trigger purely on response code, matching
  .NET — no client-side consistency-level branching.
- **Cross-region failover & preferred locations** — touched **by association**. The header
  is a backend routing hint, not a client-side region change. Client-side region selection in
  `ClientRetryPolicy` is unchanged.

### Cross-cutting concerns NOT touched

Authentication / token refresh, connection pooling, session token propagation, partition key
handling, configuration / initialization, async execution / cancellation,
serialization / response parsing.

---

## 6. Alternatives Considered

### ALT-1 — Reuse the existing `RetryContext.route_to_hub` field

**Verdict:** Rejected.
The existing `route_to_hub` field on `RetryContext` controls *client-side* routing (via
`request_context.route_to_location_endpoint(hub_uri)`), not header emission. Conflating the
two would force every latched retry to be sent directly to the local hub URI — which is **not**
what .NET does. .NET keeps the existing client-side failover ladder and lets the *backend*
re-route via the header. Adding a separate boolean keeps the two concerns orthogonal and
preserves .NET parity.

### ALT-2 — Set the header directly in `should_retry_on_session_not_available`, no latch

**Verdict:** Rejected.
A single-attempt header (no latch) violates the worked example in [§3.4](#34-worked-example)
and the .NET test expectations: the header must persist onto subsequent non-404/1002 retries
(e.g., 503) so the failback-window scenario is fully covered.

### ALT-3 — Plumb the latch through `RequestContext` (per-request) instead of `ClientRetryPolicy`

**Verdict:** Rejected.
`RequestContext` is reset per attempt; the latch state would have to be re-derived on every
retry, requiring the policy to inspect history each time. State on the policy mirrors .NET
exactly and matches Rust's lifetime model — `ClientRetryPolicy` is per-operation, giving the
latch a natural and bounded scope at no extra plumbing cost.

### ALT-4 — Implement the full issue-text spec, including the 403/3 NotWriteRegion skip-set rotation

**Verdict:** Rejected for this work item.
The user explicitly directed mirroring .NET PR #5447, which did **not** implement the
skip-set rotation. The existing `should_retry_on_endpoint_failure` already rotates on 403/3,
and the latched header rides along on the rotated retry — producing observably equivalent
behavior to the more elaborate flow described in the issue. If a future work item proves a
dedicated rotation is required, file a follow-up. See OQ-1 in [§8](#8-open-questions--future-work).

---

## 7. Testing Strategy

### 7.1 Unit Tests

Add new `#[tokio::test] async fn` entries inside the existing
`#[cfg(test)] mod tests` block in `client_retry_policy.rs`, immediately following
`test_should_retry_session_not_available_single_write` (~line 1037). Use the local
`test_*` naming prefix to match the file's existing convention (the global copilot
guidance to omit `test_` does **not** apply here — every test in this file uses the prefix).

| ID    | Name (suggested)                                                | Asserts |
|-------|-----------------------------------------------------------------|---------|
| AC-1  | `test_session_not_available_no_header_on_first_retry`           | Single-master, first 404/1002 → `before_send_request` does NOT emit the header. |
| AC-2  | `test_session_not_available_emits_header_on_second_retry`       | Single-master, second 404/1002 → next `before_send_request` DOES emit `x-ms-cosmos-hub-region-processing-only: true`. |
| AC-3  | `test_hub_region_header_persists_across_503`                    | After AC-2 latch, drive a 503 retry → `before_send_request` STILL emits the header. |
| AC-4  | `test_session_not_available_no_header_for_multi_master`         | Multi-master account, any sequence of 404/1002 retries → header NEVER emitted. |
| AC-5  | `test_session_not_available_no_header_when_discovery_disabled`  | `enable_endpoint_discovery = false` → policy returns `DoNotRetry`, latch never set. |
| AC-6 (nice) | `test_hub_region_header_persists_across_lease_not_found`  | After AC-2 latch, drive a 410/1022 retry → header still emitted. |
| AC-7 (nice) | `test_hub_region_header_persists_across_connection_failure` | After AC-2 latch, drive a connection failure retry → header still emitted. |

Test mechanics follow the existing pattern: build a `ClientRetryPolicy` with a mocked
`GlobalEndpointManager`, call `policy.should_retry(...)` with a synthetic 404/1002, then call
`policy.before_send_request(&mut request)` and assert
`request.headers.get(constants::SHOULD_PROCESS_ONLY_IN_HUB_REGION)` is `Some("true")` or `None`
as appropriate.

### 7.2 Integration Tests

No new recorded integration tests are required for this work item. Existing emulator-backed
fault-injection coverage in
`sdk/cosmos/azure_data_cosmos/tests/multi_write_tests/cosmos_multi_write_fault_injection.rs`
exercises 404/1002 paths and is expected to remain green; those tests assert on retry counts /
regions, not on outbound headers, so the additive header does not flip them. Phase 3+ may
optionally add a single recorded scenario asserting the header on a captured second-retry
request.

### 7.3 Validation Gates

Before opening the PR:

```text
cargo fmt -p azure_data_cosmos
cargo clippy -p azure_data_cosmos --all-features -- -D warnings
cargo build -p azure_data_cosmos --all-features
cargo test  -p azure_data_cosmos --all-features
```

All four must pass with no diff (fmt) and no warnings (clippy).

---

## 8. Open Questions & Future Work

### OQ-1 — Skip-set rotation parity *(resolved: Day 1 = .NET parity, no broader rotation)*

Issue [#4303](https://github.com/Azure/azure-sdk-for-rust/issues/4303) gestures at a broader
region-rotation flow (skip-set rotation on 403/3 NotWriteRegion) that .NET PR
[#5447](https://github.com/Azure/azure-cosmos-dotnet-v3/pull/5447) deliberately did not
implement. **Decision (owner-confirmed):** Day 1 ships .NET parity only — the
`x-ms-cosmos-hub-region-processing-only` header and its latch. Skip-set rotation is
explicitly out of scope for this work item. Rationale:

1. The header alone resolves the 404/1002 failback storm in .NET production telemetry; that
   is the validated fix, and parity with that fix is what #4303 is asking for on Day 1.
2. Skip-set rotation is a net-new design that touches `LocationCache` / preferred-region
   state and the `should_retry_on_endpoint_failure` path. Bundling that with header
   injection couples two unrelated risks in a single PR.
3. The latch introduced in §4.2 is the exact hook any future rotation logic would attach to,
   so deferring rotation is reversible at zero cost to this design.

Rotation is captured below under **Future Work** for a possible follow-up if post-rollout
telemetry shows the header alone is insufficient.

### OQ-2 — CHANGELOG version block *(resolved: append under existing `0.32.0 (Unreleased)`)*

Verified on `main` at spec time: the CHANGELOG already has an open
`## 0.32.0 (Unreleased)` block with a `### Breaking Changes` subsection. **Resolution:** add a
new `### Features Added` subsection above the existing `### Breaking Changes` (matching the
`Features Added` → `Breaking Changes` → `Other Changes` ordering used in `0.31.0`). Do not
introduce a new version block. Implementer should re-verify the block is still
`(Unreleased)` immediately before opening the PR; if the block has been cut to a release in
the interim, create a new `## 0.33.0 (Unreleased)` block above it.

### Future Work

- **Dedicated 403/3 skip-set rotation.** Day 1 explicitly ships .NET parity only. If
  post-rollout telemetry shows the header alone is insufficient — i.e. failback storms
  persist after the latched hub-targeted retry — file a follow-up to add an explicit
  skip-set in `should_retry_on_endpoint_failure`. The latch added in §4.2 is the intended
  hook point.
- **Diagnostics surface.** Surface "hub-region-only retry" as a first-class field on the
  Cosmos diagnostics object once the broader diagnostics design (see
  [Feed Operations Spec](https://github.com/Azure/azure-sdk-for-rust/pull/4261)) lands.
- **Backend rollout dashboarding.** Coordinate with service partners on a metric for
  header-honored vs. header-ignored response distributions during failover events.

# Slice 3b/c — Wrap/Unwrap Helpers + Transport Integration (point ops)

**Branch base:** `c475d879c` (Slice 3a)
**Spec:** `sdk/cosmos/azure_data_cosmos_driver/docs/GATEWAY_20_SPEC.md` §3, §5.2
**Scoping:** `.coding-harness/slice-3-scoping.md` §6 (revised plan), §7 (auth-signing)
**Prior brief:** `.coding-harness/slice-3a-brief.md` (style template)

This brief MERGES Slice 3b (helpers) and Slice 3c (integration) into one
sub-agent dispatch because the unwrap helper has no callable seam outside
`finalize_http_attempt` — its observable behavior only exists once integrated.

> Brief revised after rubber-duck round 1: token inventory expanded,
> transport plumbing carries explicit operation context, consistency
> resolution lives at operation-pipeline scope.

## Scope

In scope:

1. New module `driver/transport/gateway20_dispatch.rs` containing:
   - `wrap_request_for_gateway20` — produces a fresh wrapped `HttpRequest`
     (atomic; no partial mutation).
   - `unwrap_response_for_gateway20` — given a raw `(StatusCode, Headers,
     Vec<u8>)` triple from the proxy, decodes the RNTBD inner response and
     returns a synthetic `(StatusCode, Headers, Vec<u8>)` triple representing
     the inner request as if it had come straight from gateway1.
2. Add the **request-side** RNTBD token inventory enumerated in §"Verified
   facts" to `rntbd/tokens.rs` (mirror of `RntbdResponseToken`).
3. Add `account_name: Option<String>` to `TransportPipelineContext` and a
   `transport_mode: TransportMode` + the explicit operation fields enumerated
   below to `TransportRequest`, so the wrap helper has every input it needs
   without re-deriving anything from outgoing HTTP headers.
4. **Resolve the effective consistency level at the operation-pipeline
   layer** (per spec §5.2's precedence chain) and pass the resolved enum
   into `TransportRequest` — DO NOT re-derive it from the outgoing
   `x-ms-consistency-level` header inside the wrap helper.
5. Wrap call site: in `execute_transport_pipeline` immediately AFTER
   `sign_request` succeeds and BEFORE `execute_http_attempt` is invoked.
   Wrap failure converts to a synthetic `TransportError` mirroring the
   sign-failure code path (lines 239–252 of `transport_pipeline.rs`).
   Fail-fast — no silent gateway1 fallback (decided with user).
6. Unwrap call site: inside `finalize_http_attempt`'s `Response` arm,
   BEFORE `map_http_response_payload` is called. Unwrap substitutes the
   raw triple in place; on outer status != 200 the outer triple flows
   through (proxy/transport-level errors retain their outer status).
   On RNTBD decode failure substitute `TRANSPORT_GENERATED_503` via
   the existing `transport_error_result` shape.

Out of scope (deferred to a follow-up slice):

- Queries, read-feeds, partial-HPK, `StartEpkHash`/`EndEpkHash` token usage,
  EPK cutover (R5), continuation token format work, QueryPlan codec.
- Anything that requires reading from `request.requestContext.resolvedPartitionKeyRange`
  (we only support fully-specified PK in 3b/c — `EffectivePartitionKey` token only).
- DataPlane vs ControlPlane handling — eligibility already excludes ControlPlane
  in Slice 2 (`is_operation_supported_by_gateway20`).
- A new `gateway20_disabled` opt-out flag rename (R15) — separate slice.
- Any new diagnostic event for wrap/unwrap byte sizes (piggyback on existing
  per-attempt events; outer-200-but-decode-fail surfaces via the synthesized
  `TRANSPORT_GENERATED_503` failure path with diagnostic message).

## Verified facts (do not re-research)

### RNTBD request token IDs (verified against Azure/azure-sdk-for-java
`RntbdConstants.RntbdRequestHeader`)

The wrap helper MUST emit, at minimum, this token set for a point op
(omitting any of them risks silent live-only failures against the proxy):

| Token | ID | TokenType | Source |
|---|---|---|---|
| `AuthorizationToken` | `0x0001` | String | Inner Authorization header value |
| `PayloadPresent` | `0x0002` | Byte | 1 if request has body, else 0 |
| `Date` | `0x0003` | SmallString | Same RFC1123 date string used by `generate_authorization` (must match) |
| `ConsistencyLevel` | `0x0010` | Byte | Resolved consistency byte from §"Consistency byte mapping" |
| `DatabaseName` | `0x0015` | String | Parsed from `auth_context.resource_link` |
| `CollectionName` | `0x0016` | String | Parsed from `auth_context.resource_link` |
| `DocumentName` | `0x0017` | String | Parsed from `auth_context.resource_link` (only for `resource_type == Document` AND non-Create operations — see §"Resource-link parsing") |
| `TransportRequestId` | `0x004D` | ULong | Per-request monotonic counter — see §"TransportRequestId source" |
| `EffectivePartitionKey` | `0x005A` | Bytes | Hex-decoded EPK bytes (NOT the hex string) |
| `SDKSupportedCapabilities` | `0x00A2` | ULong | Same `SUPPORTED_CAPABILITIES_BITS` value already encoded into the outer `x-ms-cosmos-sdk-supportedcapabilities` header by `apply_cosmos_headers` (re-use the constant from `cosmos_headers.rs`) |
| `GlobalDatabaseAccountName` | `0x00CE` | String | Account name from `TransportPipelineContext.account_name` (`AccountEndpoint::global_database_account_name()` upstream) |

> **Verification step for the sub-agent:** every ID and TokenType in this
> table was cross-referenced against Java's
> `sdk/cosmos/azure-cosmos/src/main/java/com/azure/cosmos/implementation/directconnectivity/rntbd/RntbdConstants.java`.
> If any ID or TokenType you find in Java disagrees with this table,
> **trust Java's source over this brief** and call out the discrepancy
> in the report-back. Notable corrections from prior drafts of this brief:
>
> - `PayloadPresent` is `0x0002` (NOT `0x0001` — `0x0001` is `AuthorizationToken`).
> - `DatabaseName`, `CollectionName`, `DocumentName`, and
>   `GlobalDatabaseAccountName` are TokenType `String` (NOT `SmallString`).
>   Java uses `String` for variable-length name fields up to 64 KiB.
> - `EffectivePartitionKey` is `0x005A` and TokenType `Bytes` (variable-length).
> - A prior checkpoint mentioned `0x00F0` for `ConsistencyLevel`. That was
>   WRONG. The verified id is `0x0010`. Do not encode `0x00F0`.

### Consistency byte mapping (Java `RntbdConsistencyLevel` enum)

| Rust `DefaultConsistencyLevel` | Java byte |
|---|---|
| `Strong` | `0x00` |
| `BoundedStaleness` | `0x01` |
| `Session` | `0x02` |
| `Eventual` | `0x03` |
| `ConsistentPrefix` | `0x04` |

Cross-check the Rust enum's `pub(crate) enum DefaultConsistencyLevel`
(`models/consistency_level.rs:18`) ordering — DO NOT assume it matches
the byte values. Map explicitly via a `match` rather than `as u8`.

### Outer HTTP header set after wrap

Match Java's `ThinClientStoreModel.wrapInHttpRequest` exactly:

- `User-Agent` — preserve the value already set by `apply_cosmos_headers`
  (Rust uses the standard `USER_AGENT` header name — verified at
  `cosmos_headers.rs:49`).
- `x-ms-activity-id` — copy from the outgoing `http_request.headers`
  (verified producer at `operation_pipeline.rs:497-503`; the value
  comes from `ctx.activity_id`). Do NOT generate a new UUID. Do NOT
  read from `auth_context` (that struct does not carry an activity-id).
- DROP every other outer header. Specifically: `Authorization`,
  `x-ms-version`, `x-ms-cosmos-sdk-supportedcapabilities`, `Content-Type`,
  `Accept`, `Cache-Control`, all session/PK/consistency headers — they
  are encoded INTO the RNTBD frame instead.
- DO NOT set `x-ms-thinclient-proxy-operation-type` or
  `x-ms-thinclient-proxy-resource-type` on the OUTER request. Java does
  not (those are inside the RNTBD frame as resource-type/operation-type
  in the frame header). .NET's outer setting of these is divergent and
  Java is the canonical reference.
- DO NOT set `x-ms-thinclient-route-via-proxy` outer header. Java does
  not. The proxy distinguishes traffic by URL/port.

### Outer HTTP method/url/version

- Method: `POST` regardless of inner method.
- URL: leave unchanged. By the time wrap runs in Slice 3b/c the URL is
  already the thin-client URL because Slice 3a writes it into
  `routing.selected_url` when `transport_mode == TransportMode::Gateway20`.
- HTTP version: do not set explicitly. Java uses HTTP/2 by default via
  Reactor Netty. The Rust transport choice is the http_client_factory's
  responsibility — out of scope for the wrap helper.

### Inner authorization is signed first, captured into RNTBD

The standard `sign_request` runs FIRST against the inner method,
inner resource type, inner resource link (the `auth_context`'s fields).
The outgoing `http_request.headers` already carries the inner
`Authorization` value when wrap runs. Wrap reads that value and emits
an RNTBD `Authorization` token, then drops the outer header.

Verified against:

- Java `RxDocumentClientImpl.java:1806-1808,2432-2446`
- Java `BaseAuthorizationTokenProvider.java:137-145`
- .NET `TransportHandler.cs:99-108`
- .NET `AuthorizationTokenProviderMasterKey.cs:98-107`

### Effective partition key

Use the existing `EffectivePartitionKey::compute(pk_values, kind, version)`
helper at `models/effective_partition_key.rs:48`. The helper returns an
`EffectivePartitionKey` newtype whose `as_str()` exposes the hex string.
The RNTBD `EffectivePartitionKey` token expects RAW BYTES — hex-decode
the string before pushing it as `TokenValue::Bytes`.

`PartitionKeyDefinition` is available from the operation surface
(operation knows its container's PK definition, threaded through
`build_transport_request`).

DO NOT emit `StartEpkHash` / `EndEpkHash` — those are for partition-range-
resolved code paths that are out of scope.

### Account name source

Per Slice 3a brief §5: `AccountEndpoint::global_database_account_name() ->
Option<String>` is the producer. Slice 3a inlined `is_some()` at the
single call site. Slice 3b/c upgrades that to a real binding:

```rust
let account_name: Option<String> = account_endpoint.global_database_account_name();
```

threaded into both `resolve_endpoint` (replacing the bool) and
`TransportPipelineContext` so the wrap helper can read it. The wrap
helper requires `&str`; eligibility guarantees it is `Some` at the
Gateway 2.0 wrap call site (panic-safe `expect(...)`).

### TransportRequestId source

Java increments a per-request monotonic counter (a `static` AtomicLong
per channel in Direct mode). For Gateway 2.0 thin-client there is no
persistent channel, so a per-process atomic counter (or per-driver
counter) is acceptable for parity. The simplest implementation:

```rust
static GATEWAY20_TRANSPORT_REQUEST_ID: AtomicU32 = AtomicU32::new(1);
let trid = GATEWAY20_TRANSPORT_REQUEST_ID.fetch_add(1, Ordering::Relaxed);
```

The proxy uses this only for diagnostic correlation — uniqueness within
a process is sufficient. Thread-safety required.

### Date source

The wrap helper does NOT generate its own date. It MUST reuse the same
RFC1123 date string that `sign_request` used to produce the
`Authorization` value (mismatched dates → service-side signature
verification fails). Capture it at the wrap call site by inspecting
the outgoing `x-ms-date` header (set by `sign_request`); if missing,
return a wrap error. This guarantees the inner-encoded `Date` token
matches the date that signed the `Authorization` token.

### Consistency resolution (spec §5.2 — at operation-pipeline scope)

Per spec §5.2 precedence chain, the operation pipeline already resolves
both `ReadConsistencyStrategy` (line 73-76 of `operation_pipeline.rs`)
and `account_default_consistency` (line 61). Reconcile to one effective
consistency value at this layer using the existing helper
`ReadConsistencyStrategy::is_session_effective(account_default)` plus
the explicit precedence rules in spec §5.2:

1. If `ReadConsistencyStrategy::Eventual` → `DefaultConsistencyLevel::Eventual`.
2. If `ReadConsistencyStrategy::Session` → `DefaultConsistencyLevel::Session`.
3. If `ReadConsistencyStrategy::GlobalStrong` → `DefaultConsistencyLevel::Strong`
   (note Cosmos requires GlobalStrong only on Strong-account writes; out of
   scope for 3b/c — leave a `// TODO(slice-3d): GlobalStrong` if encountered).
4. If `ReadConsistencyStrategy::Default` → `account_default_consistency`.

Add a `pub(crate) fn resolve_effective_consistency(strategy:
ReadConsistencyStrategy, account_default: DefaultConsistencyLevel) ->
DefaultConsistencyLevel` somewhere in `models/consistency_level.rs`
(or alongside `is_session_effective` in `options/read_consistency.rs`)
and unit-test the table.

Pass the resolved `DefaultConsistencyLevel` into `TransportRequest` as
a new field. The wrap helper consumes it directly. **The wrap helper
does NOT inspect the outgoing `x-ms-consistency-level` HTTP header**
(currently no producer for that header — verified by grep).

## Surface map (file-by-file)

### `sdk/cosmos/azure_data_cosmos_driver/src/driver/transport/rntbd/tokens.rs`

Add:

- A new `RntbdRequestToken` enum (parallel to the existing
  `RntbdResponseToken`) listing every token from §"RNTBD request token IDs"
  with its `TryFrom<u16>` impl.
- Constructor helpers on `Token`:
  `Token::authorization(value: String)`, `Token::consistency_level(byte: u8)`,
  `Token::effective_partition_key(bytes: Vec<u8>)`,
  `Token::date(rfc1123: String)`, `Token::payload_present(present: bool)`,
  `Token::transport_request_id(id: u32)`, `Token::sdk_supported_capabilities(bits: u32)`,
  `Token::global_database_account_name(name: String)`,
  `Token::database_name(name: String)`, `Token::collection_name(name: String)`,
  `Token::document_name(name: String)`.
- Document `RntbdConsistencyLevel` byte mapping in a doc comment so future
  readers can verify against Java without re-grepping.

### `sdk/cosmos/azure_data_cosmos_driver/src/driver/transport/gateway20_dispatch.rs` (NEW)

```text
// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.
```

New `pub(crate)` items:

- `pub(crate) struct WrapInputs<'a>` carrying everything the wrap helper
  needs that is not already on the `HttpRequest`. Suggested fields:
  - `auth_context: &'a AuthorizationContext`
  - `account_name: &'a str`
  - `consistency: DefaultConsistencyLevel`
  - `partition_key: &'a PartitionKey`
  - `partition_key_definition: &'a PartitionKeyDefinition`
  - `operation_type: OperationType`
  - `resource_type: ResourceType`

  This struct exists purely to keep the wrap helper signature legible —
  the alternative is a 7-parameter `pub(crate) fn`.

- `pub(crate) fn wrap_request_for_gateway20(http_request: HttpRequest,
  inputs: WrapInputs<'_>) -> azure_core::Result<HttpRequest>`
  - **Atomic** — takes ownership of `http_request`, returns a new one.
    No partial mutation on error.
  - Snapshot `method`, `url`, the inner `Authorization` value, the inner
    `x-ms-activity-id` value, the inner `x-ms-date` value, the inner
    body, the inner `User-Agent` value.
  - If `Authorization` is absent → `Err(ErrorKind::DataConversion)`.
  - If `x-ms-date` is absent → `Err(ErrorKind::DataConversion)`.
  - If `x-ms-activity-id` is absent → `Err(ErrorKind::DataConversion)`
    (caller invariant guarantees presence).
  - Compute EPK from `inputs.partition_key` + `inputs.partition_key_definition`
    via `EffectivePartitionKey::compute(...)`. Hex-decode to bytes.
  - Parse database name / collection name / document name from
    `auth_context.resource_link.as_str()`. Path format: `dbs/{db}/colls/{coll}/docs/{doc}`
    — be tolerant of trailing-slash variations. Document name is `Some` only
    for `resource_type == Document` AND for non-Create operations (Java
    distinction: a Create points at the parent collection; the doc name
    only exists post-create). Verify by inspecting Java's path-parsing
    helper before encoding.
  - Build `RntbdRequestFrame` with metadata tokens enumerated in §"RNTBD
    request token IDs". Use `RntbdRequestFrame::serialize` to produce
    the body bytes.
  - Return a new `HttpRequest` with:
    - method `POST`
    - URL unchanged (already the thin endpoint)
    - body = wrapped RNTBD bytes
    - headers = ONLY `User-Agent` + `x-ms-activity-id`
    - everything else (`auth_context`, `execution_context`, `deadline`)
      unchanged — but note `HttpRequest` is the
      transport-pipeline's snapshot, not `TransportRequest`; verify the
      exact struct shape.

- `pub(crate) fn unwrap_response_for_gateway20(
      outer_status: StatusCode,
      outer_headers: Headers,
      outer_body: Vec<u8>,
  ) -> azure_core::Result<(StatusCode, Headers, Vec<u8>)>`
  - If `outer_status != 200 OK`, return the outer triple unchanged
    (proxy/transport-level error — handled by existing pipeline).
  - Else decode `RntbdResponse::deserialize(&outer_body)`. On decode
    error return `ErrorKind::DataConversion`.
  - Validate the inner status is in a sane range (e.g.,
    `100..=599`). If outside, treat as decode failure.
  - Build a synthetic `Headers` containing every header the
    downstream pipeline reads. AT MINIMUM:
    - `x-ms-substatus` ← `response.status.sub_status` (only if non-zero)
    - `x-ms-session-token` ← `response.session_token`
    - `x-ms-activity-id` ← `response.activity_id`
    - `x-ms-request-charge` ← `response.request_charge`
    - `x-ms-retry-after-ms` ← `response.retry_after_ms` (REQUIRED for
      `evaluate_transport_retry` at `transport_pipeline.rs:115-118` to
      see service-specified throttle delay)
    - `x-ms-continuation` ← `response.continuation_token` (if Some)
    - `etag` ← `response.etag` (if Some)
    - `x-ms-lsn` ← `response.lsn` (if non-zero)
    - `x-ms-item-lsn` ← `response.item_lsn` (if non-zero)
    - `x-ms-global-committed-lsn` ← `response.global_committed_lsn`
      (if non-zero)
  - Return `(response.status.status_code, synthetic_headers, response.body)`.

- Internal helpers as needed (e.g., `effective_partition_key_bytes`,
  `parse_resource_link_segments`, `consistency_to_byte`).
  Keep them `fn` (private), not `pub(crate)`.

### `sdk/cosmos/azure_data_cosmos_driver/src/driver/transport/mod.rs`

Add:

```rust
mod gateway20_dispatch;
pub(crate) use gateway20_dispatch::{
    unwrap_response_for_gateway20, wrap_request_for_gateway20, WrapInputs,
};
```

### `sdk/cosmos/azure_data_cosmos_driver/src/driver/pipeline/components.rs`

`TransportRequest` gets these new fields:

```rust
pub transport_mode: TransportMode,
pub operation_type: OperationType,
pub partition_key: Option<PartitionKey>,
pub partition_key_definition: Option<PartitionKeyDefinition>,
pub effective_consistency: DefaultConsistencyLevel,
```

Field placement: adjacent to `endpoint`. None of the new fields are
optional except the PK pair (Some only for ops that have a partition
key — eligibility ensures Some when `transport_mode == Gateway20`).

### `sdk/cosmos/azure_data_cosmos_driver/src/driver/transport/transport_pipeline.rs`

`TransportPipelineContext` gets new fields:

```rust
pub account_name: Option<String>,
```

Inside `execute_transport_pipeline` between line 252 (sign_request OK)
and line 269 (execute_http_attempt invocation):

```rust
let http_request = if request.transport_mode == TransportMode::Gateway20 {
    let account_name = ctx.account_name.as_deref()
        .expect("eligibility ensures account_name is Some when transport_mode==Gateway20");
    let pk = request.partition_key.as_ref()
        .expect("eligibility ensures PK is Some for Gateway20 point ops");
    let pk_def = request.partition_key_definition.as_ref()
        .expect("eligibility ensures PK definition is Some for Gateway20 point ops");

    let inputs = WrapInputs {
        auth_context: &request.auth_context,
        account_name,
        consistency: request.effective_consistency,
        partition_key: pk,
        partition_key_definition: pk_def,
        operation_type: request.operation_type,
        resource_type: request.auth_context.resource_type,
    };

    match wrap_request_for_gateway20(http_request, inputs) {
        Ok(req) => req,
        Err(e) => {
            diagnostics.fail_transport_request(
                request_handle,
                e.to_string(),
                RequestSentStatus::NotSent,
                CosmosStatus::CLIENT_GENERATED_400,
            );
            return TransportResult {
                outcome: TransportOutcome::TransportError {
                    status: CosmosStatus::CLIENT_GENERATED_400,
                    error: e,
                    request_sent: RequestSentStatus::NotSent,
                },
            };
        }
    }
} else {
    http_request
};
```

Plumb `should_unwrap: bool` (= `request.transport_mode ==
TransportMode::Gateway20`) into `finalize_http_attempt`. Wire it via
the existing arg-passing pattern; do not bolt onto a context struct
solely for this bool (unless the compiler protests, in which case
add it to the context struct that already passes through).

In `finalize_http_attempt`'s `HttpAttemptResult::Response { ... }`
arm, BEFORE calling `map_http_response_payload`:

```rust
let (status_code, headers, body) = if should_unwrap {
    match unwrap_response_for_gateway20(status_code, headers, body) {
        Ok(triple) => triple,
        Err(e) => {
            diagnostics.fail_transport_request(
                request_handle,
                e.to_string(),
                RequestSentStatus::Sent,
                CosmosStatus::TRANSPORT_GENERATED_503,
            );
            return ExecutedTransportAttempt {
                result: TransportResult { outcome: TransportOutcome::TransportError {
                    status: CosmosStatus::TRANSPORT_GENERATED_503,
                    error: e,
                    request_sent: RequestSentStatus::Sent,
                }},
                shard_id,
                shard_diagnostics,
            };
        }
    }
} else {
    (status_code, headers, body)
};
```

THEN call `map_http_response_payload(status_code, headers, body, ...)`.

### `sdk/cosmos/azure_data_cosmos_driver/src/driver/pipeline/operation_pipeline.rs`

- Top of `execute_operation_pipeline` (right after the existing
  `read_consistency_strategy` line at ~76):

  ```rust
  let effective_consistency = resolve_effective_consistency(
      read_consistency_strategy,
      account_default_consistency,
  );
  let account_name: Option<String> = account_endpoint.global_database_account_name();
  ```

  Pass `account_name.is_some()` to `resolve_endpoint` (signature unchanged
  from Slice 3a — still takes `account_name_present: bool`).

  Pass `account_name.clone()` into `TransportPipelineContext` at the
  `execute_transport_pipeline` call site (around line 200).

- At the `TransportRequest` construction site (`build_transport_request`
  helper at line 560-569), add to the struct literal:

  ```rust
  transport_mode: ctx.routing.transport_mode,
  operation_type: operation.operation_type(),
  partition_key: operation.partition_key().cloned(),
  partition_key_definition: operation.partition_key_definition().cloned(),
  effective_consistency: ctx.effective_consistency,
  ```

  (Verify the operation surface exposes
  `partition_key_definition()` — if not, thread it from where it lives,
  likely on the container metadata cache.)

- `OperationPipelineContext` (or whichever struct holds the per-operation
  shared context) gets `effective_consistency: DefaultConsistencyLevel`.

## Validation order (sub-agent MUST run, in order, with `-D warnings` where applicable)

1. `cargo fmt -p azure_data_cosmos_driver`
2. `cargo build -p azure_data_cosmos_driver --all-features`
3. `cargo clippy -p azure_data_cosmos_driver --all-features --all-targets -- -D warnings`
4. `cargo test -p azure_data_cosmos_driver --all-features --lib`
5. `cargo doc -p azure_data_cosmos_driver --no-deps`

Stop at first failure; fix; re-run from step 1.

## Required new tests

In `gateway20_dispatch.rs` `#[cfg(test)] mod tests`:

1. `wrap_then_decode_round_trips_authorization_token` — wrap a
   synthetic GET-document request with a known `Authorization` header,
   re-parse the produced body via the Slice 1 `RntbdRequestFrame`
   parser test helper, assert the Authorization token is present with
   the expected string value, assert it is NOT present in
   `http_request.headers` after wrap.
2. `wrap_drops_outer_headers_except_user_agent_and_activity_id` —
   start with a request carrying `Authorization`, `x-ms-version`,
   `x-ms-cosmos-sdk-supportedcapabilities`, `Accept`, `Content-Type`,
   `User-Agent`, `x-ms-activity-id`, `x-ms-date`. After wrap, only
   `User-Agent` and `x-ms-activity-id` should remain.
3. `wrap_sets_post_method_and_preserves_url` — start with method GET
   and URL pointing at the thin endpoint. After wrap, method is POST
   and URL is unchanged.
4. `wrap_encodes_consistency_byte_per_account_default` — for each of
   the 5 consistency levels, assert the encoded byte matches the
   verified table above.
5. `wrap_returns_error_when_inner_authorization_missing`
6. `wrap_returns_error_when_inner_date_missing`
7. `wrap_returns_error_when_inner_activity_id_missing`
8. `wrap_emits_required_token_set` — for one Document GET wrap, decode
   the produced frame and assert every token from §"RNTBD request token
   IDs" is present with the expected typed value.
9. `wrap_create_item_omits_document_name_token` — for a Document Create
   wrap (resource_link `dbs/db1/colls/c1`, no doc segment), decode the
   produced frame and assert: `DatabaseName=db1`, `CollectionName=c1`,
   `DocumentName` token is **absent**, and `PayloadPresent` byte is `1`
   (Create has a body). Use `OperationType::Create` with a non-empty
   request body.
10. `unwrap_decodes_inner_status_and_synthetic_headers` — feed an
    RNTBD response built via `RntbdRequestFrame` test helpers (or a
    companion `build_response_for_tests` helper if response.rs lacks
    one — add it next to the existing test scaffolding) with status
    429, retry-after-ms 250, session-token "x:y:1#2", activity-id
    uuid, request-charge 3.14. Assert the returned triple has
    StatusCode 429, headers contain `x-ms-retry-after-ms: 250`,
    `x-ms-session-token: x:y:1#2`, `x-ms-activity-id`,
    `x-ms-request-charge: 3.14`.
11. `unwrap_passes_through_outer_non_200` — feed `(StatusCode::BadGateway,
    empty headers, empty body)`. Returns the outer triple unchanged.
12. `unwrap_returns_error_on_invalid_rntbd_body` — feed a 200 outer
    status with `body = b"garbage"`. Expect
    `Err(ErrorKind::DataConversion)`.
13. `unwrap_returns_error_on_out_of_range_inner_status` — feed an
    RNTBD-decodable body whose inner status is `0`. Expect
    `Err(ErrorKind::DataConversion)`.

In `transport_pipeline.rs` `#[cfg(test)] mod tests`, add 4 integration
tests using the existing `MockTransportClient`-style scaffolding (search
for the existing `TransportClient` impls in the test module — likely
named `HangingTransportClient`, `EchoTransportClient`, etc.):

14. `gateway20_request_is_wrapped_before_send_and_unwrapped_on_response`
    — set up a mock that asserts the outgoing HTTP request method is
    POST, body parses as RNTBD, contains the inner Authorization token;
    the mock returns an RNTBD-encoded 200 response; assert the resulting
    `TransportResult` has the inner `CosmosStatus`.
15. `gateway20_inner_429_with_retry_after_triggers_throttle_retry`
    — mock returns RNTBD 429 with retry-after-ms=10 on attempt 1, RNTBD
    200 on attempt 2; assert the second attempt is reached and the
    `TransportResult` reflects the inner 200.
16. `gateway20_outer_502_propagates_as_transport_error_without_unwrap`
    — mock returns plain HTTP 502 with empty body; assert the
    `TransportResult.outcome` reflects the outer 502 unchanged
    (transport error path) and unwrap is NOT attempted.
17. `gateway20_inner_401_aborts_same_as_unwrapped_401` — mock returns
    RNTBD 401 inner; assert the resulting `TransportResult` carries
    inner status 401 and the existing 401 handling path runs (there is
    no special auth-retry branch — verify in `retry_evaluation.rs`).

In `models/consistency_level.rs` (or wherever `resolve_effective_consistency`
lives) `#[cfg(test)] mod tests`:

18. `resolve_effective_consistency_table` — full 4×5 table covering
    every `(ReadConsistencyStrategy, DefaultConsistencyLevel)` pair.

## DO NOT (carried from Slice 3a + new)

- DO NOT modify `endpoint.rs`, `cosmos_endpoint.rs`,
  `gateway20_eligibility.rs`, or `account_reference.rs`.
- DO NOT modify any file under `sdk/cosmos/azure_data_cosmos/`
  (the SDK crate). All changes are in the driver crate.
- DO NOT change `RoutingDecision` (set up correctly by Slice 3a).
- DO NOT touch `map_http_response_payload`. Unwrap happens BEFORE it.
- DO NOT add an HTTP/2 enforcement at this layer.
- DO NOT add a fault-injection hook for the wrap/unwrap bytes.
- DO NOT touch the spec doc, `slice-3-scoping.md`, or this brief.
- DO NOT push or open a PR.
- DO NOT bump any package version.
- DO NOT add new top-level dependencies in `Cargo.toml`.
- DO NOT inspect the outgoing `x-ms-consistency-level` header from
  inside the wrap helper. Pass the resolved enum in via `WrapInputs`.
- DO NOT generate a new activity-id UUID inside the wrap helper.
- DO NOT generate a new RFC1123 date string inside the wrap helper.

## Brief for the sub-agent (copy this verbatim into the prompt)

You are implementing Slice 3b/c of the Gateway 2.0 vertical-slice rollout.
Read this entire file before starting. Read the cited spec sections,
the cited Slice 1 codecs, and Slice 3a's brief and committed code. Your
deliverable is a clean working tree with one commit's worth of changes
that pass every command in §"Validation order" with `-D warnings` where
applicable. Do not commit. Do not push. Report back the file list
modified, the test count delta, the validation command outputs, and any
deviation from this brief with a one-line justification.

Verify every "verify against Java" token ID against the public
azure-sdk-for-java repository before encoding it. If a token ID
disagrees with what is encoded in this brief, **trust Java's source
over this brief** and call out the discrepancy in your report-back.



# Slice 3 Scoping — Gateway 2.0 Dispatcher (sub-slice proposal)

> **Status**: SCOPING. Not yet a per-slice implementation brief.
> **Goal of this doc**: pin scope boundaries for the dispatcher work, propose a sub-slicing strategy, and surface the design decisions a sub-agent will need.

## 1. What's left after Slices 1+2

After Slices 1 (RNTBD wire format) and 2 (eligibility helper, account-name extractor, GATEWAY20 constants), the only Gateway 2.0 traffic goes nowhere. To produce a working end-to-end Gateway 2.0 path the remaining work is:

1. **Routing decision must respect eligibility.** Today `resolve_endpoint` sets `transport_mode = TransportMode::Gateway20` whenever the endpoint has a `gateway20_url`. The helper from Slice 2 must be applied so ineligible operations fall back to standard Gateway. Same for accounts whose name doesn't parse — fall back rather than send a malformed token.
2. **Outgoing request must be wrapped.** A Gateway 2.0 outer HTTP request is always **POST** with `Content-Type: application/octet-stream`, RNTBD body, and the `x-ms-thinclient-*` / `x-ms-effective-partition-key` headers. The inner RNTBD frame carries the original operation method, resource path, and tokens.
3. **Auth signing depends on the outer method.** The Cosmos master-key / AAD signature is computed over `(method, resource_type, resource_link, date)`. Gateway 2.0's outer method is always POST, so the outer signature **must** be built with method=POST regardless of the inner operation. (Java / .NET both do this. Verified at `ThinClientStoreClient.cs` and `ThinClientStoreModel.processMessageAsync`.)
4. **Incoming response must be unwrapped.** The HTTP response carries an RNTBD frame whose inner status, sub-status, and tokens are the *real* result. The retry classification at `transport_pipeline.rs:307` (`evaluate_transport_retry`) MUST see the inner status, not the outer 200. Same for the session-token / activity-id / request-charge mapping that the operation pipeline reads at `operation_pipeline.rs:220`.
5. **EPK callsite cutover (R5).** `azure_data_cosmos/src/handler/container_connection.rs:121` still calls the SDK-side `get_hashed_partition_key_string`. Once the driver-side `EffectivePartitionKey::compute_range` is wired into the dispatcher (see step 4), the SDK helper becomes vestigial and the call must be redirected.

## 2. Why this can't be one slice

A single mega-slice has three problems:

- **Atomicity coupling**: wrap and unwrap MUST land in the same commit (otherwise every Gateway 2.0 call fails to parse). But routing wiring and the EPK callsite are independent and benefit from earlier landing.
- **Blast radius for review**: the wrap+unwrap commit alone touches the auth path, the request-build path, the retry classification path, and the diagnostics path. Code review on top of routing changes makes the diff incomprehensible.
- **Test isolation**: pure wrap/unwrap functions need golden-frame tests in isolation; routing changes need pipeline unit tests; the integration step needs end-to-end tests that depend on both.

Recommendation: **four sub-slices**.

## 3. Proposed sub-slices

### Slice 3a — Eligibility & account-name fallback wiring (small, isolated)

**Surface**: `driver/pipeline/operation_pipeline.rs::resolve_endpoint` (lines 339–390).

**Change**:
```rust
let use_gateway20 = selected.uses_gateway20(prefer_gateway20)
    && is_operation_supported_by_gateway20(operation.resource_type(), operation.operation_type())
    && account_endpoint.global_database_account_name().is_some();
```

The `account_endpoint.global_database_account_name()` plumb-through requires getting an `&AccountEndpoint` into `resolve_endpoint`. Today `resolve_endpoint` only takes `&CosmosEndpoint` (the regional/routing-level endpoint). The `AccountEndpoint` is held one layer up in `OperationPipeline`. Two options:

- **Option A (preferred)**: pass `&AccountEndpoint` as an additional argument to `resolve_endpoint`. Mechanical thread-through of `OperationPipeline::run` → `resolve_endpoint`.
- **Option B**: lift the `global_database_account_name` parse into `OperationPipeline::new` and store it as `Option<String>` on the pipeline struct. Avoids re-parsing per request.

Option B is more efficient (parse once vs per request) but introduces a new field on `OperationPipeline`. Sub-agent can decide; both are acceptable.

**Tests**: extend `resolve_endpoint_prefers_gateway20_for_dataplane_reads` with new cases:
- Ineligible op (Document × Head) → Gateway, not Gateway20.
- Unparseable account name → Gateway, not Gateway20.
- Eligible op + parseable name + dataplane → Gateway20 (the existing positive case).

**Risk**: low. No bytes change on the wire because the dispatcher (Slice 3c) hasn't landed; this just narrows when `TransportMode::Gateway20` is selected. Even if Slice 3a ships alone, downstream code still treats Gateway20 as Gateway1 (no wrap/unwrap), so this is silent.

**Wait** — there's an asymmetry. With Slice 1 and 2 alone, what happens when the routing resolves to `TransportMode::Gateway20` today? Read `transport/mod.rs:get_dataplane_transport`. Per the prior summary it routes to `AdaptiveTransport::gateway20()` when `connection_pool.is_gateway20_allowed()` is true — but no wrap/unwrap occurs. So today, Gateway 2.0 endpoints are reached over HTTP/2 but the body is still a raw HTTP request (no RNTBD wrap). The Gateway 2.0 server would reject this. **This means Gateway 2.0 is currently broken end-to-end and Slice 3a's effect is still net-positive (narrows when the broken path is taken, makes more requests succeed via the standard Gateway fallback).**

### Slice 3b — Pure wrap/unwrap helpers (medium, fully testable in isolation)

**New module**: `driver/transport/gateway20_dispatch.rs`.

**API**:
```rust
/// Builds a Gateway 2.0 outer HTTP request from a standard Cosmos
/// transport request. Forces method=POST, replaces body with an RNTBD
/// frame, injects the GATEWAY20_* + EFFECTIVE_PARTITION_KEY headers,
/// and rebuilds AuthorizationContext with method=POST.
pub(crate) fn wrap_request_for_gateway20(
    request: TransportRequest,
    operation: &CosmosOperation,
    account_name: &str,
) -> azure_core::Result<TransportRequest>;

/// Parses the RNTBD inner frame from a Gateway 2.0 HTTP response and
/// remaps status, sub-status, and tokens onto a standard
/// `TransportResult` so the retry path doesn't need to know about
/// Gateway 2.0.
pub(crate) fn unwrap_response_from_gateway20(
    http_status: u16,
    http_headers: &Headers,
    http_body: &[u8],
) -> azure_core::Result<TransportResult>;
```

**Auth signing decision**: `wrap_request_for_gateway20` must rebuild the `AuthorizationContext`. Today `AuthorizationContext::from_paths(method, resource_type, paths)` is computed in `build_transport_request` (line 468) from the *operation*'s method and resource. For Gateway 2.0 the *outer* signature uses POST + Document + the original resource link path (the link is correct since the URL host swaps but the path doesn't). Confirm by inspecting `AuthorizationContext`'s field layout — if it's `(method, resource_type, link)` then we just rebuild with `method=POST`.

**Tokens covered (minimal viable for retry correctness)**:
- Inner status code → `TransportOutcome` mapping (200/201/204 → Success; 4xx/5xx → HttpError with the code)
- Sub-status code → cosmos_headers (necessary for 404/1002, 410/1002, 429/3088)
- Session token → cosmos_headers
- Activity ID → cosmos_headers
- Request charge → cosmos_headers
- Retry-after MS → throttle path
- Continuation token → cosmos_headers
- LSN / Quorum-acked LSN → cosmos_headers (needed for some retry decisions)

**Tokens explicitly deferred to Slice 4+**:
- Diagnostics tokens (request-duration-ms, dependency timing, etc.)
- Partition-key-range-id (only relevant for direct mode pkrange routing)
- Anything not on the minimal-viable list above

**Tests**: golden-frame round-trip tests. Build a known `TransportRequest` for a Read; wrap it; assert the resulting bytes match a hand-crafted reference frame (or at minimum: re-decode the RNTBD frame and assert structural equivalence). Build a known RNTBD response frame; unwrap it; assert the resulting `TransportResult` matches the expected outcome. Use the Slice 1 codecs to construct golden frames.

**Risk**: medium. Any wrap/unwrap mismatch causes 100% failure on every Gateway 2.0 request. But mistakes are caught entirely by unit tests since the functions are pure.

### Slice 3c — Wire wrap/unwrap into the transport pipeline (small but high-stakes)

**Changes**:

1. In `transport_pipeline.rs::execute_transport_pipeline`, before line 234 (`apply_cosmos_headers`):
   ```rust
   if request.transport_mode == TransportMode::Gateway20 {
       request = wrap_request_for_gateway20(request, operation, account_name)?;
   }
   ```
   This means `TransportRequest` needs a `transport_mode` field, AND `execute_transport_pipeline` needs the `operation` ref + `account_name`. That's two new threadings.

2. After line 278 (`execute_http_attempt` returns), BEFORE line 289 (the shard-retry classification):
   ```rust
   if was_gateway20 {
       result = unwrap_gateway20_result(result)?;
   }
   ```
   The unwrap converts the outer-200-with-inner-frame outcome into a synthetic `TransportOutcome` that the retry path can classify normally.

3. The `endpoint_key` (line 202) must be the gateway20_url's authority when `transport_mode == Gateway20`. Verify `CosmosEndpoint::endpoint_key()` does this; if not, fix it. (The prior summary flagged this as a non-blocking concern from the Slice 2 rubber-duck — Slice 3c forces the issue.)

4. **Diagnostics**: a Gateway 2.0 response's outer 200 + inner 429 must be diagnosed as a 429, not a 200. Check `diagnostics.fail_transport_request` / equivalent and ensure the inner status reaches the diagnostic record.

**Tests**: integration-level unit tests in `transport_pipeline.rs` using a mock `AdaptiveTransport` that returns canned RNTBD frames. Cover:
- Document Read 200 → inner 200 → Success.
- Document Read 200 → inner 429 with retry-after 1s → throttle retry.
- Document Read 200 → inner 404/1002 → propagate (operation pipeline handles).
- Document Read 200 → inner 410/1002 → propagate.
- Document Read 200 → inner 401 → propagate.
- Wrap failure (e.g., un-encodable resource type) → fail attempt, no retry.
- Unwrap failure (malformed body) → propagate as a transport error.

**Risk**: HIGH — this is the slice that can break every Gateway 2.0 customer. Code-review must be aggressive.

### Slice 3d — R5: EPK callsite cutover (very small, isolated)

**Surface**: `azure_data_cosmos/src/handler/container_connection.rs:121`.

**Change**: replace the call to `get_hashed_partition_key_string` with the driver's `EffectivePartitionKey::compute` (single-key) or `compute_range` (range). Keep the SDK helper around as `#[deprecated]` to preserve public API the same way Slice 2 handled the constants.

**Tests**: existing tests for `container_connection` should pass unchanged. Add one new test confirming the EPK string matches the previous output for a known partition key value.

**Risk**: low. Pure SDK-side rewrite, single callsite, no protocol implications.

## 4. Open design questions for the rubber-duck

1. **Where to thread the `account_name` into `execute_transport_pipeline`?** Two options: as a new field on `TransportPipelineContext`, or store it on `TransportRequest`. The former is cleaner (it's a pipeline-level constant) but the wrap function needs it on a per-request basis. Need a recommendation.
2. **Should `TransportRequest::transport_mode` be added now (Slice 3c) or in Slice 3a?** Adding it in 3a means 3a touches more files; deferring to 3c keeps 3a small but means the wrap dispatch in 3c needs another way to know Gateway 2.0 is active. The `routing.transport_mode` is held one layer up at line 184. Probably best to add the field in 3c since 3a doesn't need it.
3. **What happens to `operation.is_idempotent()` for Gateway 2.0?** The retry path at `operation_pipeline.rs:197` uses `is_read_only() || is_idempotent()` to decide if a sent-but-no-response retry is safe. For Gateway 2.0 the RNTBD inner request is identical to the standard request, so the same idempotency rules apply. Should be a no-op.
4. **Continuation token format**: the RNTBD continuation token may have a different binary layout than the HTTP `x-ms-continuation` header value. Verify this against Java's `ThinClientStoreModel` decoding.
5. **Sub-slicing order**: should we ship 3a alone first (low-risk routing narrowing), then 3b (pure helpers), then 3c (integration), then 3d (R5)? Or pair 3a + 3b in one PR and 3c + 3d in another? Rubber-duck input wanted.
6. **Testing strategy for 3c**: the existing transport_pipeline tests use a `MockTransport`. Need to confirm the mock can return arbitrary HTTP bodies (not just status codes). If not, that's prep work in Slice 3b.
7. **Streaming responses**: are Cosmos Document responses ever streamed (chunked transfer)? If so, the unwrap function needs to buffer first. The current `transport_pipeline.rs` code reads body via `azure_core::Bytes` — assume this is already buffered.

## 5. Recommendation

Ship four sub-slices in order: **3a → 3b → 3c → 3d**.

Why:
- 3a is a one-line routing narrowing that immediately reduces the broken-Gateway-2.0 surface.
- 3b is pure functions and golden-frame tests; can be reviewed in isolation; lands before any pipeline change.
- 3c is the high-stakes integration; it imports the (already-tested) helpers from 3b and only modifies the transport pipeline. Code review on 3c is focused.
- 3d is a trivial cleanup that's blocked by 3c's `EffectivePartitionKey` usage being live.

Each sub-slice gets its own brief, sub-agent, code-review, validation, and commit. Stack on `tvaron3/gateway-2.0-impl` (Slice 1 + 2 already there).

---

## 6. Rubber-duck round 1 — findings & revised plan

The §1–§5 plan above was rubber-ducked. Six **blocking** issues surfaced; treat the §1–§5 sub-slicing as superseded by this section.

### 6.1 Confirmed defects in the original plan

1. **The 3b/3c seam is in the wrong place.**
   - Originally proposed: unwrap goes into `execute_transport_pipeline` *between* the `execute_http_attempt` return (line 278) and the throttle-retry classification (line 307).
   - Reality: `execute_http_attempt` calls `map_http_response_payload(...)` (`transport_pipeline.rs:627–657`), which **already** parses the outer status, builds `cosmos_headers` from the outer response, **and** calls `diagnostics.complete_request(handle, status_code, sub_status)` using the *outer* status. By the time control returns, the diagnostics record is already wrong.
   - **Fix**: the unwrap point must be **inside** the per-attempt body, **before** `map_http_response_payload`. Mechanically that means the dispatch shim sits at the boundary where the raw `(status, headers, body)` are first observed — i.e., the seam is "raw outer HTTP triple → raw inner triple", and *then* `map_http_response_payload` consumes the inner triple as if it had come from the wire.
   - Implication: 3b can still be a pure-helper module, but it has to expose **inner-triple synthesis** (returning `(StatusCode, Headers, Vec<u8>)`), not a `TransportResult`. Wiring the synthesis into the per-attempt path is the actual integration step in 3c.

2. **`evaluate_transport_retry` reads raw HTTP headers, not `cosmos_headers`.**
   - At `transport_pipeline.rs:115–118`: `result.response_headers().and_then(|h| h.get_optional_str(&RETRY_AFTER_MS))`. So a Gateway 2.0 inner 429 with retry-after only surfaces correctly if the synthetic inner headers are an `azure_core::http::headers::Headers` containing `x-ms-retry-after-ms`.
   - **Fix**: 3b's unwrap MUST produce a synthetic `Headers` (raw HTTP-style) carrying `x-ms-retry-after-ms`, `x-ms-substatus`, `x-ms-session-token`, `x-ms-activity-id`, `x-ms-request-charge`, `x-ms-continuation`, `x-ms-session-token-{lsn}`, and any header `map_http_response_payload` reads later.

3. **`endpoint_key` bug is real and must land in 3a, not 3c.**
   - `CosmosEndpoint::regional_with_gateway20(...)` precomputes `endpoint_key` from `gateway_url` (`endpoint.rs:71–79`). `endpoint_key()` returns the cached value (`endpoint.rs:95–97`). At `operation_pipeline.rs:202` the pipeline passes `routing.endpoint.endpoint_key()` regardless of `routing.transport_mode`. Result: Gateway 2.0 traffic is sharded by the gateway1 authority, fragmenting connection pools and breaking sticky-shard semantics.
   - **Fix**: source the key from `routing.selected_url` (or, more cleanly, store it alongside the URL inside `RoutingDecision`). 3a is the right slice — eligibility wiring already touches `resolve_endpoint`, and shipping 3a before this fix means we'd land a known-bad routing key with the narrative "we now route Gateway 2.0 correctly."

4. **Auth-signing claim is unverified.**
   - The original §3 cited `ThinClientStoreClient.cs` and `ThinClientStoreModel.processMessageAsync` as proof that the outer signature uses POST. Those files build the wrapper request — they are not the signer. The signer's behavior (whether it signs the outer method, the inner method, or omits outer signing entirely) is unconfirmed.
   - **Fix**: research is **blocking** before any 3b code is written. Two acceptable evidence sources: (a) Java/.NET `*AuthorizationTokenProvider` callsites along the Gateway 2.0 path, or (b) a wire-level test or capture that shows the `Authorization` header on a Gateway 2.0 request and the signed string-to-sign. If neither is locatable, defer Slice 3b until the spec author can confirm.

5. **3d is not safely deferrable while Slice 3 is "all dataplane reads".**
   - If Slice 3 turns Gateway 2.0 on for queries / read-feeds / partial-HPK requests, the driver still resolves PK ranges through SDK `get_hashed_partition_key_string` (`container_connection.rs:121`). That helper does not know about per-shard EPK ranges in the way the driver-side `EffectivePartitionKey::compute_range` does, so routing/header correctness is not proven for those operations.
   - **Fix**: narrow Slice 3 scope to **point operations on a fully-specified PK**. Queries, read-feeds, and partial-HPK are explicitly out-of-scope until a follow-up slice does the EPK cutover. With this narrowing, 3d (R5) is no longer required for Slice 3 to be correct — it can ship in the follow-up that opens up queries.

6. **QueryPlan codec is still placeholder; consistency-reconciliation token `0x00F0` is missing.**
   - `rntbd/request.rs:129–146`: QueryPlan currently uses the SqlQuery wire ID until metadata rules land. With Slice 3 narrowed to point ops (per finding 5), QueryPlan is out-of-scope and this is fine.
   - Spec §5.2 mandates `ReadConsistencyStrategy` ↔ `ConsistencyLevel` reconciliation and emission of RNTBD token `0x00F0`. With Slice 3 narrowed to point ops, this becomes "necessary for correct point-op consistency semantics" rather than "necessary for queries". **Decision required**: include in 3b, or land as a follow-up slice **before** turning Gateway 2.0 on by default. Recommendation: include in 3b — the cost is small (it's a header normalization + one extra token emit) and shipping point ops without it is a customer-visible consistency bug.

### 6.2 Revised sub-slicing

Three sub-slices, plus an explicit research gate.

#### Slice 3-research (BLOCKING; no code)

Research the auth-signing path for Gateway 2.0 (finding 4). Output: a one-paragraph note in this scoping doc with citations to actual signer code (Java or .NET) confirming whether the outer Authorization header is built with POST, the inner method, or omitted. Until this lands the scoping for 3b is incomplete.

#### Slice 3a — Routing eligibility + endpoint_key fix (REVISED)

Combines the original 3a with the endpoint_key fix.

Surface:
- `driver/pipeline/operation_pipeline.rs::resolve_endpoint` — gate `TransportMode::Gateway20` on `is_operation_supported_by_gateway20(...)` AND `account_endpoint.global_database_account_name().is_some()`.
- `driver/routing/endpoint.rs` — either: (option Y) make `endpoint_key()` aware of which URL was selected (new arg), or (option Z) push `endpoint_key` out of `CosmosEndpoint` and store it alongside `selected_url` in `RoutingDecision`. Option Z is cleaner; sub-agent picks.
- `driver/pipeline/components.rs::RoutingDecision` — if option Z, gain an `endpoint_key` field.
- `driver/pipeline/operation_pipeline.rs:202` — source `endpoint_key` from `routing.endpoint_key` (option Z) or `routing.endpoint.endpoint_key(routing.transport_mode)` (option Y).

Account-name plumbing: Option B from the original plan (parse once at `OperationPipeline::new`, store as `Option<String>` on the pipeline). Eliminates per-request parsing.

Tests:
- All cases from original 3a (eligible/ineligible/unparseable).
- One new endpoint_key test: build a `regional_with_gateway20`, drive `resolve_endpoint` for a dataplane Read, assert the resulting routing decision's `endpoint_key` reflects the **gateway20** authority (not gateway1).

Risk: low. No bytes change on the wire because 3b/3c haven't landed.

#### Slice 3b/c — Wrap/unwrap helpers + wire integration (MERGED)

Originally two slices; the bad 3b/3c seam (finding 1) means they have to land together. Net change is one PR, but internal structure preserves the helper module.

New module `driver/transport/gateway20_dispatch.rs` exposes:
```rust
/// Builds a Gateway 2.0 outer HTTP request from a standard Cosmos
/// transport request. Forces method=POST, replaces body with an RNTBD
/// frame, injects GATEWAY20_* and EFFECTIVE_PARTITION_KEY headers.
/// Auth signing rebuild is conditional on the result of slice 3-research.
pub(crate) fn wrap_request_for_gateway20(
    request: TransportRequest,
    operation: &CosmosOperation,
    account_name: &str,
) -> azure_core::Result<TransportRequest>;

/// Parses an RNTBD inner frame from a Gateway 2.0 HTTP response and
/// returns a synthetic raw HTTP triple that downstream code (specifically
/// `map_http_response_payload`) can consume as if it came from the wire.
/// The synthetic Headers MUST include x-ms-retry-after-ms,
/// x-ms-substatus, x-ms-session-token, x-ms-activity-id,
/// x-ms-request-charge, x-ms-continuation, and any other header read by
/// map_http_response_payload or evaluate_transport_retry.
pub(crate) fn unwrap_response_for_gateway20(
    http_status: StatusCode,
    http_headers: &Headers,
    http_body: &[u8],
) -> azure_core::Result<(StatusCode, Headers, Vec<u8>)>;
```

Integration in `transport_pipeline.rs`:
- BEFORE `apply_cosmos_headers` (line 234): if `request.transport_mode == TransportMode::Gateway20`, replace `request` with `wrap_request_for_gateway20(...)`. Requires a new `transport_mode` field on `TransportRequest` (and `account_name` on `TransportPipelineContext`).
- Inside the per-attempt body, **before** `map_http_response_payload`: if the attempt was Gateway 2.0, replace the raw `(status, headers, body)` with the unwrap output. From that point downstream code is unmodified.

Spec §5.2 work (finding 6 b): in `wrap_request_for_gateway20`, reconcile `ReadConsistencyStrategy` against `ConsistencyLevel` per spec §5.2 and emit RNTBD token `0x00F0` accordingly. Tested in 3b unit tests.

Tests (within the merged slice):
- Pure unit tests on the helpers (golden-frame round-trip, inner-triple synthesis fidelity).
- Pipeline integration tests covering the seven cases from original §3c (200/inner-200, 200/inner-429+retry-after, 200/inner-404/1002, 200/inner-410/1002, 200/inner-401, wrap-failure, unwrap-failure).

Risk: high. Mitigated by (a) the merged seam being correct, (b) golden-frame tests on the helpers, (c) integration tests that exercise the throttle-retry path with synthetic 429s and verify the retry-after-ms is honored.

#### Slice 3d — DEFERRED (out of scope)

R5 EPK callsite cutover (`container_connection.rs:121`) becomes part of the follow-up slice that opens up queries / read-feeds / partial-HPK. Slice 3 ships Gateway 2.0 for **point ops with fully-specified PK only**, and the SDK helper continues to be used for non-point operations until that follow-up.

### 6.3 Open questions still unresolved

- **Option Y vs Z for the endpoint_key fix.** Z is cleaner (decouples the key from the endpoint object), Y is smaller. Sub-agent decides during 3a.
- **Streaming responses.** Confirmed in §4 question 7 of the original — Cosmos Document responses use buffered `azure_core::Bytes`. Treat as resolved (no-op for unwrap).
- **Idempotency for Gateway 2.0** (original §4 q3). Resolved: same rules — RNTBD inner request is identical to standard.
- **Continuation token binary layout** (original §4 q4). With Slice 3 narrowed to point ops, queries/read-feeds are out of scope and this is no longer blocking. Re-open in the follow-up slice.

### 6.4 Revised gate order

1. **3-research** → unblocks 3b/c.
2. **3a** → narrows broken Gateway 2.0 surface AND fixes routing key. Lands first.
3. **3b/c (merged)** → ships Gateway 2.0 for point ops on fully-specified PK. Includes consistency reconciliation.
4. (deferred) follow-up slice for queries/read-feeds + EPK cutover + continuation-token format.

---

## 7. Slice 3-research result (auth signing for Gateway 2.0)

Resolved by general-purpose sub-agent against Java + .NET SDKs.

**Key finding**: the **outer HTTP request has no `Authorization` header**. Both Java and .NET sign the inner logical request first, then wrap. The signed inner `Authorization` is **carried as an RNTBD token** inside the wrapped frame.

Citations (external repos):
- `azure-sdk-for-java RxDocumentClientImpl.java:1806–1808, 1783–1785` — passes inner verbs (GET/DELETE/etc.) to `populateHeadersAsync`.
- `azure-sdk-for-java RxDocumentClientImpl.java:2432–2446` — signs `request.getResourceAddress()`, `request.getResourceType()`, `httpMethod`, then sets Authorization on the inner request.
- `azure-sdk-for-java BaseAuthorizationTokenProvider.java:137–145` — HMAC payload: lowercased verb + resource segment + resource link + `x-ms-date`.
- `azure-sdk-for-java ThinClientStoreModel.java:237–271` — builds outer headers separately, encodes RNTBD, sends outer `HttpMethod.POST`; **no auth re-run**.
- `azure-sdk-for-java RntbdRequestHeaders.java:143–147` — copies `Authorization` from inner headers into RNTBD metadata.
- `azure-cosmos-dotnet-v3 TransportHandler.cs:99–108` — .NET signs before store dispatch using inner method/resource type/resource address.
- `azure-cosmos-dotnet-v3 AuthorizationTokenProviderMasterKey.cs:98–107` — sets `XDate`, signs with the supplied verb/resource/link.
- `azure-cosmos-dotnet-v3 ThinClientStoreClient.cs:141–168` — serializes the **signed** request, **clears outer headers**, sets outer URI to thin endpoint, outer method to POST.

### 7.1 Implications for Slice 3 design

1. **No auth-context rebuild.** The original §3 plan (rebuild `AuthorizationContext` with method=POST inside `wrap_request_for_gateway20`) is wrong. The standard `sign_request` path is correct as-is — it signs the inner logical request, which is exactly what we want.

2. **Wrap point moves.** Instead of "before `apply_cosmos_headers` (line 234)", the wrap point is **after `sign_request` succeeds (line 252)** and **before `execute_http_attempt` (line 269)**. At that moment, `http_request.headers` already carries the signed inner `Authorization` and other Cosmos headers; the wrap captures `(method, url.path, headers, body)`, encodes them as RNTBD, and produces a new `HttpRequest` that replaces:
   - URL host with gateway20_url's authority (path may be `/` or whatever the thin endpoint expects).
   - Method with POST.
   - Body with the RNTBD frame bytes.
   - Headers with the thin-client outer set (`Content-Type: application/octet-stream`, `x-ms-thinclient-proxy-operation-type`, `x-ms-thinclient-range-min`/`-max`, `x-ms-effective-partition-key`, `x-ms-thinclient-account-name`, plus user-agent / activity-id / date that need to remain visible at the proxy). **No outer Authorization header is emitted.**

3. **Resource type in string-to-sign is the inner type** (e.g., `colls`, `dbs`, `docs`), not always `docs`. With the new flow, this is automatic — no changes needed in the auth path.

4. **`account_name` is still needed by wrap** — for the `x-ms-thinclient-account-name` outer header and for the RNTBD frame's account metadata token, not for auth. Slice 2 already provides the helper; Slice 3a routes it.

### 7.2 Prep work surfaced by the research

**Slice 1's RNTBD codecs do not include the `Authorization` token id** (verified via `grep -i authoriz` against `driver/transport/rntbd/`). Slice 3b/c must add:
- The token id constant in `rntbd/tokens.rs` (matching Java's `Authorization` request-header id — verify against `azure-sdk-for-java RntbdConstants.java`).
- Encode-side support so the wrap function can copy `Authorization` from the signed inner `http_request.headers` into the RNTBD frame.

This is small (a single token id + encode call) but it is **net-new RNTBD wire-format work** that didn't exist in Slice 1's scope. It must land in 3b/c, not in a stand-alone slice (because no other code path needs it).

### 7.3 Updated wrap helper signature (replaces §6.2 sketch)

```rust
/// Wrap a signed inner Cosmos HttpRequest as a Gateway 2.0 outer POST.
///
/// Preconditions:
/// - `inner` MUST already be signed (sign_request must have run).
/// - `inner.headers["authorization"]` is captured into the RNTBD frame
///   and removed from the outer headers (matches .NET's "clears outer
///   headers" behaviour).
/// - `gateway20_url` is the thin endpoint authority for the selected
///   region.
/// - `account_name` populates the x-ms-thinclient-account-name outer
///   header and the RNTBD account-name token.
pub(crate) fn wrap_request_for_gateway20(
    inner: HttpRequest,
    operation: &CosmosOperation,
    gateway20_url: &Url,
    account_name: &str,
    effective_partition_key: Option<&[u8]>,
) -> azure_core::Result<HttpRequest>;
```

The unwrap helper signature from §6.2 is unchanged.

### 7.4 Gate status

Slice 3-research: **resolved**. Slices 3a and 3b/c are unblocked.

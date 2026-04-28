# SDK-to-Driver Cutover: Design Specification

## Overview

This document describes the design for routing `azure_data_cosmos` SDK operations through the `azure_data_cosmos_driver` execution engine, replacing the legacy gateway pipeline path. The first operation cut over is `ContainerClient::read_item`, which serves as the **reference pattern** for all subsequent operations.

### Context

Prior to this work, the Cosmos SDK had two separate execution paths:

- **Gateway pipeline** (`azure_data_cosmos`): The SDK handled auth, routing, retries, and request construction via `CosmosRequest` → `GatewayPipeline` → HTTP.
- **Driver** (`azure_data_cosmos_driver`): A newer execution engine with its own transport, routing, and operation model (`CosmosOperation` + `OperationOptions`). Previously used only in driver-level tests.

[PR #4005](https://github.com/Azure/azure-sdk-for-rust/pull/4005) bridged the two worlds by having `ContainerClient::new()` call `driver.resolve_container()` for eager metadata resolution. This PR takes the next step: routing the first data operation through the driver.

### Goal

Make the SDK client a **thin wrapper** over the driver. The SDK translates public-facing types into driver concepts, delegates execution, and translates the response back. All real work (auth, routing, retries, transport) happens inside `driver.execute_operation()`.

## Architecture

### Data Flow

```text
User calls:     container_client.read_item(pk, id, options)
                              │
                    ┌─────────▼────────────┐
                    │  SDK ContainerClient │
                    └─────────┬────────────┘
                              │
          ┌───────────────────┼───────────────────┐
          │                   │                   │
  PartitionKey           ItemOptions        ContainerRef
  (SDK type)             (SDK type)        (driver type,
          │                   │           stored on client)
          │                   │                   │
          ▼                   ▼                   ▼
  into_driver_pk()   item_options_to_       ItemReference::
          │           operation_options()    from_name()
          │                   │                   │
          └───────────────────┼───────────────────┘
                              │
                    ┌─────────▼──────────┐
                    │  CosmosOperation:: │
                    │    read_item()     │
                    └─────────┬──────────┘
                              │
                    ┌─────────▼───────────┐
                    │  driver.execute_    │
                    │  operation(op, opts)│
                    │                     │
                    │  (auth, routing,    │
                    │   retries, HTTP)    │
                    └─────────┬───────────┘
                              │
                    ┌─────────▼───────────┐
                    │  driver_response_   │
                    │  to_cosmos_response │
                    └─────────┬───────────┘
                              │
                    ┌─────────▼───────────┐
                    │  CosmosResponse<T>  │
                    │  (SDK public type)  │
                    └─────────────────────┘
```

### Key Principle

The SDK's public API does not change. `read_item` retains the same signature, return type, and observable behavior. This is a pure internal refactor.

## Design Decision: Driver as Required Infrastructure

An alternative approach was explored where the driver is **optional** — stored as `Option<Arc<CosmosDriver>>` on `CosmosClient`, `DatabaseClient`, and `ContainerClient`. In that model, each operation checks at runtime whether a driver is available: if so, it takes the driver path; otherwise, it falls back to the legacy gateway pipeline. Container metadata resolution is also optional and failure is silently ignored.

We chose **not** to take that approach, since we want to verify the behavior of the driver being used only and this single method will serve as the test. In this design, the driver is **required**:

- `CosmosClient` stores `Arc<CosmosDriver>` (not `Option`).
- `ContainerClient::new()` eagerly resolves container metadata via the driver and returns `Result` — if resolution fails, the client cannot be created.
- Operations have a **single codepath** through the driver, with no gateway fallback.

### Rationale

The purpose of this cutover is to validate that the driver can fully replace the gateway pipeline for each operation. A fallback path undermines that goal:

- **Testability:** If the driver path can silently fall back to the gateway, we can't be 100% sure that the driver path is exercised in production or tests. Failures would be hidden rather than surfaced.
- **Correctness:** A dual-codepath design requires maintaining behavioral parity between two implementations indefinitely. A single path is easier to reason about, test, and debug.
- **Options fidelity:** A fallback path tempts skipping the options translation (e.g., passing empty `OperationOptions` on the driver path), which silently drops user-configured session tokens, etags, and excluded regions.
- **Response fidelity:** A minimal fallback implementation may skip reconstructing response headers from the driver's typed response, causing callers to get `None` for `request_charge()`, `session_token()`, and `etag()`.

The cutover is intentionally incremental — one operation at a time. Operations that haven't been cut over yet continue using the gateway pipeline naturally (they don't call the driver). This gives us the gradual rollout benefit without the complexity of runtime branching within a single operation.

## Type Translation Decisions

### PartitionKey (SDK → Driver)

The SDK and driver define **separate `PartitionKey` types** with identical structure but in different crates. Both represent a JSON array of typed values (string, number, bool, null).

**Approach:** Added `into_driver_partition_key()` on the SDK's `PartitionKey` that maps each `InnerPartitionKeyValue` variant to the driver's `PartitionKeyValue`.

**Driver change required:** Made `PartitionKeyValue` `pub` (was `pub(crate)`) so the SDK crate can construct `Vec<PartitionKeyValue>` for the conversion.

**Future consideration:** Once Ashley's options alignment work unifies these types, this conversion can be eliminated, and we can just use the Driver's definitions the way we did with the ContainerReference.

```rust
// SDK partition_key.rs
pub(crate) fn into_driver_partition_key(self) -> driver::PartitionKey {
    let driver_values: Vec<DriverPKV> = self.0.into_iter()
        .map(|v| match v.0 {
            InnerPartitionKeyValue::String(s) => DriverPKV::from(s),
            InnerPartitionKeyValue::Number(n) => DriverPKV::from(n),
            InnerPartitionKeyValue::Bool(b) => DriverPKV::from(b),
            InnerPartitionKeyValue::Null => DriverPKV::from(Option::<String>::None),
            // ...
        })
        .collect();
    DriverPK::from(driver_values)
}
```

### ItemOptions → OperationOptions

The SDK's `ItemOptions` (item-scoped request options) maps to the driver's `OperationOptions` field-by-field. The types in each field differ between crates, so values are bridged via their string representations.

| SDK `ItemOptions` field | Driver `OperationOptions` | Conversion |
| --- | --- | --- |
| `session_token: Option<SessionToken>` | `.with_session_token()` | `DriverSessionToken::new(token.to_string())` |
| `if_match_etag: Option<Etag>` | `.with_etag_condition()` | `Precondition::if_match(ETag::new(etag.to_string()))` |
| `custom_headers: HashMap<...>` | `.with_custom_headers()` | Passed through directly (types are the same) |
| `excluded_regions: Option<Vec<RegionName>>` | `.with_excluded_regions()` | `Region::new(name.to_string())` for each |
| `content_response_on_write_enabled: bool` | *Ignored for reads* | Driver always returns body for point reads |

**Driver change required:** Added `custom_headers` support to `OperationOptions` (new field, setter, getter) and wired it into `build_transport_request` in `operation_pipeline.rs`. Custom headers may be removed in the future as we analyze which options are truly needed.

### Response Bridge (Driver → SDK)

The driver returns an untyped `CosmosResponse { body: Vec<u8>, headers: CosmosResponseHeaders, status: CosmosStatus }`. The SDK returns a typed `CosmosResponse<T>` wrapping `azure_core::Response<T>`.

**Approach:** Reconstruct the SDK response from driver parts:

```rust
pub(crate) fn driver_response_to_cosmos_response<T>(
    driver_response: DriverResponse,
) -> CosmosResponse<T> {
    let status_code = driver_response.status().status_code();
    let headers = cosmos_response_headers_to_headers(driver_response.headers());
    let body = driver_response.into_body();

    let raw = RawResponse::from_bytes(status_code, headers, Bytes::from(body));
    let typed: Response<T> = raw.into();
    CosmosResponse::new(typed, None)
}
```

The header conversion maps each typed `CosmosResponseHeaders` field back to its raw header name/value pair (reverse of the driver's `from_headers()` parser).

**Caveat:** Only headers that the driver explicitly parses are preserved (activity ID, request charge, session token, etag, continuation, item count, substatus). Any other server headers are lost. This covers all standard Cosmos response metadata. We will probably come back to this when we do the work on verifying the headers we want.

### CosmosRequest → Optional

The SDK's `CosmosResponse<T>` previously held the original `CosmosRequest` — a gateway pipeline concept with no driver equivalent. The driver uses `CosmosOperation` + `OperationOptions` instead, which are consumed during execution.

**Decision:** Made the `request` field `Option<CosmosRequest>`:

- Gateway-routed operations (all methods not yet cut over) continue setting `Some(request)`.
- Driver-routed operations set `None`.
- The field is only accessed behind `#[cfg(feature = "fault_injection")]` and marked `#[allow(dead_code)]`.
- A TODO comment marks it for removal once all operations are on the driver.

## Structural Changes

### ContainerClient

Added two fields to `ContainerClient` so `read_item` can reach the driver at execution time:

```rust
pub struct ContainerClient {
    // ... existing fields ...
    driver: Arc<CosmosDriver>,         // retained from new()
    container_ref: ContainerReference,  // cloned before passing to ContainerConnection
}
```

Previously, the driver was discarded after `new()` and `ContainerReference` was buried inside `ContainerConnection`.

### driver_bridge Module

New private module at `src/driver_bridge.rs` containing:

- `driver_response_to_cosmos_response<T>()` — response conversion
- `item_options_to_operation_options()` — options translation
- `driver_response_headers_to_headers()` — converts the driver's typed response headers (e.g., `activity_id: Option<ActivityId>`, `request_charge: Option<RequestCharge>`) into raw `azure_core::Headers` key-value pairs for the SDK response

This module is the shared foundation for all future operation cutover. When cutting over `create_item`, `delete_item`, etc., they reuse the same bridge functions.

## Applying This Pattern to Other Operations

To cut over another item operation (e.g., `create_item`), follow this template:

1. **Build the operation:** Use the appropriate `CosmosOperation::*` factory method (e.g., `CosmosOperation::create_item(container_ref, pk)`).
2. **Attach the body:** For write operations, serialize the item to bytes and call `.with_body(bytes)` on the operation.
3. **Wire session token and etag:** These live on `CosmosOperation`, not `OperationOptions`. Set them inline before executing:

   ```rust
   if let Some(session_token) = options.session_token() {
       operation = operation.with_session_token(session_token.to_string());
   }
   if let Some(etag) = options.if_match_etag() {
       operation = operation.with_precondition(
           Precondition::if_match(ETag::new(etag.to_string())),
       );
   }
   ```

   This is separate from the bridge function because Ashley's options alignment (#4055) moved session token and etag to `CosmosOperation` (the operation itself carries per-request state, while `OperationOptions` carries cross-cutting config).

4. **Translate options:** Reuse `item_options_to_operation_options()` from `driver_bridge.rs`. This handles `excluded_regions` and `custom_headers`.
5. **Execute:** Call `self.driver.execute_operation(operation, driver_options).await?`.
6. **Bridge response:** Call `driver_response_to_cosmos_response(driver_response)` to get a `CosmosResponse<T>`, then wrap it in the appropriate public response type (e.g., `ItemResponse::new(cosmos_response)` for item operations, `ResourceResponse::new(cosmos_response)` for resource operations).

The public method signature should not change.

## Response Type Wrapping

PR #3960 introduced dedicated public response types that wrap the internal `CosmosResponse<T>`:

| Public Type | Used For | Extra Fields |
| --- | --- | --- |
| `ItemResponse<T>` | create/read/replace/upsert/delete item | `etag()` |
| `ResourceResponse<T>` | create/read/delete database/container | — |
| `BatchResponse` | transactional batch | `etag()` |
| `QueryFeedPage<T>` | query operations | `index_metrics()`, `query_metrics()` |

`CosmosResponse<T>` is now `pub(crate)`. The bridge function `driver_response_to_cosmos_response()` returns `CosmosResponse<T>`, and the caller wraps it:

```rust
// In read_item:
Ok(ItemResponse::new(
    crate::driver_bridge::driver_response_to_cosmos_response(driver_response),
))

// In a future create_container:
Ok(ResourceResponse::new(
    crate::driver_bridge::driver_response_to_cosmos_response(driver_response),
))
```

`CosmosResponse` has two constructors:
- `new(response, request)` — for gateway-routed operations (has a `CosmosRequest`)
- `from_response(response)` — for driver-routed operations (no `CosmosRequest`, sets `request: None`)

Both constructors parse `CosmosResponseHeaders` from the raw HTTP headers and build `CosmosDiagnostics` (activity ID, server duration) automatically. The bridge's `driver_response_headers_to_headers()` ensures the driver's typed headers are converted back to raw headers so the SDK's parsing works correctly.

### `request_url()` and Fault Injection Tests

`ItemResponse::request_url()` returns `Option<Url>` — `None` for driver-routed operations, `Some(url)` for gateway-routed operations. Other response types (`ResourceResponse`, `BatchResponse`) return `Url` directly since they are always gateway-routed.

For fault injection tests that verify failover endpoints:
- Gateway-routed operations: use `.request_url().expect("...")`
- Driver-routed operations: use `if let Some(url) = response.request_url() { ... }`

This means failover endpoint assertions are **silently skipped** for driver-routed reads. Once driver diagnostics expose the effective endpoint (tracked as future work), these assertions should be restored.

### Driver Response Does Not Expose the Effective Endpoint

The driver's `CosmosResponse` returns the response body, headers, and status — but does **not** expose which endpoint (URL or region) was ultimately used to serve the request. This information is critical for:

- **Failover verification tests** — asserting that a request was routed to the expected region after a fault-triggered failover
- **Diagnostics and observability** — understanding which region served a request for debugging and performance analysis

The gateway pipeline tracked this via `CosmosRequest` (which held the final URL). The driver handles routing internally in the operation pipeline (`resolve_endpoint` → `RoutingDecision`) but does not propagate the resolved endpoint back through the response.

**Future work:** The driver should surface the effective endpoint in its response (e.g., as a field on `CosmosResponse` or through `DiagnosticsContext`). This would allow:
1. Restoring failover endpoint assertions in tests (currently skipped with `if let Some`)
2. Providing users with routing transparency for observability
3. Removing the `request_url()` → `Option` workaround once all operations are driver-routed

**Tests with skipped endpoint assertions** (these should be restored once the driver exposes the effective endpoint):

| Test File | Test Name | What it verifies |
| --- | --- | --- |
| `cosmos_items.rs` | `assert_response` helper (all item tests) | Endpoint matches expected host |
| `cosmos_fault_injection.rs` | `fault_injection_429_retry_with_hit_limit` | Endpoint matches hub |
| `cosmos_multi_write_retry_policies.rs` | `read_cross_region_retry_on_408` | Failover to satellite region |
| `cosmos_multi_write_retry_policies.rs` | `read_cross_region_retry_on_500` | Failover to satellite region |
| `cosmos_multi_write_fault_injection.rs` | `fault_injection_read_unaffected_by_create_rule` | Endpoint matches hub |
| `cosmos_multi_write_fault_injection.rs` | `fault_injection_read_region_retry_503` | Failover to satellite region |
| `cosmos_multi_write_fault_injection.rs` | `fault_injection_read_session_retry_404_1002` | Failover to hub region |
| `cosmos_multi_write_fault_injection.rs` | `fault_injection_read_connection_error_failover` | Failover to satellite region |
| `cosmos_multi_write_fault_injection.rs` | `fault_injection_read_response_timeout_retries_to_satellite` | Failover to satellite region |
| `cosmos_multi_write_fault_injection.rs` | `fault_injection_connection_error_local_retry_succeeds` | Stays on hub (no failover) |

## Files Changed

| File | Change |
| --- | --- |
| `azure_data_cosmos_driver/src/options/operation_options.rs` | Added `custom_headers` field + setter/getter |
| `azure_data_cosmos_driver/src/driver/pipeline/operation_pipeline.rs` | Wired custom headers into request construction |
| `azure_data_cosmos_driver/src/models/partition_key.rs` | Made `PartitionKeyValue` `pub` |
| `azure_data_cosmos_driver/src/models/mod.rs` | Re-exported `PartitionKeyValue` |
| `azure_data_cosmos/src/driver_bridge.rs` | **New** — shared conversion module |
| `azure_data_cosmos/src/clients/container_client.rs` | Added `driver`/`container_ref` fields; rewrote `read_item` |
| `azure_data_cosmos/src/models/cosmos_response.rs` | Made `request` field optional |
| `azure_data_cosmos/src/partition_key.rs` | Added `into_driver_partition_key()` |
| `azure_data_cosmos/src/options/mod.rs` | Added `pub(crate)` accessors for bridge |
| `azure_data_cosmos/src/pipeline/mod.rs` | Updated `CosmosResponse::new` call site |
| `azure_data_cosmos/src/lib.rs` | Registered `mod driver_bridge` |

## Open Items and Future Work

- **Options alignment:** Ashley is working on aligning SDK options with the driver's options model. Once complete, the `ItemOptions` → `OperationOptions` translation may simplify or become unnecessary.
- **PartitionKey unification:** The dual `PartitionKey` types and `into_driver_partition_key()` conversion should be eliminated once the types are unified.
- **`CosmosRequest` removal:** Once all operations are routed through the driver, the `Option<CosmosRequest>` field on `CosmosResponse<T>` can be removed entirely.
- **`custom_headers` review:** The `custom_headers` field on `OperationOptions` was added for feature parity. It may be removed as we analyze which options are truly needed at the driver level.
- **Remaining operations:** `create_item`, `delete_item`, `replace_item`, `upsert_item`, `patch_item`, and query operations should follow the same pattern established here.

## Fault Injection Wiring

When cutting `read_item` over to the driver, the SDK's fault injection tests initially failed because the two execution paths (gateway and driver) have **independent fault injection systems**. This section documents how they were connected.

### Problem

The SDK and driver each have their own fault injection module (`azure_data_cosmos::fault_injection` and `azure_data_cosmos_driver::fault_injection`). They define parallel but separate types (`FaultInjectionRule`, `FaultInjectionCondition`, `FaultInjectionResult`, etc.) with identical variants but different Rust types. Prior to this work, only the gateway pipeline received fault injection rules — the driver was built without them.

### Solution: Rule Translation with Shared State

The bridge module (`driver_bridge.rs`) includes `sdk_fi_rules_to_driver_fi_rules()`, which translates SDK fault injection rules into driver fault injection rules. The translation covers:

- `FaultOperationType` — variant-by-variant match (identical variant names)
- `FaultInjectionErrorType` — variant-by-variant match
- `FaultInjectionCondition` — `RegionName` → `Region`, operation type and container ID mapped directly
- `FaultInjectionResult` — `Duration` → `Option<Duration>`, probability copied
- Timing fields — `start_time: Instant` → `Option<Instant>`, `end_time` and `hit_limit` copied

### Shared Mutable State

SDK `FaultInjectionRule` has `enabled: Arc<AtomicBool>` and `hit_count: Arc<AtomicU32>` that tests mutate at runtime (`.disable()`, `.enable()`, `.hit_count()`). The driver's `FaultInjectionRuleBuilder` accepts external `Arc`s via `with_shared_state()`, so both the SDK gateway path and the driver path reference the **same atomic state**. This means:

- Calling `.disable()` on the SDK rule also disables it in the driver
- Hit counts are shared — both paths increment the same counter
- Tests that toggle rules or assert hit counts work correctly across both paths

### Wiring in `CosmosClientBuilder`

In `CosmosClientBuilder::build()`:

1. Before the `FaultInjectionClientBuilder` is consumed for the gateway transport, `rules()` extracts a reference to the SDK rules
2. `sdk_fi_rules_to_driver_fi_rules()` translates them to driver rules with shared state
3. The translated rules are passed to `CosmosDriverRuntimeBuilder::with_fault_injection_rules()`
4. The SDK's `fault_injection` Cargo feature now forwards to the driver's `fault_injection` feature

### Test Patterns for Future Cutover

When cutting over additional operations, **no additional fault injection wiring is needed** — it's handled once at the `CosmosClientBuilder` level. However, tests need to account for two behavioral differences:

**`request_url()` returns `None` for driver-routed operations:**

```rust
// Gateway-routed operations return Some(url)
// Driver-routed operations return None
if let Some(url) = response.request_url() {
    assert_eq!(url.host_str().unwrap(), expected_endpoint);
}
```

**Hit-count asymmetry between gateway and driver paths:**

The driver retries certain errors internally (e.g., 500 on reads triggers up to 3 failover retries). Each retry attempt evaluates fault injection rules independently, so a single SDK-level `read_item` call can consume up to **4 fault injection hits** (initial + 3 retries). In contrast, the gateway path typically consumes 1 hit per SDK call.

When writing `hit_limit`-based tests for driver-routed operations, multiply the expected hits per call by the driver's retry budget:

```rust
// Each read_item call consumes up to 4 hits (1 initial + 3 failover retries).
// For 2 calls to fail: 2 × 4 = 8 hits.
let rule = FaultInjectionRuleBuilder::new("test", error)
    .with_hit_limit(8)  // not 2 or 4
    .build();
```

This asymmetry will disappear once all operations are driver-routed, since there will be only one hit-counting path.

### `custom_response` Translation

Translation of `CustomResponse` (synthetic HTTP responses) is not yet implemented. None of the current tests use custom responses for `ReadItem` operations. When needed, the bridge function should be extended to translate `CustomResponse` fields (`status_code`, `headers`, `body`).

### Consolidating to Driver Fault Injection After Cutover

The current dual-system architecture (SDK fault injection + driver fault injection + translation bridge) exists only because the cutover is incremental — some operations still go through the gateway while others go through the driver. Once **all** operations are routed through the driver:

1. **Drop `azure_data_cosmos::fault_injection`** — the SDK's HTTP-client-level fault interception module becomes unreachable. Delete the entire `src/fault_injection/` directory.
2. **Re-export driver types** — the SDK re-exports the driver's fault injection types directly:

   ```rust
   #[cfg(feature = "fault_injection")]
   pub use azure_data_cosmos_driver::fault_injection;
   ```

3. **Remove the translation layer** — `sdk_fi_rules_to_driver_fi_rules()` in `driver_bridge.rs` and the `shared_enabled()`/`shared_hit_count()` accessors on the SDK rule are no longer needed.
4. **Simplify `CosmosClientBuilder`** — `with_fault_injection()` accepts `Vec<Arc<driver::FaultInjectionRule>>` directly and passes them to `CosmosDriverRuntimeBuilder::with_fault_injection_rules()`. No translation, no cloning, no intermediary builder.
5. **Update tests** — tests construct driver `FaultInjectionRule` directly (same builders, same API) instead of SDK rules.

At that point the SDK has **no fault injection logic of its own** — it's a pass-through to the driver, matching the overall "SDK as thin wrapper" goal. The driver is the single source of truth for all transport-related concerns including fault injection.

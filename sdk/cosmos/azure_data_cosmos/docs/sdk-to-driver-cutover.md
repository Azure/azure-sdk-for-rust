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

We chose **not** to take that approach. In this design, the driver is **required**:

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
3. **Translate options:** Reuse `item_options_to_operation_options()` from `driver_bridge.rs`. For write-specific options (e.g., `content_response_on_write_enabled`), extend the bridge function.
4. **Execute:** Call `self.driver.execute_operation(operation, driver_options).await?`.
5. **Bridge response:** Reuse `driver_response_to_cosmos_response(driver_response)`.

The public method signature should not change.

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

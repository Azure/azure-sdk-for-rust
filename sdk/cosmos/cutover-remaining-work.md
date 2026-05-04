# Cosmos SDK-to-Driver Cutover: Remaining Work

This document tracks the remaining work to fully cut over the `azure_data_cosmos` SDK
to use `azure_data_cosmos_driver` for all operations, and remove the SDK's legacy
infrastructure.

## Current State (after PR #4342)

PR #4342 routes `read_feed_ranges()` and `feed_range_from_partition_key()` through the
driver's partition key range cache. The SDK-layer cache is now **unused** but still present
in the codebase.

## PR Split

### PR 1: PKRange cache cutover ✅ (PR #4342)

Routes PKRange resolution through the driver. No deletions — just rewires existing SDK
methods to delegate to the driver.

**Files changed:**

- `azure_data_cosmos_driver/src/driver/cosmos_driver.rs` — added `resolve_all_partition_key_ranges()` and `resolve_partition_key_ranges_for_key()`
- `azure_data_cosmos/src/clients/container_client.rs` — rewrote `read_feed_ranges()` and `feed_range_from_partition_key()`
- `azure_data_cosmos/src/feed_range.rs` — removed dead adapter methods

---

### PR 2: Dead code removal (after PR 1 merges)

Pure deletion of SDK-layer infrastructure that has no remaining callers.

**Files to delete:**

| File | What it was |
|------|-------------|
| `routing/partition_key_range_cache.rs` | SDK-layer PKRange cache (fetched/stored mappings) |
| `routing/collection_routing_map.rs` | In-memory routing map (FeedRange → PartitionKeyRangeId) |
| `routing/partition_key_range.rs` | SDK-layer PKRange model + parsing |
| `routing/range.rs` | Range<T> type for EPK comparisons |
| `routing/service_identity.rs` | Collection identity tracking for cache invalidation |
| `routing/global_partition_endpoint_manager.rs` | PPAF/PPCB retry coordinator (~2,400 lines) |
| `handler/container_connection.rs` | Wrapper pairing GatewayPipeline + container metadata |
| `handler/retry_handler.rs` | SDK-layer retry orchestrator |
| `handler/mod.rs` | Module declaration |
| `pipeline/mod.rs` (gut contents) | `GatewayPipeline` — SDK's HTTP dispatch layer |
| `retry_policies/mod.rs` | Retry policy types/enums |
| `retry_policies/client_retry_policy.rs` | Cross-region retry policy (~1,800 lines) |
| `retry_policies/metadata_request_retry_policy.rs` | Metadata request retry logic |
| `retry_policies/resource_throttle_retry_policy.rs` | 429 throttle backoff policy |

**Other changes needed:**

- `clients/mod.rs`: Remove `pipeline` and `global_partition_endpoint_manager` from `ClientContext`, add `endpoint: Url`
- `clients/cosmos_client_builder.rs`: Remove `GatewayPipeline` creation, `GlobalPartitionEndpointManager` init
- `clients/cosmos_client.rs`: Use `self.context.endpoint` instead of `self.context.pipeline.endpoint`
- `routing/mod.rs`: Remove deleted module declarations
- `lib.rs`: Remove `handler`, `retry_policies` module declarations
- `request_context.rs`: Remove `resolved_partition_key_range` field
- Add `#[allow(dead_code)]` to types still referenced by `GlobalEndpointManager`/`LocationCache` (which stay for background account refresh)

---

### PR 3: FaultClient simplification (after PR 2)

Remove the SDK-layer `FaultClient` HTTP interceptor. This is a **breaking public API change**.

**Files to delete:**

| File | What it was |
|------|-------------|
| `fault_injection/http_client.rs` | SDK `FaultClient` — wrapped HTTP transport to intercept requests |

**Other changes:**

- `fault_injection/client_builder.rs`: Remove `build()` and `with_inner_client()` methods (breaking)
- `fault_injection/mod.rs`: Remove `mod http_client;`
- `clients/cosmos_client_builder.rs`: Stop wrapping transport in `FaultClient`; just use plain transport for `GlobalEndpointManager`'s pipeline
- `fault_injection/rule.rs`: Mark `increment_hit_count()` and `record_passthrough_status()` as `#[allow(dead_code)]`
- `CHANGELOG.md`: Document breaking change

**Public API impact:**

- `FaultInjectionClientBuilder::build() -> Transport` — removed
- `FaultInjectionClientBuilder::with_inner_client()` — removed
- Users already pass the builder to `CosmosClientBuilder::with_fault_injection()` which handles everything internally, so these methods were only needed for advanced/direct usage

---

## Test Coverage Gaps

The deleted SDK files contained ~3,500 lines of unit tests. The driver has integration/E2E
coverage but lacks fine-grained unit tests for the equivalent logic.

### Coverage comparison

| Category | Deleted SDK tests | Driver coverage | Gap severity |
|----------|-------------------|-----------------|--------------|
| 410 Gone / partition split retry | ~15 unit tests | 3 integration tests (emulator) | ⚠️ Medium |
| Cross-region failover | ~20 unit tests | 6 integration tests (multi-region) | ⚠️ Medium |
| 429 throttle backoff | ~10 unit tests (timing/math) | **None explicit** | 🔴 High |
| Session consistency retry | ~5 unit tests | Smoke test only (TODO) | 🔴 High |
| PKRange cache invalidation | ~15 unit tests (concurrency) | 2 integration tests | ⚠️ Medium |
| Routing map / EPK lookups | ~20 unit tests (binary search) | Implicit via E2E | 🔴 High |
| Range comparison logic | ~15 unit tests | **None** | 🔴 High |
| PPAF/PPCB state machine | ~30 unit tests | 6 integration tests | ⚠️ Medium |

### Recommendation

Before or alongside PR 2, consider adding driver unit tests for:

1. **429 backoff math** — verify exponential backoff timing, max delay cap, retry-after header
2. **Range comparison** — verify `contains()`, `overlaps()`, min/max EPK boundary logic
3. **Session consistency** — verify retry on ReadSessionNotAvailable
4. **Routing map lookups** — verify EPK → PKRange resolution with splits

The integration tests provide confidence that the happy paths work end-to-end, but the
unit tests would catch regressions in edge-case logic.

---

## Implementation Notes

- `GlobalEndpointManager` and `LocationCache` **stay** — they handle background account property refresh
- `cosmos_request.rs`, `operation_context.rs`, `request_context.rs` stay with `#[allow(dead_code)]` because `GlobalEndpointManager` references them
- `pipeline/authorization_policy.rs` and `pipeline/cosmos_headers_policy.rs` stay — used by `GlobalEndpointManager`'s azure-core Pipeline
- The driver's `FaultClient` handles fault injection for all data-plane operations
- `driver_bridge.rs` with `sdk_fi_rules_to_driver_fi_rules()` stays — translates user-configured SDK rules to driver format

---

## Timeline

- **PR 1** — ready for review now
- **PR 2** — can be prepared immediately after PR 1 merges (changes are stashed locally)
- **PR 3** — depends on PR 2; small and focused
- **Test gap coverage** — can be done in parallel or as a follow-up

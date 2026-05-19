# Release History

## 0.3.0 (Unreleased)

### Features Added

- Added `ResponseBody`, a typed response body enum that explicitly distinguishes single-payload responses (`ResponseBody::Bytes(Bytes)` — point reads/writes, batches) from feed responses (`ResponseBody::Items(Vec<Bytes>)` — reserved for a future zero-copy split of the `Documents` array on Query / ChangeFeed; today the pipeline still emits `Bytes` for those responses). Provides `from_bytes` / `from_items` constructors, plus `single()`, `single_item::<T>()`, `into_items::<T>()`, and `into_string()` helpers. Used as the body type on `CosmosResponse` and exposed for SDK consumption. `is_empty()` reflects whether a payload exists at all (a feed of zero documents is empty; a feed of one or more entries is not).
- Added typed `CosmosRequestHeaders` fields for known query / changefeed headers: `max_item_count: Option<i32>`, `incremental_feed: bool`, `populate_index_metrics: Option<bool>`, `populate_query_metrics: Option<bool>`, and `enable_cross_partition_query: bool`. `write_to_headers` emits the corresponding wire headers (`x-ms-max-item-count`, `A-IM`, `x-ms-cosmos-populateindexmetrics`, `x-ms-documentdb-populatequerymetrics`, `x-ms-documentdb-query-enablecrosspartition`) only when explicitly set, removing the need to pass them via `custom_headers`. The `populate_*` fields use `Option<bool>` (rather than `bool`) so consumers can distinguish "caller already chose" from "caller did not say" with the same merge semantics as `max_item_count`.
- The operation pipeline now auto-emits `x-ms-documentdb-isquery: True` and `Content-Type: application/query+json` for `OperationType::Query`, matching the existing behavior for `Upsert` and `Batch`.
- `CosmosStatus` now implements `PartialEq<StatusCode>` (and the reverse), `From<CosmosStatus> for StatusCode`, and `From<CosmosStatus> for u16`, plus a `CosmosStatus::new(StatusCode)` constructor. This lets callers compare and convert without reaching for internal fields.

- Added support for the `x-ms-cosmos-hub-region-processing-only` request header on retries after a `404 / 1002 (READ_SESSION_NOT_AVAILABLE)` response on single-master data-plane Cosmos operations. The header asks the backend to route only to a region that has caught up to the requested LSN, reducing the chance of a follow-up retry hitting a region whose session is also behind. The header is scoped to single-master accounts (multi-master accounts already have a different recovery path) and to data-plane operations (metadata-pipeline operations are out of scope per the design spec). Once latched on the first 1002 within an operation, the header is emitted on every subsequent retry for that operation. ([#4389](https://github.com/Azure/azure-sdk-for-rust/pull/4389))
- Added local query-plan generator scaffolding under `crate::query` (lexer, parser, AST, planner, and in-memory evaluator). The scaffolding is **not wired into the production query path** yet — production callers still issue Gateway query-plan requests via `CosmosOperation::query_plan`. The `__internal_testing` cargo feature exposes `query::__test_only_generate_query_plan_for_pk_paths`, `query::__TEST_ONLY_SUPPORTED_QUERY_FEATURES`, and `CosmosOperation::query_plan` for cross-crate gateway-comparison tests; this feature is intentionally unstable and **not covered by SemVer**.
- Added per-partition automatic failover (PPAF) for writes on single-master accounts. On 403/3 WriteForbidden, 503 ServiceUnavailable, 429/3092 SystemResourceUnavailable, 410/1022 Gone, or 408 RequestTimeout from a region, the affected partition is failed over to the next preferred region; subsequent writes for that partition skip the failed region. ([#4156](https://github.com/Azure/azure-sdk-for-rust/pull/4156))
- Added per-partition circuit breaker (PPCB) for reads (any account) and writes (multi-master accounts). Tracks failure counts per `(partition_key_range_id, region)` and routes to an alternate region once the threshold (default 10 reads, 5 writes) is exceeded. A background failback loop probes the original region for recovery. ([#4156](https://github.com/Azure/azure-sdk-for-rust/pull/4156))
- Added `OperationOptions` fields for tuning PPCB: `circuit_breaker_failure_count_for_reads`, `circuit_breaker_failure_count_for_writes`, `circuit_breaker_timeout_counter_reset_window_in_minutes`, `allowed_partition_unavailability_duration_in_seconds`, `ppcb_stale_partition_unavailability_refresh_interval_in_seconds`, and `per_partition_circuit_breaker_enabled` (each also configurable via the corresponding `AZURE_COSMOS_*` environment variable). ([#4156](https://github.com/Azure/azure-sdk-for-rust/pull/4156))

### Breaking Changes

- Slimmed the cached `PartitionKeyRange` to six fields, dropping eight metadata fields the routing-map cache never reads (`resource_id`, `self_link`, `etag`, `timestamp`, `rid_prefix`, `target_throughput`, `lsn`, `owned_archival_pk_range_ids`). The struct now retains the four fields the routing layer consults (`id`, `min_inclusive`, `max_exclusive`, `status`) plus `throughput_fraction` and `parents`, kept on the cached representation for downstream consumers that read them directly. As part of this change, `PartialEq` and `Hash` no longer hash `resource_id`: two ranges with the same `id` / `min_inclusive` / `max_exclusive` are now equal regardless of their `_rid`. Internal callers never used `PartitionKeyRange` as a hash-map key, but downstream consumers that did so should review their assumptions. Service responses are unchanged on the wire — the dropped JSON fields are silently ignored by serde on deserialization. ([#4393](https://github.com/Azure/azure-sdk-for-rust/pull/4393))
- Changed `CosmosResponse::diagnostics()` to return `Arc<DiagnosticsContext>` instead of `&DiagnosticsContext`. The returned `Arc` derefs transparently for read-only inspection (existing call patterns like `response.diagnostics().activity_id()` continue to work), but bindings of the form `let d = response.diagnostics();` now own a cloned `Arc` handle rather than a borrow — letting callers retain operation diagnostics across `into_body()`. Replaces the additive `CosmosResponse::diagnostics_arc()` accessor introduced earlier in this preview cycle.
- Removed `CosmosResponse::body() -> &[u8]`. The previous accessor panicked on multi-item feed bodies, which is unsafe for a public API. The non-consuming `body_parts() -> &ResponseBody` accessor has been renamed to `body()`. Callers needing borrowed access should pattern-match on `ResponseBody::Bytes(b)` / `ResponseBody::Items(items)`; consuming callers can use `into_body().single_item::<T>()` or `into_body().single()`.
- Removed `ResponseBody::as_contiguous_bytes()`, `ResponseBody::into_bytes()`, and `CosmosResponse::into_bytes()`. These helpers silently concatenated feed items into a single buffer, which is semantically wrong (feed items are independent JSON documents, not pieces of a larger byte stream). Callers should explicitly handle the `ResponseBody::Bytes` and `ResponseBody::Items` variants based on the operation type, or use `ResponseBody::single()` for single-payload responses. `ResponseBody::into_string()` now returns an error for feed bodies instead of concatenating items.

### Bugs Fixed

- `CosmosResponseHeaders` now parses `x-ms-offer-replace-pending` case-insensitively (`true` / `True` / `TRUE` and `false` / `False` / `FALSE` are all accepted). Previously the field used strict `bool::FromStr` parsing, which would silently drop Pascal-case values the service may emit and cause the throughput-replace poller to treat in-progress replacements as completed.
- PPCB now records every 5xx failure for the affected partition, including the final failure that exhausts the failover retry budget. Previously the budget-exhausted abort path skipped emitting `MarkPartitionUnavailable`, causing the most diagnostic failure to be silently dropped from PPCB's per-partition counter. ([#4156](https://github.com/Azure/azure-sdk-for-rust/pull/4156))

### Other Changes

- The PPCB failback sweep now applies a per-entry random jitter (uniform in `[0, partition_unavailability_duration / 2)`) before transitioning an `Unhealthy` entry to `ProbeCandidate`. This spreads the failback of partitions that failed in the same burst across the failback window, mitigating a thundering-herd effect on the recovering region. ([#4156](https://github.com/Azure/azure-sdk-for-rust/pull/4156))
- The `reqwest` feature now also enables the crate's `tokio` feature, ensuring the partition failback background loop is compiled in when using the reqwest transport. ([#4156](https://github.com/Azure/azure-sdk-for-rust/pull/4156))

## 0.2.0 (2026-04-24)

### Features Added

- Added `item_lsn` field to `CosmosResponseHeaders` for the `x-ms-item-lsn` response header.
- Added `partition_key_range_id` and `internal_partition_id` fields to `CosmosResponseHeaders` for the `x-ms-documentdb-partitionkeyrangeid` and `x-ms-cosmos-internal-partition-id` response headers. ([#4278](https://github.com/Azure/azure-sdk-for-rust/pull/4278))
- Added `rustls` feature flag (enabled by default) that configures reqwest with rustls as the TLS stack. ([#4252](https://github.com/Azure/azure-sdk-for-rust/pull/4252))
- Added `native_tls` feature flag that configures reqwest with native-tls as the TLS stack. Disable default features and enable `native_tls` to use the platform TLS stack. ([#4252](https://github.com/Azure/azure-sdk-for-rust/pull/4252))
- Added `SessionToken::merge()` for merging two session tokens by partition key range ID. ([#4214](https://github.com/Azure/azure-sdk-for-rust/pull/4214))

## 0.1.0 (2026-04-09)

### Features Added

- Initial release of `azure_data_cosmos_driver` (core Cosmos DB protocol implementation for cross-language SDK reuse). ([#3772](https://github.com/Azure/azure-sdk-for-rust/pull/3772) and [#3592](https://github.com/Azure/azure-sdk-for-rust/pull/3592))
- Added cache priming via `CosmosDriver::initialize()` and `CosmosDriver::prime_container()` to avoid cold-start latency on the first data-plane operation. ([#3864](https://github.com/Azure/azure-sdk-for-rust/pull/3864))
- Added response metadata fields (`index_metrics`, `query_metrics`, `server_duration_ms`, `lsn`) to `CosmosResponseHeaders` and `RequestDiagnostics`, with base64 decoding for `index_metrics`. ([#3960](https://github.com/Azure/azure-sdk-for-rust/pull/3960))
- Added hierarchical partition key (MultiHash) support. ([#4087](https://github.com/Azure/azure-sdk-for-rust/pull/4087))


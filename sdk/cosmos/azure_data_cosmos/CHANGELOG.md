# Release History

## 0.38.0 (Unreleased)

### Features Added

### Breaking Changes

### Bugs Fixed

- Fixed `ContainerClient::read_feed_ranges` failing on containers with large partition counts by fully draining routing metadata and retrying missing cache results with a forced refresh. ([#4845](https://github.com/Azure/azure-sdk-for-rust/pull/4845))

### Other Changes

## 0.37.0 (2026-07-20)

### Features Added

- Added runtime diagnostics output configuration via `CosmosRuntimeBuilder::with_diagnostics_options`, including the env-backed `AZURE_COSMOS_DIAGNOSTICS_DEFAULT_VERBOSITY` option. The built-in default is now summary diagnostics JSON. ([#4733](https://github.com/Azure/azure-sdk-for-rust/pull/4733))
- Gateway 2.0 transport (a regional proxy forwarding RNTBD-over-HTTP/2) is selected automatically when the account advertises thin-client endpoints, the connectivity probe confirms them, and the runtime has not opted out. ([#4319](https://github.com/Azure/azure-sdk-for-rust/pull/4319), [#4803](https://github.com/Azure/azure-sdk-for-rust/pull/4803))
- Added the re-exported `ConnectionPoolOptions::gateway_v2_disabled` and `ConnectionPoolOptionsBuilder::with_gateway_v2_disabled`, with `AZURE_COSMOS_CONNECTION_POOL_GATEWAY_V2_DISABLED` configuration and an environment-only `_OVERRIDE` incident switch. ([#4763](https://github.com/Azure/azure-sdk-for-rust/pull/4763))
- HTTP 449 (RetryWith) responses are now retried transparently in-region with exponential backoff, so callers no longer see spurious 449 errors from concurrent writes. ([#4319](https://github.com/Azure/azure-sdk-for-rust/pull/4319))
- `ReadConsistencyStrategy` is now honored across Gateway V1 and V2 reads. Adds the `LatestCommitted` variant (a quorum read independent of the account default); `GlobalStrong` is rejected with `BadRequest` unless the account default is `Strong`. Per-request strategy overrides the client default. ([#4319](https://github.com/Azure/azure-sdk-for-rust/pull/4319))
- Added preview distributed transaction SDK builders, patch operations, diagnostics, and response accessors behind the disabled-by-default `preview_dtx` feature. ([#4702](https://github.com/Azure/azure-sdk-for-rust/pull/4702))
- Added change feed pull support via `ContainerClient::query_change_feed()`, which takes a required `ChangeFeedStartFrom` start position (`Beginning`, `Now`, `PointInTime`) and returns a `ChangeFeedPageIterator<T>` that streams `FeedPage<T>` results. New `feed` types `ChangeFeedPageIterator`, `FeedScope`, and `ContinuationToken`, plus `options` types `ChangeFeedOptions` and `ChangeFeedMode` (currently `LatestVersion`); supports single-partition, per-partition-key, and full-container (cross-partition fan-out) reads with continuation-token resumption that persists the original start position so never-polled partitions don't replay history on resume. ([#4621](https://github.com/Azure/azure-sdk-for-rust/pull/4621))
- Change feed items are now surfaced as an envelope. `ContainerClient::query_change_feed::<YourDoc>()` yields `ChangeFeedItem<YourDoc>`, binding the envelope into the return type so the post-change document is read via `ChangeFeedItem::current()` and cannot be silently deserialized away. The envelope also exposes the pre-change document (`previous()`) and per-change `metadata()` (populated by full-fidelity reads; absent for `LatestVersion`). A full-fidelity delete returns an empty `current` object, which maps to `None` so callers with strict document types still deserialize the delete; the deleted item's identity is available via `ChangeFeedMetadata::id()` and `ChangeFeedMetadata::partition_key()`. `ChangeFeedOperationType` includes an `Unknown` catch-all so a future operation type cannot fail a page and stall the feed. A backend that does not envelope change feed items (such as the Cosmos emulator) returns the bare document, which is mapped onto `current()` so no data is lost. Added the `models` types `ChangeFeedItem<T>`, `ChangeFeedMetadata`, `ChangeFeedOperationType`, and `LogicalSequenceNumber`. ([#4723](https://github.com/Azure/azure-sdk-for-rust/pull/4723))
- Added `TlsBackend` (re-exported) and a `tls_backend` option on `ConnectionPoolOptions` (`ConnectionPoolOptionsBuilder::with_tls_backend`), defaulting to `TlsBackend::Rustls`, available under the `rustls` feature, to pin the TLS backend used by the transport. This is additive and changes no behavior for the default (rustls) build; it only has an effect in builds that compile in multiple reqwest TLS backends, where reqwest would otherwise default to native-tls and the driver now pins rustls instead. ([#4649](https://github.com/Azure/azure-sdk-for-rust/pull/4649))

### Bugs Fixed

- Improved HTTP 429 (throttle) retry handling: data-plane operations now default to 18 retries with a per-retry interval clamped to 15s (~270s cumulative, so the retry count is the limiter), previously 9 retries / 30s; metadata stays at 9 retries / 30s with a 5s per-retry interval, and the bootstrap account-properties fetch now retries 429 (previously it had none). Both remain configurable through `ThrottlingRetryOptions`. ([#4758](https://github.com/Azure/azure-sdk-for-rust/pull/4758))
- `http://` (non-HTTPS) account endpoints are now rejected unless the host is a known Cosmos DB emulator host, failing fast during client construction with an "invalid account endpoint" error instead of attempting an insecure connection to a production account. This validation also covers configured backup endpoints. ([#4757](https://github.com/Azure/azure-sdk-for-rust/pull/4757))
- Fixed hierarchical-partition-key (HPK) queries. A `FeedScope::partition` scope with only a *prefix* of the key hierarchy (e.g. `("USA", "CA")` on a `/country/state/city` container) now filters to that prefix instead of returning every item in the physical partition (issue [#4680](https://github.com/Azure/azure-sdk-for-rust/issues/4680)), and cross-partition queries over an HPK container no longer fail with `400 Bad Request` (issue [#4681](https://github.com/Azure/azure-sdk-for-rust/issues/4681)). ([#4729](https://github.com/Azure/azure-sdk-for-rust/pull/4729))
- Fixed the `AZURE_COSMOS_PPCB_*` environment variables (including `AZURE_COSMOS_PPCB_ENABLED` and the `AZURE_COSMOS_PPCB_ENABLED_OVERRIDE` kill switch) being ignored when a `CosmosClient` was built without calling `CosmosClientBuilder::with_partition_failover_options`. The per-partition circuit breaker (PPCB) stayed enabled even with `AZURE_COSMOS_PPCB_ENABLED=false`. The client's driver now resolves these options from the environment when they are not supplied explicitly. ([#4655](https://github.com/Azure/azure-sdk-for-rust/pull/4655))

## 0.36.0 (2026-06-19)

### Features Added

- Derived `SafeDebug` on `CosmosCredential`, `ItemResponse`, `ResourceResponse<T>`, and `BatchResponse`. ([#4512](https://github.com/Azure/azure-sdk-for-rust/pull/4512))
- Added standard derives (`Clone`, `Copy`, `PartialEq`, `Eq`, `Hash`, `Serialize`, `Deserialize`) to `ConsistencyLevel` and `RoutingStrategy`. ([#4512](https://github.com/Azure/azure-sdk-for-rust/pull/4512))
- `Query::with_text` now accepts `impl Into<String>`. ([#4512](https://github.com/Azure/azure-sdk-for-rust/pull/4512))
- Exposed `CosmosRuntime` and a runtime-aware `CosmosClientBuilder`, splitting the Cosmos client into a per-process runtime (transport / cert / proxy / UA defaults) and per-client driver (operation defaults, fault injection, throughput-control groups), and re-exporting the driver's options surface from `azure_data_cosmos::options`:
  - New `CosmosRuntime` and `CosmosRuntimeBuilder` types. A default process-wide runtime is initialized lazily; users can configure their own runtime through `CosmosRuntimeBuilder` and attach it via `CosmosClientBuilder::with_runtime`. The runtime builder exposes:
    - `with_connection_pool(ConnectionPoolOptions)` — runtime-wide transport / cert / proxy settings.
    - `with_default_operation_options(OperationOptions)` — runtime-default `OperationOptions`.
    - `with_user_agent_suffix(UserAgentSuffix)` — runtime-default User-Agent suffix.
    - `with_cpu_refresh_interval(Duration)` — diagnostics sampler interval.
    - `build()` — auto-applies an `azsdk-rust-cosmos/<crate-version>` wrapping SDK identifier so wire User-Agent strings always advertise the SDK alongside any custom suffix.
  - New per-client setters on `CosmosClientBuilder`:
    - `with_runtime(CosmosRuntime)` — attach an explicit runtime; when not set, `build()` resolves `CosmosRuntime::global()` lazily.
    - `with_default_operation_options(OperationOptions)` — client-level default `OperationOptions` (overrides runtime defaults; overridden by per-call options).
    - `with_partition_failover_options(PartitionFailoverOptions)` — configures the driver's per-partition circuit-breaker / failover tuning for this client; when unset, the driver falls back to `PartitionFailoverOptions::default()`, which honors the `AZURE_COSMOS_PPCB_*` environment variables.
    - `with_fault_injection_rules(Vec<Arc<FaultInjectionRule>>) -> Result<Self>` — registers fault-injection rules on this specific client (gated on `fault_injection`).
    - `register_throughput_control_group(ThroughputControlGroupOptions) -> Result<Self>` — registers a throughput-control group for this client's driver.
  - New re-exports from `azure_data_cosmos::options` (so users configuring a custom runtime don't have to take a direct dependency on the driver crate): `ConnectionPoolOptions`, `ConnectionPoolOptionsBuilder`, `ServerCertificateValidation`, `PartitionFailoverOptions`, `PartitionFailoverOptionsBuilder`, `ThroughputControlOptions`, `ThroughputControlOptionsBuilder`, and `ThroughputControlOptionsView`.
  - New nested `OperationOptions::throughput_control` group lets callers set `throughput_bucket` and `priority_level` per request without first registering a throughput-control group; registered groups are still consulted as fallbacks through `ThroughputControlOptions::group_name`. (See the driver CHANGELOG for the full per-field layering and header-emission rules.)

### Breaking Changes

- Reorganized the public API: types are now grouped under `models`, `diagnostics`, `feed`, and `options`; the `query`, `regions`, and `routing_strategy` modules were removed; the previously `#[doc(hidden)]` feature-gated builder methods on `CosmosClientBuilder` are now visible (and remain feature-gated); `PartitionKey::EMPTY`, its `Default` impl, and `From<()> for PartitionKey` were removed (use the query/feed APIs for cross-partition operations); and `ETag` is no longer re-exported from `azure_data_cosmos::options` — use `azure_core::http::Etag` directly (construct via `Etag::from(&str)` / `Etag::from(String)`). See the PR for the full list of moves and import paths. ([#4512](https://github.com/Azure/azure-sdk-for-rust/pull/4512))
- Renamed `CosmosClientBuilder::with_operation_options` to `CosmosClientBuilder::with_default_operation_options` to reflect the fact that it specifies defaults for per-operation options rather than actual client-level options. ([#4588](https://github.com/Azure/azure-sdk-for-rust/pull/4588))
- `TransactionalBatch::{create_item, upsert_item, replace_item}` and `TransactionalBatchOperationResult::into_model` now return `azure_data_cosmos::Result<_>` instead of `Result<_, serde_json::Error>`. The underlying `resource_body` is now stored as `Option<Box<serde_json::value::RawValue>>` and exposed via a new `resource_body()` accessor. ([#4512](https://github.com/Azure/azure-sdk-for-rust/pull/4512))
- `DatabaseProperties::id` is now `Option<String>` (previously `String`) to match the wire schema. ([#4512](https://github.com/Azure/azure-sdk-for-rust/pull/4512))
- Partition Circuit Breaker (PPCB) is now ENABLED by default. To disable it, set `PartitionFailoverOptions::circuit_breaker_enabled` to `false` when configuring a `CosmosClient` or set the `AZURE_COSMOS_PPCB_ENABLED` environment variable to `false`. ([#4588](https://github.com/Azure/azure-sdk-for-rust/pull/4588))
- `CosmosClientBuilder` has been slimmed to a runtime-aware surface. Per-runtime concerns (transport, cert validation, proxy, UA defaults) move onto `CosmosRuntime` and are shared across clients; per-client concerns (operation defaults, FI rules, throughput-control groups) stay on the builder ([#4588](https://github.com/Azure/azure-sdk-for-rust/pull/4588)). Migration impact:
  - `with_proxy_allowed` — removed. Move to `CosmosRuntimeBuilder::with_connection_pool(ConnectionPoolOptionsBuilder::new().with_proxy_allowed(true).build())`.
  - `with_throttling_retry_options` — removed. The throttle settings now live on `OperationOptions`; use `with_default_operation_options(OperationOptionsBuilder::new().with_throttling_retry_options(...).build())` (or attach to a `CosmosRuntime` for process-wide defaults).
  - `with_fault_injection` — renamed to `with_fault_injection_rules` and now returns `Result<Self>` to surface duplicate-ID errors at registration time.
  - `with_throughput_control_group` — renamed to `register_throughput_control_group` and now returns `Result<Self>`. Throughput-control groups are now a per-client (driver-level) concept only; `CosmosRuntimeBuilder` does not expose a corresponding registration method.
  - `with_driver_runtime_builder` — replaced by `with_runtime(CosmosRuntime)`. The `__internal_in_memory_emulator` harness builds its runtime via `CosmosRuntimeBuilder::from(driver_builder)` (the `From<CosmosDriverRuntimeBuilder>` escape hatch).
  - The `allow_invalid_certificates` Cargo feature has been removed. The capability is now in the default feature set but requires explicit opt-in via `CosmosRuntimeBuilder::with_connection_pool(ConnectionPoolOptionsBuilder::new().with_server_certificate_validation(ServerCertificateValidation::RequiredUnlessEmulator).build())`. The new `RequiredUnlessEmulator` policy is not a blanket "disable validation" knob — it validates the server certificate normally and only relaxes validation for detected Cosmos DB emulator hosts (via `AccountEndpoint` + `Region` heuristics, or the `AZURE_COSMOS_EMULATOR_HOST` environment variable). See the driver CHANGELOG for the underlying `EmulatorServerCertValidation` → `ServerCertificateValidation` rename.
- Per-account driver caching has been removed from the underlying runtime — each `CosmosClient::build(...)` now constructs a fresh `CosmosDriver`. Clients sharing the same `CosmosRuntime` continue to share transport pools, sampler, account cache, etc.; only the per-account `CosmosDriver` instance is no longer reused. ([#4588](https://github.com/Azure/azure-sdk-for-rust/pull/4588))


### Bugs Fixed

- `403/1008 (DatabaseAccountNotFound)` and `403/3 (WriteForbidden)` now trigger an account-topology refresh and retry against the refreshed endpoints instead of bubbling up. ([#4590](https://github.com/Azure/azure-sdk-for-rust/pull/4590))
- Gateway-mode transport connect failures no longer bump the per-partition circuit breaker counter; only the endpoint-unavailable mark is emitted. ([#4590](https://github.com/Azure/azure-sdk-for-rust/pull/4590))
- `403/3 (WriteForbidden)` and `403/1008 (DatabaseAccountNotFound)` on a PPCB-managed multi-write partition no longer mark the endpoint unavailable; the per-partition counter drives failover so other partitions on the same endpoint keep writing normally. ([#4590](https://github.com/Azure/azure-sdk-for-rust/pull/4590))
- The per-partition circuit breaker override now respects `OperationOptions::excluded_regions`; previously a tripped override could silently route to a region the caller had excluded. ([#4590](https://github.com/Azure/azure-sdk-for-rust/pull/4590))

### Other Changes

- `DatabaseClient::read_throughput` and `begin_replace_throughput` no longer panic in release builds if the service returns an offer without `_rid`; they now return a synthetic `CosmosError`. ([#4512](https://github.com/Azure/azure-sdk-for-rust/pull/4512))
- `azure_data_cosmos::error` is now a public module, and `ContainerClient` / `DatabaseClient` are re-exported at the crate root. ([#4512](https://github.com/Azure/azure-sdk-for-rust/pull/4512))
- Documented that control-plane create/replace methods (`CosmosClient::create_database`, `DatabaseClient::create_container`, `ContainerClient::replace`, and the throughput-replace methods) always return the resource body regardless of `ContentResponseOnWrite`, and pointed `CosmosClient`'s rustdoc at the `CosmosClient::builder()` factory. ([#4512](https://github.com/Azure/azure-sdk-for-rust/pull/4512))

## 0.35.0 (2026-06-09)

### Features Added

- Added `AZURE_COSMOS_HEDGING_ENABLED_OVERRIDE` and `AZURE_COSMOS_PPCB_ENABLED_OVERRIDE` kill-switch environment variables. When set, an override wins over **every** source of that feature's enablement — for hedging, every configuration layer including a hard-coded per-request value and a programmatic `AvailabilityStrategy`; for the per-partition circuit breaker, both the `PartitionFailoverOptions::circuit_breaker_enabled` driver option and the server account property `enable_per_partition_failover_behavior`. This lets operators force hedging or PPCB on/off fleet-wide during a livesite incident without a code change or redeploy. Overrides are inert unless set and should normally be left unset. ([#4562](https://github.com/Azure/azure-sdk-for-rust/pull/4562))
- Added the `AZURE_COSMOS_HEDGING_ENABLED` environment variable as a master switch for cross-region read hedging. Hedging remains **enabled by default**. When set, the variable is the **source of truth** and takes precedence over a programmatic `AvailabilityStrategy` in **both** directions: `false` disables hedging even when an explicit `AvailabilityStrategy::Hedging(..)` is configured, and `true` enables hedging even when an explicit `AvailabilityStrategy::Disabled` is configured (a programmatic `Hedging(..)` still supplies its custom threshold). This applies whether the strategy is set on a request or via `CosmosClientBuilder::with_operation_options`. Leaving it unset defers to the programmatic strategy and keeps the built-in default threshold of `min(1000ms, request_timeout / 2)`. ([#4562](https://github.com/Azure/azure-sdk-for-rust/pull/4562))
- Exposed cross-regional read hedging. Enable it by attaching an `OperationOptions` built with `OperationOptionsBuilder::with_availability_strategy(AvailabilityStrategy::Hedging(HedgingStrategy::new(HedgeThreshold::new(threshold)?)))` to a request (e.g. `ItemReadOptions::with_operation_options`) or to the client defaults via `CosmosClientBuilder::with_operation_options`. The `AvailabilityStrategy`, `HedgingStrategy`, and `HedgeThreshold` types are now re-exported from `azure_data_cosmos`. When enabled, the driver speculatively dispatches the read to a second preferred region after the configured threshold elapses and returns whichever response classifies as final first, cancelling the losing leg structurally (no detached tasks); `AvailabilityStrategy::Disabled` turns hedging off for that scope, and when no strategy is configured the driver applies a built-in default for multi-region reads. ([#4432](https://github.com/Azure/azure-sdk-for-rust/pull/4432))
- Added configurable retry limits for throttled (HTTP 429, rate-limited) requests, mirroring the .NET and Java SDKs' `ThrottlingRetryOptions`. A new nested `ThrottlingRetryOptions` group on `OperationOptions` (field `throttling_retry_options`) carries `max_retry_count` (env `AZURE_COSMOS_MAX_THROTTLE_RETRY_COUNT`, default `9`, `0` disables throttle retries) and `max_retry_wait_time` (default `30s`), settable per-request via `OperationOptions`/`OperationOptionsBuilder` and `ThrottlingRetryOptionsBuilder`. New client-wide setter `CosmosClientBuilder::with_throttling_retry_options(ThrottlingRetryOptions)` forwards the group as runtime-layer defaults. Both budgets apply *per transport-pipeline invocation*, not per logical operation — an operation that fans out across regions (failover, hedging) starts a fresh budget per leg; use `OperationOptions::end_to_end_latency_policy` to bound total per-operation wall-clock time. ([#4544](https://github.com/Azure/azure-sdk-for-rust/pull/4544))

### Bugs Fixed

- Writes to multi-write Cosmos accounts now send the `x-ms-cosmos-allow-tentative-writes: true` request header. Without it, satellite write regions returned `403 / 3 (WriteForbidden)`, breaking write failover to non-primary regions. ([#4500](https://github.com/Azure/azure-sdk-for-rust/pull/4500))

## 0.34.0 (2026-05-29)

### Features Added

- `CosmosError` can capture a stack backtrace on construction. Capture is opt-in (off by default; on when `RUST_BACKTRACE` is set or when explicit capacities are supplied) and protected against error storms by two configurable per-second limiters on the runtime builder. ([#4442](https://github.com/Azure/azure-sdk-for-rust/pull/4442))
- Introduced `azure_data_cosmos::CosmosError` and the crate-wide `azure_data_cosmos::Result<T>` alias, surfacing typed `CosmosStatus` (with predicate accessors like `is_not_found()` / `is_throttled()` / `is_transient()`), the originating `CosmosResponse`, and the operation `DiagnosticsContext` on every failure. `From<CosmosError> for azure_core::Error` is provided so callers using `?` against `azure_core::Error` continue to compose. ([#4442](https://github.com/Azure/azure-sdk-for-rust/pull/4442))
- Added `RoutingStrategy::PreferredRegions` to allow specifying a fixed region preference order for failover, hedging, and retry. ([#4485](https://github.com/Azure/azure-sdk-for-rust/pull/4485))
- Standardized every client-method options type with a public `operation: OperationOptions` field and `with_operation_options(OperationOptions) -> Self` setter, so any per-request `OperationOptions` setting can be configured via any options type. The following options types previously had no way to attach `OperationOptions` and now do: `ReadContainerOptions`, `ReadDatabaseOptions`, `ReplaceContainerOptions`, `CreateContainerOptions`, `CreateDatabaseOptions`, `DeleteContainerOptions`, `DeleteDatabaseOptions`, `QueryContainersOptions`, `QueryDatabasesOptions`, `ThroughputOptions`, `ReadFeedRangesOptions`. For `CreateContainerOptions` / `CreateDatabaseOptions` / `ReplaceContainerOptions`, the SDK still forces `content_response_on_write = Enabled` on the resolved options because control-plane mutations require the response body. `ReadFeedRangesOptions::operation` is currently inert (the underlying routing-map cache does not go through the operation pipeline) but is added for shape consistency with the other options types. ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- Added `new()` constructors and `with_x` consuming setters to multi-required-field model types so callers can build them declaratively without struct-literal syntax (which is now blocked by `#[non_exhaustive]`): `VectorEmbedding::new(path, data_type, dimensions, distance_function)` + `with_path` / `with_data_type` / `with_dimensions` / `with_distance_function`; `ConflictResolutionPolicy::new(mode)` + `with_resolution_path` / `with_resolution_procedure`; `SpatialIndex::new(path)` + `with_type` (singular pusher onto `types`); `CompositeIndexProperty::new(path, order)` + `with_path` / `with_order`; `VectorIndex::new(path, index_type)` + `with_path` / `with_index_type`. These types do **not** implement `Default` — their constructors require values that have no meaningful default. ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- Derived `Default` on `VectorEmbeddingPolicy`, `UniqueKeyPolicy`, `UniqueKey`, `PropertyPath`, and `CompositeIndex`, and added singular `with_x` pushers / setters: `VectorEmbeddingPolicy::with_embedding`, `UniqueKeyPolicy::with_unique_key`, `UniqueKey::with_path`, `PropertyPath::with_path`, and `CompositeIndex::with_property`. This matches the existing `IndexingPolicy::with_included_path` style and lets callers build these policies declaratively without constructing intermediate `Vec`s. ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- Added `QueryFeedPage::as_feed_page()` returning `&FeedPage<T>`, so a query page can be passed to APIs that accept the more general `FeedPage` type. Query-specific metadata (index/query metrics) remains accessible on the `QueryFeedPage` itself. ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- Added `QueryOptions::with_populate_index_metrics(bool)`, `with_populate_query_metrics(bool)`, and `with_max_item_count(MaxItemCountHint)` setters. These replace the previous pattern of passing raw `x-ms-cosmos-populateindexmetrics`, `x-ms-documentdb-populatequerymetrics`, and `x-ms-max-item-count` values through `OperationOptions::with_custom_headers` for query execution. `max_item_count` takes the new `MaxItemCountHint` enum with `ServerDecides` and `Limit(NonZeroU32)` variants, so callers don't have to traffic in the `-1` wire sentinel directly. ([#4401](https://github.com/Azure/azure-sdk-for-rust/pull/4401))
- Added `ContainerClient::patch_item()` for applying JSON-Patch-style mutations to a single item. Supports `add`/`set`/`replace`/`remove`/`increment`/`move` ops via the new `PatchInstructions`/`PatchOperation`/`CosmosNumber` types. Added `PatchItemOptions` for per-request configuration (`max_attempts`, `session_token`, etc.). `PatchItemOptions` intentionally does not expose a `Precondition` or SQL filter predicate — the driver-side PATCH handler owns the internal `If-Match` end-to-end, and predicate evaluation is out of scope for this preview. The method's rustdoc documents the non-idempotent-under-transport-failure caveat. ([#4386](https://github.com/Azure/azure-sdk-for-rust/pull/4386))
- Support for simple cross-partition queries with `SELECT` projections and `WHERE` filters. Cross-partition queries are now done through fan-out in the client, and provide a client-generated continuation token that can be used to resume the query. See `ContainerClient::query_items()` and `FeedScope` for details. ([#4440](https://github.com/Azure/azure-sdk-for-rust/pull/4440))
- Added `QueryOptions::continuation_token` and `QueryOptions::with_continuation_token(...)` for resuming queries from a continuation token. ([#4440](https://github.com/Azure/azure-sdk-for-rust/pull/4440))
- Added a new `FeedOptions` type that wraps the paging knobs (`max_item_count`, `continuation_token`) common to feed-style operations, so future feed APIs (change feed, read-feed, etc.) can adopt the same shape without redeclaring fields. `QueryOptions` now embeds it as a `pub feed: FeedOptions` field with a `with_feed_options(FeedOptions)` setter. The existing `QueryOptions::with_max_item_count` and `QueryOptions::with_continuation_token` setters are retained as convenience functions that mutate the embedded `feed` (they're expected to be used commonly enough to warrant the shortcut). ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))

### Breaking Changes

- All fallible public APIs now return `azure_data_cosmos::Result<T>` (= `Result<T, CosmosError>`) instead of `azure_core::Result<T>`, and the error type was renamed `Error` → `CosmosError` (with `CosmosErrorBuilder` for construction). Categorization moved from a `Kind` enum to predicates on `CosmosStatus` (`is_not_found()`, `is_throttled()`, `is_transient()`, …); the underlying `azure_core::Error` is still reachable via `std::error::Error::source()`. ([#4442](https://github.com/Azure/azure-sdk-for-rust/pull/4442))
- Moved `QueryOptions::max_item_count` and `QueryOptions::continuation_token` into the new `QueryOptions::feed: FeedOptions` field. Callers that read or assign these fields directly should switch to `options.feed.max_item_count` / `options.feed.continuation_token`. The `with_max_item_count` / `with_continuation_token` convenience setters on `QueryOptions` continue to work unchanged. ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- `ThroughputProperties` is now `#[non_exhaustive]` and no longer derives `Default`. The `Default` impl produced a meaningless wire payload (no manual throughput and no autoscale settings, which would send an empty offer body). Callers should use `ThroughputProperties::manual(throughput)` or `ThroughputProperties::autoscale(starting_maximum_throughput, increment_percent)` instead. ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- Marked the following public model enums and response wrappers as `#[non_exhaustive]` to allow future variants/fields to be added without further breaking changes: `VectorDataType`, `VectorDistanceFunction`, `ConflictResolutionMode`, `IndexingMode`, `SpatialType`, `CompositeIndexOrder`, `VectorIndexType`, `BatchResponse`, `ItemResponse`, `ResourceResponse<T>`, `ResponseBody`, `ResponseHeaders`, `PartitionKeyVersion` and `CosmosStatus`. Callers must use `..` wildcard arms in `match`es over these enums and cannot construct these structs via struct-literal syntax (the SDK already provides constructors / setters for the constructable types). ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- `CosmosClientBuilder::build` now takes `AccountReference` directly instead of `impl Into<AccountReference>`. Callers should construct an `AccountReference` explicitly via `AccountReference::with_credential` or `AccountReference::with_authentication_key` and pass it in. ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- Replaced `EffectivePartitionKey::min()` / `EffectivePartitionKey::max()` with associated constants `EffectivePartitionKey::MIN` / `EffectivePartitionKey::MAX`. The inner storage also changed from `String` to `Cow<'static, str>` so the constants can borrow static strings without allocating. Callers should rewrite `EffectivePartitionKey::min()` as `EffectivePartitionKey::MIN.clone()` (or just `&EffectivePartitionKey::MIN` for comparisons). ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- Removed `PartitionKeyValue::undefined()` (use the existing `PartitionKeyValue::UNDEFINED` associated constant instead) and replaced the test-only `PartitionKeyValue::infinity()` constructor with a publicly-available `PartitionKeyValue::INFINITY` associated constant. ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- Removed `FeedRange::can_merge()` and `FeedRange::merge_with()`. These are SDK-internal helpers used only by the session-token coalescing pipeline and are now implemented internally. ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- Renamed `CosmosAccountEndpoint` → `AccountEndpoint` and `CosmosAccountReference` → `AccountReference`. The `Cosmos` prefix is implied by the containing `azure_data_cosmos` crate. ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- Renamed `CosmosAccountReference::with_master_key()` to `with_authentication_key()`. ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- Renamed `FeedPageIterator` → `QueryPageIterator` and `FeedItemIterator` → `QueryItemIterator`. These iterators are only produced by query APIs today; the `Feed*` names are reserved for future non-query feed APIs. ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- Removed the `request_charge()` and `session_token()` convenience accessors from `FeedPage` and `QueryFeedPage`. Use `page.headers().request_charge()` and `page.headers().session_token()` instead — the parsed `ResponseHeaders` already exposes these values and provides full typed access to every other response header. ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- Tightened `AccountReference` constructors. `with_credential` now accepts `impl Into<AccountEndpoint>` instead of a concrete `AccountEndpoint`. The former `with_master_key` is renamed to `with_authentication_key` and now takes `AccountEndpoint` (which has `FromStr` and `From<Url>` impls) and `impl Into<Secret>` for the key. The two `From<(AccountEndpoint, _)>` / `From<(Url, _)>` tuple conversions are removed; construct a `AccountReference` via the named constructors instead. ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- Removed `azure_data_cosmos::ConnectionString` from the public API. The type was a parsing helper not consumed by any public SDK API. Users who still need support for Connection String parsing can parse the connection string themselves and construct an `AccountReference` via the named constructors. ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))
- Refactored the response surface to be SDK-owned. `ItemResponse` drops its type parameter (use `response.into_model::<MyItem>()` or `response.into_body().into_single::<MyItem>()`); `ResourceResponse<T>` keeps its parameter so `.into_model()?` still works without a turbofish. `status()` now returns `CosmosStatus`, `headers()` returns `&ResponseHeaders` (typed accessors only — `etag()`, `request_charge()`, `session_token()`, `continuation()`, `activity_id()`, `substatus()`, `index_metrics()`, `query_metrics()`, `offer_replace_pending()`, `server_duration_ms()`, `lsn()`, `item_lsn()`, `item_count()`, …), and `into_body()` returns the SDK-owned `ResponseBody` enum (`NoPayload` / `Bytes` / `Items`) with `single()`, `items()`, `into_single::<T>()`, `into_items::<T>()`, and `is_empty()` helpers. `FeedPage::headers()` / `QueryFeedPage::headers()` now return `&ResponseHeaders` instead of `&azure_core::http::headers::Headers`. The `ItemResponse::etag()` convenience accessor is removed (use `response.headers().etag()`). `CosmosStatus` is re-exported from the driver and implements `PartialEq<StatusCode>` and `From<CosmosStatus> for StatusCode/u16`, so existing comparisons keep working. ([#4401](https://github.com/Azure/azure-sdk-for-rust/pull/4401))

### Other Changes

- Removed the SDK-side `FaultInjectionClientBuilder`, parallel duplicate types (`FaultInjectionRule`, `FaultInjectionCondition`, `FaultInjectionResult`, `CustomResponse`, the matching builders, `FaultInjectionErrorType`, `FaultOperationType`), and the SDK-side `FaultClient` HTTP wrapper from `azure_data_cosmos::fault_injection`. The module is now a pure re-export of the driver's fault-injection types — fault-injection rules flow directly to the driver runtime and are evaluated by the driver's transport-layer fault-injection client. `CosmosClientBuilder::with_fault_injection` now accepts the driver's `Vec<Arc<FaultInjectionRule>>` directly instead of `FaultInjectionClientBuilder`. Callers should construct rules via the re-exported `FaultInjectionRuleBuilder` and pass the vector.([#4426](https://github.com/Azure/azure-sdk-for-rust/pull/4426))
- Removed the `request_url()` accessor (gated on the `fault_injection` feature) from `ItemResponse`/`ResourceResponse`/`BatchResponse`. Operations never populated it, so it always returned `None` in current usage.
- `CosmosClientBuilder::with_user_agent_suffix` (and `CosmosClientOptions::with_user_agent_suffix`) now take `UserAgentSuffix` instead of `impl Into<String>`. Callers passing a `&str` or `String` must construct the value explicitly via `UserAgentSuffix::new` (panics on invalid input) or `UserAgentSuffix::try_new` (returns `Option`). Validation rules (max 25 characters, HTTP-header-safe) are now enforced at the construction site instead of being applied silently inside the builder. ([#4368](https://github.com/Azure/azure-sdk-for-rust/pull/4368))
- Changed how continuation tokens are returned. Instead of a `continuation()` accessor on `QueryFeedPage` and `FeedPage`, continuation tokens are now returned as a `Option<ContinuationToken>` from the `FeedPageIterator::to_continuation_token(&self)` method. Generating a continuation token for a cross-partition query requires computation, so this change makes it explicit that callers must opt in to generating a continuation token and allows them to choose when to pay the cost of generation. ([#4440](https://github.com/Azure/azure-sdk-for-rust/pull/4440))
- `ContainerClient::query_items()` now takes a `FeedScope` (`FeedScope::partition(...)`, `FeedScope::range(...)`, or `FeedScope::full_container()`) instead of a partition key where `()` represented cross-partition queries. ([#4440](https://github.com/Azure/azure-sdk-for-rust/pull/4440))
- Replaced `CosmosDiagnostics` with `DiagnosticsContext` (a re-export of `azure_data_cosmos_driver::diagnostics::DiagnosticsContext`). All response types now return `Arc<DiagnosticsContext>` from `diagnostics()` (the returned `Arc` derefs transparently to `DiagnosticsContext` for read-only inspection, and can be retained alongside a consumed response body). The previous `activity_id() -> Option<&str>` and `server_duration_ms() -> Option<f64>` accessors on `CosmosDiagnostics` are replaced by `DiagnosticsContext::activity_id() -> &ActivityId` and per-request server timing via `DiagnosticsContext::requests()[i].server_duration_ms()`. ([#4376](https://github.com/Azure/azure-sdk-for-rust/pull/4376))
- Removed `azure_data_cosmos::constants::SubStatusCode` and its `new`/`value`/`from_header_value`/`From`/`Display`/`Debug` API. The SDK no longer maintains a parallel sub-status-code type.
- The `User-Agent` header on every outgoing Cosmos DB request now identifies the wrapping SDK in addition to the driver. The new format is `azsdk-rust-cosmos/<sdk-version> azsdk-rust-cosmos-driver/<driver-version> <os>/<arch> rustc/<ver> [suffix]`, where `<sdk-version>` is this crate's version. This is wired automatically via the new `CosmosDriverRuntimeBuilder::with_wrapping_sdk_identifier` API in the driver, and lets telemetry distinguish callers using `azure_data_cosmos` from callers driving `azure_data_cosmos_driver` directly. No API surface in `azure_data_cosmos` changes. ([#4465](https://github.com/Azure/azure-sdk-for-rust/pull/4465))
- The `azure_data_cosmos::constants` module is no longer public. It only contained internal HTTP-header-name constants used by the SDK's own pipeline plumbing; nothing from it was intended for consumer use. The one previously-exposed public item (`SubStatusCode`) is re-exported from the crate root — see the bullet above. ([#4447](https://github.com/Azure/azure-sdk-for-rust/pull/4447))

### Bugs Fixed

- Fixed `CosmosClientBuilder::with_user_agent_suffix` not propagating the suffix to data-plane requests. The suffix was only applied to the SDK's account-metadata pipeline; requests issued through the driver transport pipeline (the vast majority of operations) had a `User-Agent` header without the configured suffix. The suffix is now forwarded to `CosmosDriverRuntimeBuilder` so it appears on every outgoing request. ([#4368](https://github.com/Azure/azure-sdk-for-rust/pull/4368))

### Other Changes

- Per-partition automatic failover (PPAF) and per-partition circuit breaker (PPCB) are now driven by the `azure_data_cosmos_driver` crate, replacing the SDK's prior implementation. Behavior is unchanged from a configuration standpoint — the existing `AZURE_COSMOS_PER_PARTITION_CIRCUIT_BREAKER_ENABLED` environment variable continues to work — but routing is now per-`(partition_key_range_id, region)` instead of per-region. Driver-level changes are described in [`azure_data_cosmos_driver` 0.3.0](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/cosmos/azure_data_cosmos_driver/CHANGELOG.md). ([#4156](https://github.com/Azure/azure-sdk-for-rust/pull/4156))

## 0.33.0 (2026-04-24)

### Features Added

- Added throughput control API: re-exported `ThroughputControlGroupOptions` and `PriorityLevel` from the driver. Users can register throughput control groups on `CosmosClientBuilder` via `with_throughput_control_group()` to configure priority-based execution and throughput bucket server features. ([#4078](https://github.com/Azure/azure-sdk-for-rust/pull/4078))
- Added `ThroughputPoller` type that implements `IntoFuture` and `Stream` for tracking asynchronous throughput replacement operations.
- Added `FeedRange` type with `ContainerClient::read_feed_ranges()` and `ContainerClient::feed_range_from_partition_key()` - supports hierarchical partition keys (MultiHash) including prefix partition keys that return multiple feed ranges. ([#4149](https://github.com/Azure/azure-sdk-for-rust/pull/4149))
- Added `lsn()` and `item_lsn()` accessors on `ItemResponse<T>` exposing the `lsn` and `x-ms-item-lsn` Cosmos DB response headers. ([#4176](https://github.com/Azure/azure-sdk-for-rust/pull/4176))
- Added `partition_key_range_id` and `internal_partition_id` response headers to the driver bridge, making them accessible on SDK response types. ([#4278](https://github.com/Azure/azure-sdk-for-rust/pull/4278))
- Added `rustls` feature flag (enabled by default) that configures reqwest with rustls as the TLS stack. ([#4252](https://github.com/Azure/azure-sdk-for-rust/pull/4252))
- Added `native_tls` feature flag that configures reqwest with native-tls as the TLS stack. Disable default features and enable `native_tls` to use the platform TLS stack. ([#4252](https://github.com/Azure/azure-sdk-for-rust/pull/4252))
- The `allow_invalid_certificates` feature now works with any TLS backend (`rustls` or `native_tls`). ([#4252](https://github.com/Azure/azure-sdk-for-rust/pull/4252))
- Added `ContainerClient::get_latest_session_token()`. ([#4214](https://github.com/Azure/azure-sdk-for-rust/pull/4214))

### Breaking Changes

- `ContainerClient::create_item()` and `ContainerClient::upsert_item()` now require an `item_id: &str` parameter (same pattern as `replace_item` and `read_item`). The item id is passed to the driver via `ItemReference` so the body never needs to be parsed to extract the document id.
- Renamed `replace_throughput` to `begin_replace_throughput` on `ContainerClient` and `DatabaseClient`. The return type changed from `ResourceResponse<ThroughputProperties>` to `ThroughputPoller`. ([#4096](https://github.com/Azure/azure-sdk-for-rust/pull/4096))
- Removed `CreateDatabaseOptions::with_throughput()`. Database-level shared throughput provisioning is no longer supported through the SDK. Use container-level throughput instead. ([#4147](https://github.com/Azure/azure-sdk-for-rust/pull/4147))

### Other Changes

- Database and container CRUD operations (`create_database`, `read`, `create_container`, `delete`) now route through the Cosmos driver pipeline. Throughput provisioning uses typed request headers via the driver. ([#4147](https://github.com/Azure/azure-sdk-for-rust/pull/4147))
- Query operations (`query_items`, `query_databases`, `query_containers`) now route through the Cosmos driver pipeline, gaining driver-level transport, routing, and retry capabilities. ([#4174](https://github.com/Azure/azure-sdk-for-rust/pull/4174))

## 0.32.0 (2026-04-09)

### Features Added

- Added `CosmosClientBuilder::with_backup_endpoints()` for specifying fallback endpoints when the primary global endpoint is unavailable during initialization. Regional endpoints discovered during bootstrap are automatically used as fallback for subsequent account metadata refreshes. ([#4099](https://github.com/Azure/azure-sdk-for-rust/issues/4099))
- Added `CosmosClientBuilder::with_proxy_allowed(bool)` for explicit opt-in to HTTP proxy usage with documented support limitations. ([#4062](https://github.com/Azure/azure-sdk-for-rust/pull/4062))
- Added `CustomResponseBuilder` and `FaultInjectionRule::hit_count()` APIs for fault injection, enabling ergonomic construction of synthetic HTTP responses and test verification of rule activation counts. ([#3888](https://github.com/Azure/azure-sdk-for-rust/pull/3888))

### Breaking Changes

- HTTP proxies (`HTTPS_PROXY`, `HTTP_PROXY`, `ALL_PROXY` environment variables) are now ignored by default. Use `CosmosClientBuilder::with_proxy_allowed(true)` to opt in. ([#4062](https://github.com/Azure/azure-sdk-for-rust/pull/4062))
- Client methods now return dedicated response types instead of `CosmosResponse<T>`: `ItemResponse<T>` for point operations, `ResourceResponse<T>` for resource management, `BatchResponse` for transactional batch, and `QueryFeedPage<T>` for query pages. `etag()` returns `Option<&Etag>` instead of `Option<&str>`, and `activity_id()` / `server_duration_ms()` are accessed via `response.diagnostics()`. ([#3960](https://github.com/Azure/azure-sdk-for-rust/pull/3960))
- `FeedPage::deconstruct()` has been removed. Use `into_items()`, `continuation()`, `headers()`, and `diagnostics()` instead. ([#3960](https://github.com/Azure/azure-sdk-for-rust/pull/3960))
- Replaced `CosmosClientBuilder::with_application_region()` with a mandatory `RoutingStrategy` parameter on `build()`. Use `RoutingStrategy::ProximityTo(region)` to specify the application region. Also removed `CosmosClientOptions::with_application_region()`. ([#3889](https://github.com/Azure/azure-sdk-for-rust/pull/3889))
- Changed `default_ttl` and `analytical_storage_ttl` fields on `ContainerProperties` from `Option<Duration>` to `TimeToLive`, a new enum with variants `Forever`, `NoDefault`, and `Seconds(u32)`, to correctly handle the `-1` wire value (TTL enabled with no default expiration).
- `DatabaseClient::container_client()` now returns `azure_core::Result<ContainerClient>`, eagerly resolving container metadata (RID, partition key definition) at construction time. ([#4005](https://github.com/Azure/azure-sdk-for-rust/pull/4005))
- `PartitionKeyDefinition` fields (`paths`, `kind`, `version`) are now private; use accessor methods `paths()`, `kind()`, and `version()` instead. `PartitionKeyKind` changed from a string newtype to an enum with variants `Hash`, `MultiHash`, and `Range`. `PartitionKeyVersion` is now an enum (`V1`, `V2`) instead of `Option<i32>`. ([#4005](https://github.com/Azure/azure-sdk-for-rust/pull/4005))
- Replaced `ItemOptions` with `ItemReadOptions` (for `read_item`) and `ItemWriteOptions` (for `create_item`, `replace_item`, `upsert_item`, `delete_item`). `QueryOptions` and `BatchOptions` now also embed `OperationOptions` for general-purpose settings like custom headers, excluded regions, and content response behavior. Replaced per-operation `with_custom_headers` and `with_content_response_on_write_enabled` helpers with `with_operation_options`. Removed `CosmosClientOptions::with_custom_headers()`. ([#4059](https://github.com/Azure/azure-sdk-for-rust/pull/4059))
- Replaced `SessionToken`, `RegionName`, ETag-based conditional fields, content response, and excluded regions types with driver-aligned equivalents: `SessionToken` (now `Cow<'static, str>`), `Region` (use `Region::EAST_US` instead of `regions::EAST_US`), `precondition: Option<Precondition>` (replacing `if_match_etag`/`if_match`/`if_none_match`), `OperationOptions::content_response_on_write: Option<ContentResponseOnWrite>` (replacing `content_response_on_write_enabled: bool`), and `OperationOptions::excluded_regions: Option<ExcludedRegions>`. ([#4059](https://github.com/Azure/azure-sdk-for-rust/pull/4059))

### Bugs Fixed

- Fixes Circuit Breaker Failover Logic for Multi-Master Writes on 403/3. ([#3861](https://github.com/Azure/azure-sdk-for-rust/pull/3861))
- Fixed partition key range fetch using mixed name/RID addressing, which caused 404 errors on certain operations. ([#4047](https://github.com/Azure/azure-sdk-for-rust/pull/4047))

### Other Changes

- `ContainerClient::read_item` now executes through the `azure_data_cosmos_driver` pipeline, gaining driver-level transport, routing, and retry capabilities. ([#4053](https://github.com/Azure/azure-sdk-for-rust/pull/4053))
- `ContainerClient::create_item` now executes through the `azure_data_cosmos_driver` pipeline, gaining driver-level transport, routing, and retry capabilities. ([#4111](https://github.com/Azure/azure-sdk-for-rust/pull/4111))
- Removed internal OpenTelemetry tracing spans pending alignment with [Cosmos DB semantic conventions](https://opentelemetry.io/docs/specs/semconv/registry/attributes/azure/#azure-cosmos-db-attributes). Spans will return in a future release. ([#4104](https://github.com/Azure/azure-sdk-for-rust/pull/4104))
- Added `azure_data_cosmos_driver` as a runtime dependency for internal transport and caching. ([#4005](https://github.com/Azure/azure-sdk-for-rust/pull/4005))

## 0.31.0 (2026-02-25)

### Features Added

- Added `with_excluded_regions` to `ItemOptions` for additional regional routing options. ([#3602](https://github.com/Azure/azure-sdk-for-rust/pull/3602))
- Added `effective_preferred_regions` to the client, ensuring multi-region accounts use all regions for cross-regional availability without supplying regional preferences to their client. ([#3602](https://github.com/Azure/azure-sdk-for-rust/pull/3602))
- Added basic multi-region writes support. ([#3482](https://github.com/Azure/azure-sdk-for-rust/pull/3482) and [#3495](https://github.com/Azure/azure-sdk-for-rust/pull/3495))
- Added new `CosmosResponse` that wraps `azure_core::Response` for all operations except queries. ([#3622](https://github.com/Azure/azure-sdk-for-rust/pull/3622))
- Added transactional batch support for executing multiple operations atomically within the same partition key. ([#3664](https://github.com/Azure/azure-sdk-for-rust/pull/3664))
- Added fault injection support for testing cosmosdb clients in disaster scenarios. Fault injection is behind the feature flag `fault_injection`. ([#3599](https://github.com/Azure/azure-sdk-for-rust/pull/3599))

### Breaking Changes

- Changed our minimum supported Rust version (MSRV) from 1.85 to 1.88.
- Removed `ContainerClient::patch_item`, `PatchDocument`, and `PatchOperation` temporarily to redesign the PATCH API for safe idempotency. Use a Read/Modify/Replace model with ETag-based optimistic concurrency instead.
- Changed return type of query methods from `FeedPager<T>` (an alias for `ItemIterator<FeedPage<T>, String>`) to `FeedItemIterator<T>`, which implements `Stream<Item = Result<T>>` and provides `into_pages()` for page-level access. ([#3515](https://github.com/Azure/azure-sdk-for-rust/pull/3515))
- Introduced `CosmosClientBuilder` for constructing `CosmosClient` instances, replacing constructor-based API. Removed `consistency_level`, `priority`, `throughput_bucket`, `excluded_regions`, `SessionRetryOptions`, triggers, and `IndexingDirective` from options. Simplified `CosmosAccountReference` to take `CosmosAccountEndpoint` directly. Made option struct fields private with getters and `with_*` setters. ([#3744](https://github.com/Azure/azure-sdk-for-rust/pull/3744))
- Removed `with_application_preferred_regions` API. Use `with_application_region` to set the Azure region the app is executing in (or the closest region to the actual location you're running in); the SDK generates preferred regions by geographic proximity. ([#3796](https://github.com/Azure/azure-sdk-for-rust/pull/3796))
- Made `CosmosClientBuilder::build()` and `DatabaseClient::container_client()` async to prepare for future cache population (account, collection, partition key range caches).
- Support for `wasm32-unknown-unknown` has been removed ([#3377](https://github.com/Azure/azure-sdk-for-rust/issues/3377))

### Other Changes

- Added default HTTP client timeouts and added retries for connection errors. ([#3752](https://github.com/Azure/azure-sdk-for-rust/pull/3752))
- Retry policies now retry reads on all non-whitelisted status codes and retry service unavailable errors across all applicable endpoints. ([#3728](https://github.com/Azure/azure-sdk-for-rust/pull/3728))

## 0.30.0 (2026-01-21)

### Features Added

- Added GlobalEndpointManager, LocationCache to support Cross Regional Retry.
- Added `continuation_token` to `PagerOptions` for methods that return a `Pager`.
- Added `throughput_bucket`, `priority`, and `custom_headers` to different request options. ([#3482](https://github.com/Azure/azure-sdk-for-rust/pull/3482))
- Added several new options to `QueryOptions`. ([#3482](https://github.com/Azure/azure-sdk-for-rust/pull/3482))

### Breaking Changes

- Removed `Pager::with_continuation_token()` for methods that return a `Pager`.

### Other Changes

- Added `ALLOWED_COSMOS_HEADERS` for use in default logging policy. ([#3554](https://github.com/Azure/azure-sdk-for-rust/pull/3554))

## 0.29.0 (2025-11-10)

### Features Added

- Added Regions to pass preferred regions through Cosmos Client Options. ([#3274](https://github.com/Azure/azure-sdk-for-rust/pull/3274))
- Adjusted the query engine abstraction to support future enhancements and optimizations. ([#3166](https://github.com/Azure/azure-sdk-for-rust/pull/3166))

### Breaking Changes

- Renamed `Response<T, F>::into_body(self) -> Result<Response<T>>` to `into_model(self) -> Result<Response<T>>`. `into_body(self)` now returns a `ResponseBody`.

## 0.28.0 (2025-10-07)

### Features Added

- Added `Query::with_text()` and `Query::append_text()` methods to modify query text after creation ([#3044](https://github.com/Azure/azure-sdk-for-rust/pull/3044))
- Added `PatchDocument::with_condition()` methods to allow setting a condition on a patch operation ([#2969](https://github.com/Azure/azure-sdk-for-rust/pull/2969))

### Breaking Changes

- Client methods that return a `Response<T>>` asynchronously buffer the entire model within the internal pipeline, so `into_body()` and other methods on the response are no longer async.

## 0.27.0 (2025-09-17)

### Other Changes

- Updated Core SDK dependencies

## 0.26.0 (2025-08-06)

### Other Changes

- Updated Core SDK dependencies

## 0.25.0 (2025-08-05)

### Features Added

- Added `if_match_etag` to `ItemOptions` ([#2705](https://github.com/Azure/azure-sdk-for-rust/pull/2705))
- Added several more options to `ItemOptions`: `pre_triggers`, `post_triggers`, `session_token`, `consistency_level`, and `indexing_directive` ([#2744](https://github.com/Azure/azure-sdk-for-rust/pull/2744))

### Breaking Changes

- Minimum supported Rust version (MSRV) is now 1.85.

## 0.24.0 (2025-06-10)

### Features Added

- Added a function `CosmosClient::with_connection_string` to enable `CosmosClient` creation via connection string. ([#2641](https://github.com/Azure/azure-sdk-for-rust/pull/2641))
- Added support for executing limited cross-partition queries through the Gateway. See <https://learn.microsoft.com/rest/api/cosmos-db/querying-cosmosdb-resources-using-the-rest-api#queries-that-cannot-be-served-by-gateway> for more details on these limitations. ([#2577](https://github.com/Azure/azure-sdk-for-rust/pull/2577))
- Added a preview feature (behind `preview_query_engine` feature flag) to allow the Rust SDK to integrate with an external query engine for performing cross-partition queries. ([#2577](https://github.com/Azure/azure-sdk-for-rust/pull/2577))

### Breaking Changes

- `FeedPager<T>` now asynchronously iterates items of type `T` instead of pages containing items of type `T`. Call `FeedPager::into_pages()` to get a `PageIterator` to asynchronously iterate over all pages. ([#2665](https://github.com/Azure/azure-sdk-for-rust/pull/2665))

## 0.23.0 (2025-05-06)

### Features Added

- Decoupled query responses from HTTP to allow for handling non-HTTP transports for queries. ([#2393](https://github.com/Azure/azure-sdk-for-rust/pull/2393))

### Breaking Changes

- Query APIs (`CosmosClient::query_databases`, `DatabaseClient::query_containers`, `ContainerClient::query_items`) now return a `FeedPager` instead of an `azure_core::Pager`. The `FeedPager` type provides an abstraction over the transport layer, allowing for more flexibility when queries are executed over non-HTTP transports or are decoupled from specific HTTP responses (such as in cross-partition queries). ([#2393](https://github.com/Azure/azure-sdk-for-rust/pull/2393))

## 0.22.1 (2025-03-05)

### Bugs Fixed

- Fixed a publishing issue that caused the `key_auth` feature to be omitted. ([#2241](https://github.com/Azure/azure-sdk-for-rust/issues/2241))

## 0.22.0 (2025-02-25)

### Features Added

- Initial supported release.

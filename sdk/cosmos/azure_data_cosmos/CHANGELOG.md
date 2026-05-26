# Release History

## 0.34.0 (Unreleased)

### Features Added

- Added `new()` constructors and `with_x` consuming setters to multi-required-field model types so callers can build them declaratively without struct-literal syntax (which is now blocked by `#[non_exhaustive]`): `VectorEmbedding::new(path, data_type, dimensions, distance_function)` + `with_path` / `with_data_type` / `with_dimensions` / `with_distance_function`; `ConflictResolutionPolicy::new(mode)` + `with_resolution_path` / `with_resolution_procedure`; `SpatialIndex::new(path)` + `with_type` (singular pusher onto `types`); `CompositeIndexProperty::new(path, order)` + `with_path` / `with_order`; `VectorIndex::new(path, index_type)` + `with_path` / `with_index_type`. These types do **not** implement `Default` — their constructors require values that have no meaningful default.
- Derived `Default` on `VectorEmbeddingPolicy`, `UniqueKeyPolicy`, `UniqueKey`, `PropertyPath`, and `CompositeIndex`, and added singular `with_x` pushers / setters: `VectorEmbeddingPolicy::with_embedding`, `UniqueKeyPolicy::with_unique_key`, `UniqueKey::with_path`, `PropertyPath::with_path`, and `CompositeIndex::with_property`. This matches the existing `IndexingPolicy::with_included_path` style and lets callers build these policies declaratively without constructing intermediate `Vec`s.
- Added `QueryFeedPage::as_feed_page()` returning `&FeedPage<T>`, so a query page can be passed to APIs that accept the more general `FeedPage` type. Query-specific metadata (index/query metrics) remains accessible on the `QueryFeedPage` itself.
- Added `QueryOptions::with_populate_index_metrics(bool)`, `with_populate_query_metrics(bool)`, and `with_max_item_count(MaxItemCountHint)` setters. These replace the previous pattern of passing raw `x-ms-cosmos-populateindexmetrics`, `x-ms-documentdb-populatequerymetrics`, and `x-ms-max-item-count` values through `OperationOptions::with_custom_headers` for query execution. `max_item_count` takes the new `MaxItemCountHint` enum with `ServerDecides` and `Limit(NonZeroU32)` variants, so callers don't have to traffic in the `-1` wire sentinel directly. ([#4401](https://github.com/Azure/azure-sdk-for-rust/pull/4401))
- Added `ContainerClient::patch_item()` for applying JSON-Patch-style mutations to a single item. Supports `add`/`set`/`replace`/`remove`/`increment`/`move` ops via the new `PatchDocument`/`PatchOperation`/`CosmosNumber` types (re-exported at the crate root). Added `PatchItemOptions` for per-request configuration (`max_attempts`, `session_token`, etc.). `PatchItemOptions` intentionally does not expose a `Precondition` or SQL filter predicate — the driver-side PATCH handler owns the internal `If-Match` end-to-end, and predicate evaluation is out of scope for this preview. The method's rustdoc documents the non-idempotent-under-transport-failure caveat. ([#4386](https://github.com/Azure/azure-sdk-for-rust/pull/4386))
- Support for simple cross-partition queries with `SELECT` projections and `WHERE` filters. Cross-partition queries are now done through fan-out in the client, and provide a client-generated continuation token that can be used to resume the query. See `ContainerClient::query_items()` and `FeedScope` for details. ([#4440](https://github.com/Azure/azure-sdk-for-rust/pull/4440))
- Added `QueryOptions::continuation_token` and `QueryOptions::with_continuation_token(...)` for resuming queries from a continuation token. ([#4440](https://github.com/Azure/azure-sdk-for-rust/pull/4440))

### Breaking Changes

- Marked the following public model enums and response wrappers as `#[non_exhaustive]` to allow future variants/fields to be added without further breaking changes: `VectorDataType`, `VectorDistanceFunction`, `ConflictResolutionMode`, `IndexingMode`, `SpatialType`, `CompositeIndexOrder`, `VectorIndexType`, `BatchResponse`, `ItemResponse`, `ResourceResponse<T>`, `ResponseBody`, `ResponseHeaders`, and the re-exported driver types `PartitionKeyVersion` and `CosmosStatus`. Callers must use `..` wildcard arms in `match`es over these enums and cannot construct these structs via struct-literal syntax (the SDK already provides constructors / setters for the constructable types).
- `CosmosClientBuilder::build` now takes `AccountReference` directly instead of `impl Into<AccountReference>`. With the tuple `From` impls removed in the previous change, the generic bound no longer added flexibility — callers should construct an `AccountReference` explicitly via `AccountReference::with_credential` or `AccountReference::with_authentication_key` and pass it in.
- Renamed `CosmosAccountEndpoint` → `AccountEndpoint` and `CosmosAccountReference` → `AccountReference`. The `Cosmos` prefix is implied by the containing `azure_data_cosmos` crate. Note that the driver crate has its own `AccountReference` type with a different shape; SDK source files that need both refer to the driver type via its fully-qualified path (`azure_data_cosmos_driver::models::AccountReference`).
- Renamed `FeedPageIterator` → `QueryPageIterator` and `FeedItemIterator` → `QueryItemIterator`. These iterators are only produced by query APIs today; the `Feed*` names are reserved for future non-query feed APIs.
- Removed the `request_charge()` and `session_token()` convenience accessors from `FeedPage` and `QueryFeedPage`. Use `page.headers().request_charge()` and `page.headers().session_token()` instead — the parsed `ResponseHeaders` already exposes these values and provides full typed access to every other response header.
- Tightened `AccountReference` constructors. `with_credential` now accepts `impl Into<AccountEndpoint>` instead of a concrete `AccountEndpoint`. The former `with_master_key` is renamed to `with_authentication_key` and now takes `impl Into<AccountEndpoint>` and `impl Into<Secret>` for the key. The two `From<(AccountEndpoint, _)>` / `From<(Url, _)>` tuple conversions are removed; construct a `AccountReference` via the named constructors instead.
- Removed `azure_data_cosmos::ConnectionString` from the public API. The type was a parsing helper not consumed by any public SDK API (the SDK takes a `AccountEndpoint` and a credential, not a connection string). Callers that need to parse a Cosmos DB connection string can use `azure_data_cosmos_driver::models::ConnectionString` directly.
- Refactored the response surface to be SDK-owned. `ItemResponse` drops its type parameter (use `response.into_model::<MyItem>()` or `response.into_body().into_single::<MyItem>()`); `ResourceResponse<T>` keeps its parameter so `.into_model()?` still works without a turbofish. `status()` now returns `CosmosStatus`, `headers()` returns `&ResponseHeaders` (typed accessors only — `etag()`, `request_charge()`, `session_token()`, `continuation()`, `activity_id()`, `substatus()`, `index_metrics()`, `query_metrics()`, `offer_replace_pending()`, `server_duration_ms()`, `lsn()`, `item_lsn()`, `item_count()`, …), and `into_body()` returns the SDK-owned `ResponseBody` enum (`NoPayload` / `Bytes` / `Items`) with `single()`, `items()`, `into_single::<T>()`, `into_items::<T>()`, and `is_empty()` helpers. `FeedPage::headers()` / `QueryFeedPage::headers()` now return `&ResponseHeaders` instead of `&azure_core::http::headers::Headers`. The `ItemResponse::etag()` convenience accessor is removed (use `response.headers().etag()`). `CosmosStatus` is re-exported from the driver and implements `PartialEq<StatusCode>` and `From<CosmosStatus> for StatusCode/u16`, so existing comparisons keep working. ([#4401](https://github.com/Azure/azure-sdk-for-rust/pull/4401))

### Other Changes

- Removed the SDK-side `FaultInjectionClientBuilder`, parallel duplicate types (`FaultInjectionRule`, `FaultInjectionCondition`, `FaultInjectionResult`, `CustomResponse`, the matching builders, `FaultInjectionErrorType`, `FaultOperationType`), and the SDK-side `FaultClient` HTTP wrapper from `azure_data_cosmos::fault_injection`. The module is now a pure re-export of the driver's fault-injection types — fault-injection rules flow directly to the driver runtime and are evaluated by the driver's transport-layer fault-injection client. `CosmosClientBuilder::with_fault_injection` now accepts the driver's `Vec<Arc<FaultInjectionRule>>` directly instead of `FaultInjectionClientBuilder`. Callers should construct rules via the re-exported `FaultInjectionRuleBuilder` and pass the vector.([#4426](https://github.com/Azure/azure-sdk-for-rust/pull/4426))
- Removed the `request_url()` accessor (gated on the `fault_injection` feature) from `ItemResponse`/`ResourceResponse`/`BatchResponse`. Driver-routed operations never populated it, so it always returned `None` in current usage.
- `CosmosClientBuilder::with_user_agent_suffix` (and `CosmosClientOptions::with_user_agent_suffix`) now take `UserAgentSuffix` instead of `impl Into<String>`. Callers passing a `&str` or `String` must construct the value explicitly via `UserAgentSuffix::new` (panics on invalid input) or `UserAgentSuffix::try_new` (returns `Option`). Validation rules (max 25 characters, HTTP-header-safe) are now enforced at the construction site instead of being applied silently inside the builder. ([#4368](https://github.com/Azure/azure-sdk-for-rust/pull/4368))
- Changed how continuation tokens are returned. Instead of a `continuation()` accessor on `QueryFeedPage` and `FeedPage`, continuation tokens are now returned as a `Option<ContinuationToken>` from the `FeedPageIterator::to_continuation_token(&self)` method. Generating a continuation token for a cross-partition query requires computation, so this change makes it explicit that callers must opt in to generating a continuation token and allows them to choose when to pay the cost of generation. ([#4440](https://github.com/Azure/azure-sdk-for-rust/pull/4440))
- `ContainerClient::query_items()` now takes a `FeedScope` (`FeedScope::partition(...)`, `FeedScope::range(...)`, or `FeedScope::full_container()`) instead of a partition key where `()` represented cross-partition queries. ([#4440](https://github.com/Azure/azure-sdk-for-rust/pull/4440))
- Replaced `CosmosDiagnostics` with `DiagnosticsContext` (a re-export of `azure_data_cosmos_driver::diagnostics::DiagnosticsContext`). All response types now return `Arc<DiagnosticsContext>` from `diagnostics()` (the returned `Arc` derefs transparently to `DiagnosticsContext` for read-only inspection, and can be retained alongside a consumed response body). The previous `activity_id() -> Option<&str>` and `server_duration_ms() -> Option<f64>` accessors on `CosmosDiagnostics` are replaced by `DiagnosticsContext::activity_id() -> &ActivityId` and per-request server timing via `DiagnosticsContext::requests()[i].server_duration_ms()`.
- Removed `azure_data_cosmos::constants::SubStatusCode` and its `new`/`value`/`from_header_value`/`From`/`Display`/`Debug` API. The SDK no longer maintains a parallel sub-status-code type — fault-injection (the only remaining consumer) now uses `azure_data_cosmos_driver::models::SubStatusCode` directly. Callers that referenced the SDK type should switch to the driver re-export.
- The `User-Agent` header on every outgoing Cosmos DB request now identifies the wrapping SDK in addition to the driver. The new format is `azsdk-rust-cosmos/<sdk-version> azsdk-rust-cosmos-driver/<driver-version> <os>/<arch> rustc/<ver> [suffix]`, where `<sdk-version>` is this crate's version. This is wired automatically via the new `CosmosDriverRuntimeBuilder::with_wrapping_sdk_identifier` API in the driver, and lets telemetry distinguish callers using `azure_data_cosmos` from callers driving `azure_data_cosmos_driver` directly. No API surface in `azure_data_cosmos` changes. ([#4465](https://github.com/Azure/azure-sdk-for-rust/pull/4465))

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
- Introduced `CosmosClientBuilder` for constructing `CosmosClient` instances, replacing constructor-based API. Removed `consistency_level`, `priority`, `throughput_bucket`, `excluded_regions`, `SessionRetryOptions`, triggers, and `IndexingDirective` from options. Simplified `AccountReference` to take `AccountEndpoint` directly. Made option struct fields private with getters and `with_*` setters. ([#3744](https://github.com/Azure/azure-sdk-for-rust/pull/3744))
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

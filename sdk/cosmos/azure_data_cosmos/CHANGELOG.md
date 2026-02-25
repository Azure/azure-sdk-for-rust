# Release History

## 0.31.0 (Unreleased)

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

### Bugs Fixed

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

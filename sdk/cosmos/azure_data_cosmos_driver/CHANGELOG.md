# Release History

## 0.3.0 (Unreleased)

### Features Added

### Breaking Changes

### Bugs Fixed

### Other Changes

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


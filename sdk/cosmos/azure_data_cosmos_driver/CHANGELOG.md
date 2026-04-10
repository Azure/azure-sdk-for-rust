# Release History

## 0.1.0 (2026-04-09)

### Features Added

- Initial release of `azure_data_cosmos_driver` (core Cosmos DB protocol implementation for cross-language SDK reuse). ([#3772](https://github.com/Azure/azure-sdk-for-rust/pull/3772) and [#3592](https://github.com/Azure/azure-sdk-for-rust/pull/3592))
- Added cache priming via `CosmosDriver::initialize()` and `CosmosDriver::prime_container()` to avoid cold-start latency on the first data-plane operation. ([#3864](https://github.com/Azure/azure-sdk-for-rust/pull/3864))
- Added response metadata fields (`index_metrics`, `query_metrics`, `server_duration_ms`, `lsn`) to `CosmosResponseHeaders` and `RequestDiagnostics`, with base64 decoding for `index_metrics`. ([#3960](https://github.com/Azure/azure-sdk-for-rust/pull/3960))
- Added hierarchical partition key (MultiHash) support. ([#4087](https://github.com/Azure/azure-sdk-for-rust/pull/4087))


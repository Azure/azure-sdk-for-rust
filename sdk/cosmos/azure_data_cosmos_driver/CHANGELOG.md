# Release History

## 0.1.0 (Unreleased)

### Features Added

- Initial release of `azure_data_cosmos_driver` (core Cosmos DB protocol implementation for cross-language SDK reuse). ([#3772](https://github.com/Azure/azure-sdk-for-rust/pull/3772) and [#3592](https://github.com/Azure/azure-sdk-for-rust/pull/3592))
- Added cache priming via `CosmosDriver::initialize()` and `CosmosDriver::prime_container()` to avoid cold-start latency on the first data-plane operation. ([#3864](https://github.com/Azure/azure-sdk-for-rust/pull/3864))
- Added `index_metrics`, `query_metrics`, `server_duration_ms`, and `lsn` fields to `CosmosResponseHeaders` for access to additional response metadata. The `index_metrics` field is base64-decoded from the raw header value.
- Added `server_duration_ms` to `RequestDiagnostics`, populated from the `x-ms-request-duration-ms` response header.
- Added fault injection support behind `fault_injection` feature flag for transport-level fault injection per Transport Pipeline Spec §7.

### Breaking Changes

### Bugs Fixed

### Other Changes

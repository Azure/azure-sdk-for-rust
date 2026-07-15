# Release History

## 0.1.0 (Unreleased)

### Features Added

- Initial release of `azure_data_cosmos_driver` (core Cosmos DB protocol implementation for cross-language SDK reuse).
  - ([#3772](https://github.com/Azure/azure-sdk-for-rust/pull/3772))
- Added the `DiagnosticsContext` foundation: `CosmosResponse::diagnostics()` returns the always-collected operation diagnostics (`is_completed`/`is_failure`/`is_threshold_violated`, `duration`, `status`, `total_request_charge`, `contacted_regions`, `operation_name`, `requests`, `to_json_string`), backed by `RequestDiagnostics`, `ExecutionContext`, and `DiagnosticsOptions`/`DiagnosticsVerbosity`.

### Breaking Changes

### Bugs Fixed

### Other Changes

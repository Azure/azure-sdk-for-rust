# Release History

## 0.23.0 (Unreleased)

### Features Added

- Added `azure_core::process::Executor` to run commands asynchronously.
  The `tokio` feature is disabled by default so `std::process::Command` is used; otherwise, if enabled, `tokio::process::Command` is used.
- Moved `Pager` from `typespec_client_core::http` to `azure_core::http` module since it is Azure-specific.

### Breaking Changes

- Consolidated all the `tokio` features into a single feature named `tokio`. Traits remain separate but `tokio` support is enabled with a single feature.
- Removed `Header` re-export from `http` module. It is still defined in the `http::headers` module.

### Bugs Fixed

### Other Changes

## 0.22.0 (2025-02-18)

### Features Added

- Initial supported release.

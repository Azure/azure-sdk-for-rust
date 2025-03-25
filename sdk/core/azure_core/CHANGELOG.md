# Release History

## 0.23.0 (Unreleased)

### Features Added

- Added `azure_core::process::Executor` to run commands asynchronously.
  The `tokio_process` feature is disabled by default so `std::process::Command` is used; otherwise, if enabled, `tokio::process::Command` is used.

### Breaking Changes

- Removed `azure_core::credentials::TokenCredential::clear_cache()`

### Bugs Fixed

### Other Changes

## 0.22.0 (2025-02-18)

### Features Added

- Initial supported release.

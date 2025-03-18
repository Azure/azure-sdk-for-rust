# Release History

## 0.23.0 (Unreleased)

### Features Added

- Added `azure_core::process::Executor` to run commands asynchronously.
  The `tokio` feature is disabled by default so `std::process::Command` is used; otherwise, if enabled, `tokio::process::Command` is used.

### Breaking Changes

### Bugs Fixed

### Other Changes

## 0.22.0 (2025-02-18)

### Features Added

- Initial supported release.

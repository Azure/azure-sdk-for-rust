# Release History

## Unreleased

### Features Added

- Added `cosmos_read_many_request_init` and `cosmos_read_many_open_submit` with versioned request records for item or complete-partition selections and optional parameterized filters, returning paged results through the existing native cursor APIs without durable checkpoint/resume support. ([#5385](https://github.com/Azure/azure-sdk-for-rust/pull/5385))

### Breaking Changes

### Bugs Fixed

### Other Changes

## 0.2.0 (2026-10-01)

### Features Added

- Added runtime, client, and request operation options to the native C FFI, including immutable admission snapshots for operation submission. ([#5366](https://github.com/Azure/azure-sdk-for-rust/pull/5366))

### Breaking Changes

- Changed the pre-1.0 operation-options and request ABI layouts, including 64-bit retry-count fields; hosts using the 0.1.0 bootstrap ABI must rebuild for 0.2.0. ([#5366](https://github.com/Azure/azure-sdk-for-rust/pull/5366))

### Bugs Fixed

### Other Changes

- Began maintained native release history at 0.2.0; earlier 0.1.0 development is treated as the bootstrap baseline.

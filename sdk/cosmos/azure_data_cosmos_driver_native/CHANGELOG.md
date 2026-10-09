# Release History

## 0.3.0 (Unreleased)

### Features Added

- Added `cosmos_completion_item_count` and `cosmos_completion_item_page` to inspect original binary pages and item ranges owned by queue and cursor completions; cursor item buffers remain standalone. ([#5409](https://github.com/Azure/azure-sdk-for-rust/pull/5409))

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

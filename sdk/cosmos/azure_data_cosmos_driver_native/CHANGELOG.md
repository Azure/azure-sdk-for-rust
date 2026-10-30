# Release History

## 0.3.0 (Unreleased)

### Features Added

- Added cross-partition aggregate, `GROUP BY`, and distinct-count queries to live query cursors; aggregate and grouped cursors reject external checkpoints until drained.
- Added a driver option for disabling default-on resource-name validation, including item names supplied to Create and Upsert, when compatibility requires it. ([#5482](https://github.com/Azure/azure-sdk-for-rust/pull/5482))

### Breaking Changes

- Expanded `cosmos_driver_options_config_v2_t`; native hosts must rebuild against the 0.3.0 header and library. ([#5482](https://github.com/Azure/azure-sdk-for-rust/pull/5482))

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

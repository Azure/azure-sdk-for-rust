# Release History

## 0.3.0 (Unreleased)

### Features Added

- Added `bootstrap_timeout_ms` in the processor request's first reserved slot; zero keeps the 120-second default, and nonzero values require a supporting native library. Request size and ABI version are unchanged. ([#5589](https://github.com/Azure/azure-sdk-for-rust/pull/5589))
- Added `cosmos_processor_start_with_policy_submit` and `cosmos_processor_acknowledge_submit` for policy-controlled checkpointing, with distinct acknowledged and durably saved completion results.
- Added `cosmos_processor_*` completion-queue APIs for driver-owned change feed processing and explicit post-callback checkpoint acknowledgments.
- Added a driver option for disabling default-on resource-name validation, including item names supplied to Create and Upsert, when compatibility requires it. ([#5482](https://github.com/Azure/azure-sdk-for-rust/pull/5482))

### Breaking Changes

- Renamed processor request fields to `workload_name` and `worker_name`; native consumers must update source field accesses. Field order, layout, and ABI version are unchanged. ([#5589](https://github.com/Azure/azure-sdk-for-rust/pull/5589))
- `cosmos_processor_start_submit` now defaults to deferred checkpointing with a five-minute interval; use policy `0` with `cosmos_processor_start_with_policy_submit` for eager persistence. Explicit checkpoint submission remains a durable barrier; ABI signatures and policy values are unchanged. ([#5589](https://github.com/Azure/azure-sdk-for-rust/pull/5589))
- Processor leases now permit takeover after processing inactivity, and idle reads no longer flush deferred checkpoints; the processor ABI is unchanged.
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

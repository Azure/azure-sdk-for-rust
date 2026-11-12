# Release History

## 0.4.0 (Unreleased)

### Features Added

- Added cross-partition aggregate, `GROUP BY`, and distinct-count queries to live query cursors; aggregate and grouped cursors reject external checkpoints until drained. ([#5541](https://github.com/Azure/azure-sdk-for-rust/pull/5541))

### Breaking Changes

### Bugs Fixed

### Other Changes

## 0.3.0 (2026-11-11)

### Features Added

- Added `cosmos_read_many_request_init` and `cosmos_read_many_open_submit` with versioned request records for key/id or partition selections, including hierarchical prefixes and optional parameterized filters, returning paged results through the existing native cursor APIs without durable checkpoint/resume support. ([#5385](https://github.com/Azure/azure-sdk-for-rust/pull/5385))
- Added a driver option for disabling default-on resource-name validation, including item names supplied to Create and Upsert, when compatibility requires it. ([#5482](https://github.com/Azure/azure-sdk-for-rust/pull/5482))

### Breaking Changes

- Expanded `cosmos_driver_options_config_v2_t`; native hosts must rebuild against the 0.3.0 header and library. ([#5482](https://github.com/Azure/azure-sdk-for-rust/pull/5482))
- Terminal 404/1002, 403/3, and 403/1008 service failures now complete as synthetic 503 errors with the new sub-statuses `COSMOS_SUB_STATUS_CLIENT_READ_SESSION_NOT_AVAILABLE` (20310), `COSMOS_SUB_STATUS_CLIENT_WRITE_FORBIDDEN` (20311), and `COSMOS_SUB_STATUS_CLIENT_DATABASE_ACCOUNT_NOT_FOUND` (20312). `http_status_code` now reports the effective status, and these wrappers report `is_from_wire == 0` while retaining the original response metadata. ([#5392](https://github.com/Azure/azure-sdk-for-rust/pull/5392))
- New query and change-feed checkpoints use the `c2.` format, which 0.2.0 rejects; keep every reader on 0.3.0 or later once `c2.` checkpoints are written. Legacy `c1.` checkpoints remain readable, except that older unpolled single-request change-feed checkpoints require the original start and older prefix checkpoints with a single saved ETag are rejected. ([#5448](https://github.com/Azure/azure-sdk-for-rust/pull/5448))

### Bugs Fixed

- Fixed change-feed checkpoint resume so checkpoints captured before the first Next keep their original start, saved ETags, and next range, and fixed hierarchical-prefix feed ranges to fan out across every overlapping physical range. ([#5448](https://github.com/Azure/azure-sdk-for-rust/pull/5448))

### Other Changes

- Rust allocations in the native library now use mimalloc by default, which improved point-read throughput and CPU per operation in benchmarks. Build with `--no-default-features` to keep the system allocator. The host process's `malloc`/`free` are not overridden. ([#5579](https://github.com/Azure/azure-sdk-for-rust/pull/5579))
- Updated the bundled `azure_data_cosmos_driver` from 1.0.0-beta.2 to 1.1.0-beta.3, which includes its routing, hedging, diagnostics, and search fixes and features.

## 0.2.0 (2026-10-01)

### Features Added

- Added runtime, client, and request operation options to the native C FFI, including immutable admission snapshots for operation submission. ([#5366](https://github.com/Azure/azure-sdk-for-rust/pull/5366))

### Breaking Changes

- Changed the pre-1.0 operation-options and request ABI layouts, including 64-bit retry-count fields; hosts using the 0.1.0 bootstrap ABI must rebuild for 0.2.0. ([#5366](https://github.com/Azure/azure-sdk-for-rust/pull/5366))

### Bugs Fixed

### Other Changes

- Began maintained native release history at 0.2.0; earlier 0.1.0 development is treated as the bootstrap baseline.

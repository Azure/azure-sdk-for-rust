# Release History

## 0.9.0 (Unreleased)

### Features Added

### Breaking Changes

- Event Hubs treats the consumer group as case insensitive, so one deployment that spelled the group `$Default` on one run and `$default` on the next built two disjoint key sets and reprocessed events. ([#5099](https://github.com/Azure/azure-sdk-for-rust/issues/5099))
- The checkpoint and ownership blob key that `BlobCheckpointStore` reads and writes now folds the fully qualified namespace, the event hub name, and the consumer group to lowercase ASCII. The partition id keeps its case. The .NET, JavaScript, and Python clients fold these three names; the Go and Java clients do not, so this change moves the Rust crate from the second group into the first.
- The old key `NS.ServiceBus.Windows.Net/My-Hub/$Default/checkpoint/0` becomes `ns.servicebus.windows.net/my-hub/$default/checkpoint/0`. The ownership key changes in the same way.
- Migration: records that an older Rust client wrote stay at the old key and become unreachable. The change adds no dual read and no fallback lookup. A processor that starts against an existing container resumes from its configured start position.

### Bugs Fixed

### Other Changes

## 0.8.0 (2026-04-22)

### Other Changes

- Updated dependencies.

## 0.7.0 (2026-04-14)

### Other Changes

- Updated dependencies.

## 0.6.0 (2026-03-10)

### Breaking Changes

- Support for `wasm32-unknown-unknown` has been removed ([#3377](https://github.com/Azure/azure-sdk-for-rust/issues/3377))

### Other Changes

- Updated dependencies.

## 0.5.0 (2026-02-11)

### Breaking Changes

- Changed our minimum supported Rust version (MSRV) from 1.85 to 1.88.

### Other Changes

- Updated dependencies.

## 0.4.0 (2026-02-10)

### Other Changes

- Updated dependencies.

## 0.3.0 (2025-11-11)

### Other Changes

- Updated dependencies.

## 0.2.0 (2025-10-08)

### Other Changes

- Updated dependencies.

## 0.1.0 (2025-09-16)

### Features Added

- Initial Release

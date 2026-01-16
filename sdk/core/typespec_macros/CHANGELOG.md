# Release History

## 0.10.0 (2026-01-16)

### Other Changes

- Updated dependencies.

## 0.9.1 (2025-11-09)

### Other Changes

- Increment version for re-release following a fix to publishing.

## 0.9.0 (2025-11-07)

### Other Changes

- Updated dependencies.

## 0.8.1 (2025-10-06)

### Bugs Fixed

- Fix feature documentation ([#3118](https://github.com/Azure/azure-sdk-for-rust/issues/3118))

## 0.8.0 (2025-10-03)

### Features Added

- Added `Error::with_error_fn()`.

### Breaking Changes

- Renamed a number of construction functions for `Error` to align with [guidelines](https://azure.github.io/azure-sdk/rust_introduction.html)
  - Renamed `Error::full()` to `Error::with_error()`.
  - Renamed `Error::with_message()` to `Error::with_message_fn()`.
  - Renamed `Error::message()` to `Error::with_message()`.
  - Renamed `Error::with_context()` to `Error::with_context_fn()`.
  - Renamed `Error::context()` to `Error::with_context()`.
  - Renamed `ResultExt::map_kind()` to `ResultExt::with_kind()`.
  - Renamed `ResultExt::with_context()` to `ResultExt::with_context_fn()`.
  - Renamed `ResultExt::context()` to `ResultExt::with_context()`.

## 0.7.0 (2025-09-11)

### Other Changes

- Updated dependencies.

## 0.6.0 (2025-08-01)

## 0.5.0 (2025-07-10)

### Breaking Changes

- Minimum supported Rust version (MSRV) is now 1.85.

## 0.4.0 (2025-06-06)

### Features Added

- Added `#[safe]` attribute helper for `SafeDebug` derive macro to show or hide types and members as appropriate.

### Breaking Changes

- Removed `Model` derive macro.

## 0.3.0 (2025-05-02)

### Other Changes

- Deriving `SafeDebug` formats non-exhaustive types by default. Enable `debug` feature to format normal `Debug` output.
- Updated dependencies.

## 0.2.0 (2025-04-08)

### Other Changes

- Updated dependencies.

## 0.1.0 (2025-02-18)

### Features Added

- Initial supported release.

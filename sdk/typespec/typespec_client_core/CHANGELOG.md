# Release History

## 0.3.0 (Unreleased)

### Features Added

### Breaking Changes

### Bugs Fixed

### Other Changes

- Deriving `SafeDebug` formats non-exhaustive types by default. Enable `debug` feature to format normal `Debug` output.

## 0.2.0 (2025-04-08)

### Breaking Changes

- Consolidated all the `tokio` features into a single feature named `tokio`. Traits remain separate but `tokio` support is enabled with a single feature.
- Removed `Header` re-export from `http` module. It is still defined in the `http::headers` module.
- Removed `http-types` dependency and implemented `Method` instead.
- Removed `Pager`.
- Removed `parsing` module.

### Other Changes

- Use `std::sync::LazyLock` added in rustc 1.80 instead of `once_cell::sync::Lazy`.

## 0.1.0 (2025-02-18)

### Features Added

- Initial supported release.

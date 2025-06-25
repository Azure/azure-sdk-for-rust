# Release History

## 0.5.0 (Unreleased)

### Features Added

- Added `get_async_runtime()` and `set_async_runtime()` to allow customers to replace
the default asynchronous runtime with another.

### Breaking Changes

- Removed `Pipeline::replace_policy`.
- Renamed `typespec_client_core::date` to `typespec_client_core::time` and added `typespec_client_core::time::Duration`

### Bugs Fixed

### Other Changes

- The `CustomHeadersPolicy` is executed after the retry policy in the `Pipeline`.

## 0.4.0 (2025-06-06)

### Features Added

- Added `#[safe]` attribute helper for `SafeDebug` derive macro to show or hide types and members as appropriate.
- Added module `fmt::as_string` which is used to (de)serialize types in string format.
- Added `Response<T, F>` type to represent a response with a specific format and model type.
- Added `RawResponse` type to represent a raw response without a specific model.

### Breaking Changes

- Removed the `Model` trait and replaced it with `Response<T, F>`, which moves the "format" information (JSON/XML/etc.) from the model to the service client method's return type. This allows for more flexibility in handling different response formats and user data types in Responses.
- Split `Response` into `Response<T, F>` and `RawResponse` (which carries the raw response data without a specific format).

## 0.3.0 (2025-05-02)

### Breaking Changes

- The `reqwest_rustls` feature enables `rustls-tls-native-roots-no-provider` instead of `rustls-tls-native-roots` to remove the dependency on the `ring` crate.

### Other Changes

- Deriving `SafeDebug` formats non-exhaustive types by default. Enable `debug` feature to format normal `Debug` output.
- Updated dependencies.

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

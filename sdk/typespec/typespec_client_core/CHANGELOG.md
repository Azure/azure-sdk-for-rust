# Release History

## 0.7.0 (2025-09-11)

### Features Added

- Added `RequestContent::from_slice()`.
- Added `TryFrom<T> for RequestContent<T, JsonFormat>` for JSON primitives.
- Added support for WASM to the `async_runtime` module.

### Breaking Changes

- Added pipeline configuration options (`PipelineOptions`) to `typespec_client_core::http::Pipeline::new()` to enable customization of the options for an HTTP pipeline.
- Changed `FromStr for RequestContent<T, F>` to `RequestContent::from_str()`.
- Changed `TryFrom<&'static str> for RequestContent<T, F>` to `RequestContent::from_static()`.
- Changed `TryFrom<Bytes> for RequestContent<T, F>` to `From<Bytes> for RequestContent<T, F>` because it was already infallible.
- Removed `TryFrom<Vec<u8>> for RequestContent<T, F>` since `RequestContent::from()` already exists.
- Removed feature `reqwest_rustls_tls`. See [README.md](https://github.com/heaths/azure-sdk-for-rust/blob/main/sdk/typespec/typespec_client_core/README.md) for alternative HTTP client configuration.
- Removed the `fs` module including the `FileStream` and `FileStreamBuilder` types. Moved to `examples/` to copy if needed.
- Removed the `setters` macro.
- Removed the cloud service specific retry headers from typespec_client_core.
- Renamed `RawResponse` to `BufResponse`. New `RawResponse` contains complete body as `Bytes` used in `ErrorKind::HttpResponse`.
- Removed HttpError type from typespec_client_core because it is an azure_core construct.

## 0.6.0 (2025-08-01)

### Features Added

- Added `TryFrom<>` for `RequestContent<T>` implementations for a wider variety of types.

### Breaking Changes

- Added `Format` to `RequestContent<T>` making it `RequestContent<T, F>`. This provides parity with `Response<T, F>` added in version 0.4.0.

- When a retry policy receives a response whose status code indicates the policy shouldn't retry the request, it now returns that response instead of an error

## 0.5.0 (2025-07-10)

### Features Added

- Added `get_async_runtime()` and `set_async_runtime()` to allow customers to replace
the default asynchronous runtime with another.
- Added `Request::set_method()` to allow changing the HTTP method of a request.

### Breaking Changes

- Minimum supported Rust version (MSRV) is now 1.85.
- Removed `Pipeline::replace_policy`.
- Renamed `typespec_client_core::date` to `typespec_client_core::time` and added `typespec_client_core::time::Duration`

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

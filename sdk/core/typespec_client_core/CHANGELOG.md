# Release History

## 0.9.0 (Unreleased)

### Features Added

- Added `UrlExt::append_path()`.

### Breaking Changes

- Added `S` type parameter to `xml::from_xml` congruent with `json::from_json()`.
- Moved deserializers and serializers for optional base64-encoded bytes to `base64::option` module. `base64` module now deserializes or serializes non-optional fields congruent with the `time` module.
- Removed `CustomHeaders` policy.
- Removed `ErrorKind::MockFramework`.
- Removed `xml::read_xml_str()`.
- Renamed `xml::read_xml()` to `xml::from_xml()` congruent with `json::from_json()`.

### Bugs Fixed

### Other Changes

## 0.8.1 (2025-10-06)

### Bugs Fixed

- Fix feature documentation ([#3118](https://github.com/Azure/azure-sdk-for-rust/issues/3118))

## 0.8.0 (2025-10-03)

### Features Added

- Added `AsyncResponse<T>` for responses that may stream the body outside the HTTP pipeline. This replaces `Response<T, F>` requiring an async read of the body that occurred outside the HTTP pipeline.
- Added `Error::with_error_fn()`.
- Added `http::response::BufResponseBody`, which also implements `Stream`.
- Added `RawResponse::deconstruct()`.
- Added `ResponseBody::into_string()`.
- Added `ResponseBody::from_bytes()`.
- Added a `Pipeline::stream()` to return a `Result<BufResponse>`.
- Implemented `AsRef<[u8]>` and `Deref<Target = [u8]>` for `ResponseBody`.

### Breaking Changes

- Changed `ClientOptions::retry` from `Option<RetryOptions>` to `RetryOptions`.
- Changed `DeserializeWith::deserialize_with()` to be sync.
- Changed `Pipeline::send()` to return a `Result<RawResponse>`.
- Changed `RawResponse::body()` to return a `&ResponseBody` instead of `&Bytes`. `ResponseBody` wraps `&Bytes`, and implements `AsRef<[u8]>` and `Deref<Target = [u8]>`.
- Changed `RawResponse::into_body()` to return a `ResponseBody` instead of `Bytes`. `ResponseBody` wraps `&Bytes`, and implements `AsRef<[u8]>` and `Deref<Target = [u8]>`.
- Changed `RawResponse::json()` from `async` to a sync function. The body was already buffered.
- Changed `RawResponse::xml()` from `async` to a sync function. The body was already buffered.
- Changed `Response<T, F>` to fully sync; it holds a `RawResponse` that was already buffered entirely from the service so no longer needs or defines async functions.
- Changed `ResponseBody::json()` and `xml()` to borrow `self`.
- Removed `create_extensible_enum` and `create_enum` macros.
- Removed `BufResponse::json()`.
- Removed `BufResponse::xml()`.
- Removed `CustomHeadersPolicy` from public API.
- Removed `ErrorKind::http_response()`. Construct an `ErrorResponse::HttpResponse` variant instead.
- Removed `ExponentialRetryPolicy` from public API.
- Removed `FixedRetryPolicy` from public API.
- Removed `LoggingPolicy` from public API.
- Removed `NoRetryPolicy` from public API.
- Removed implementation of `Stream` for `ResponseBody`.
- Renamed `TransportOptions::new_custom_policy()` to `Transport::with_policy()`.
- Renamed `TransportOptions` to `Transport`.
- Renamed a number of construction functions for `Error` to align with [guidelines](https://azure.github.io/azure-sdk/rust_introduction.html)
  - Renamed `Error::full()` to `Error::with_error()`.
  - Renamed `Error::with_message()` to `Error::with_message_fn()`.
  - Renamed `Error::message()` to `Error::with_message()`.
  - Renamed `Error::with_context()` to `Error::with_context_fn()`.
  - Renamed `Error::context()` to `Error::with_context()`.
  - Renamed `ResultExt::map_kind()` to `ResultExt::with_kind()`.
  - Renamed `ResultExt::with_context()` to `ResultExt::with_context_fn()`.
  - Renamed `ResultExt::context()` to `ResultExt::with_context()`.
- Replaced implementation of `From<BufResponse>` for `Response<T, F>` to `From<RawResponse>`.
- Replaced implementation of `From<Response<T, F>>` for `BufResponse` to `From<AsyncResponse<T>>`.

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
- Removed feature `reqwest_rustls_tls`. See [README.md](https://github.com/Azure/azure-sdk-for-rust/blob/main/sdk/core/typespec_client_core/README.md) for alternative HTTP client configuration.
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

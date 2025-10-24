# Release History

## 0.30.0 (Unreleased)

### Features Added

- Added `Response::to_raw_response()` function to create a `RawResponse` from cloned data.
- Added `UrlExt::append_path()`.
- Implemented `IntoFuture` for a `Poller`. Call `await` on a Poller to get the final model, or `into_stream()` to get a `futures::Stream` to poll the operation manually.

### Breaking Changes

- Added `F: Format` type parameter to `Poller` and `PollerResult`.
- Added `Format` associated type to `StatusMonitor`.
- Added `Format::deserialize()` function to `Format` trait.
- Added `S` type parameter to `xml::from_xml` congruent with `json::from_json()`.
- Moved deserializers and serializers for optional base64-encoded bytes to `base64::option` module. `base64` module now deserializes or serializes non-optional fields congruent with the `time` module.
- Removed `constants` module.
- Removed `CustomHeaders` policy.
- Removed `ErrorKind::MockFramework`.
- Removed `Poller::wait()` function. Call `await` on a `Poller` to wait for it to complete and, upon success, return the final model.
- Removed `xml::read_xml_str()`.
- Renamed `xml::read_xml()` to `xml::from_xml()` congruent with `json::from_json()`.

### Bugs Fixed

### Other Changes

## 0.29.1 (2025-10-06)

### Breaking Changes

- Removed the `azurite_workaround` feature (unused).

### Bugs Fixed

- Fix feature documentation ([#3118](https://github.com/Azure/azure-sdk-for-rust/issues/3118))

## 0.29.0 (2025-10-03)

### Features Added

- Added `Error::with_error_fn()`.
- Added `AsyncResponse<T>` for responses that may stream the body outside the HTTP pipeline. This replaces `Response<T, F>` requiring an async read of the body that occurred outside the HTTP pipeline.
- Added `http::response::BufResponseBody`, which also implements `Stream`.
- Added a `Pipeline::stream()` to return a `Result<BufResponse>`.
- Added `RawResponse::deconstruct()`.
- Added `ResponseBody::into_string()`.
- Added `ResponseBody::from_bytes()`.
- Added the `cloud` module with types for configuring clients to use different Azure clouds.
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
- Removed several unreferenced HTTP headers and accessor structures for those headers.
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

### Bugs Fixed

- `ErrorKind::HttpResponse { raw_response, .. }` may have been incorrectly `None`.

## 0.28.0 (2025-09-11)

### Features Added

- Added `RawResponse` to `ErrorKind::HttpResponse` that contains the HTTP status code, headers, and complete error response body.
- Added `RequestContent::from_slice()`.
- Added `TokenRequestOptions { method_options: ClientMethodOptions }`.
- Added `TryFrom<T> for RequestContent<T, JsonFormat>` for JSON primitives.
- Added support for WASM to the `async_runtime` module.
- Added logging policy to log HTTP requests and responses in the pipeline. As a part of this change, sanitization support was added to places which log HTTP headers and URLs. The `azure_core::http::ClientOptions` has been enhanced with a `LoggingOptions` which allows a user/service client to specify headers or URL query parameters which should be allowed. Note that the sanitization feature is disabled if you build with the `debug` feature enabled.
- Added support for a new `azure_core::error::http::ErrorResponse` structure which describes an error according to the Azure REST API guidelines.
- Added a new `azure_core::http::check_success(BufResponse)` function to convert a buffered response to an `ErrorKind::HttpResponse`.

### Breaking Changes

- Added a lifetime parameter to `TokenRequestOptions`.
- Added the ability to configure pipeline configuration independently from `ClientOptions`. This adds a new optional `PipelineOptions` parameter to `azure_core::http::Pipeline::new()`. If not specified, it defaults to the expected options for `azure_core` services.
- Changed `FromStr for RequestContent<T, F>` to `RequestContent::from_str()`.
- Changed `TryFrom<&'static str> for RequestContent<T, F>` to `RequestContent::from_static()`.
- Changed `TryFrom<Bytes> for RequestContent<T, F>` to `From<Bytes> for RequestContent<T, F>` because it was already infallible.
- Removed `TryFrom<Vec<u8>> for RequestContent<T, F>` since `RequestContent::from()` already exists.
- Removed feature `reqwest_rustls_tls`. See [README.md](https://github.com/heaths/azure-sdk-for-rust/blob/main/sdk/core/azure_core/README.md) for alternative HTTP client configuration.
- Removed the `fs` module including the `FileStream` and `FileStreamBuilder` types. Moved to `examples/` for `typespec_client_core` to copy if needed.
- Removed the `setters` macro.
- Renamed `RawResponse` to `BufResponse`. New `RawResponse` contains complete body as `Bytes` used in `ErrorKind::HttpResponse`.

## 0.27.0 (2025-08-01)

### Features Added

- Added support for distributed tracing macros in Azure service clients, including convenience macros `#[tracing::new]`, `#[tracing::client]` and `#[tracing::function]` to reduce implementation details. See [distributed tracing in rust](https://github.com/Azure/azure-sdk-for-rust/blob/main/doc/distributed-tracing-for-rust-service-clients.md) for more information on distributed tracing.

### Breaking Changes

- `Pager::from_callback` and `PageIterator::from_callback` define a parameter of type `PagerState<C>` instead of `Option<C>`, where `None` => `Initial` and `Some(C)` => `More(C)`.
- `Poller::from_callback` defines a parameter of type `PollerState<N>` instead of `Option<N>`, where `None` => `Initial` and `Some(N)` => `More(N)`.

## 0.26.0 (2025-07-10)

### Features Added

- Added `get_async_runtime()` and `set_async_runtime()` to allow customers to replace the asynchronous runtime used by the Azure SDK.
- Added `PageIterator::continuation_token()` and `PageIterator::with_continuation_token()` to support reconstructing a `PageIterator` in another process or on another machine to continue paging.
- Added `Poller<T>` for long-running operations (LROs).
- Added `Request::set_method()` to allow changing the HTTP method of a request.
- Added `StatusMonitor` for long-running operations.

### Breaking Changes

- Added `http::PollerOptions` parameter to `http::poller::get_retry_after`.
- Implemented `FromStr` where `FromStr::Err = Infallible` for `PollerStatus` instead of `From<&str>`.
- Minimum supported Rust version (MSRV) is now 1.85.
- `azure_core::http::Pipeline::new` now takes an `azure_core::http::ClientOptions` which is defined in `azure_core`, but convertible to `typespec_client_core::http::ClientOptions`.
- Moved `process::Executor` to `azure_identity`.
- Removed `Pipeline::replace_policy`.
- Removed unused `location` and `body` modules from `http::poller`.
- Renamed `azure_core::date` to `azure_core::time` and added `azure_core::time::Duration` as the standard "duration" type for the SDK.
- Renamed `http::poller::body_content` to `http::poller::body`.
- Renamed `PagerResult::More { next }` to `continuation`.
- Renamed `PollerStatus::Other` to `PollerStatus::UnknownValue` following [guidelines](https://azure.github.io/azure-sdk/rust_introduction.html#rust-enum-extensible).
- Renamed `TelemetryOptions` to `UserAgentOptions`.
- Renamed `TelemetryPolicy` to `UserAgentPolicy`.

### Other Changes

- The `CustomHeadersPolicy` is executed after the retry policy in the `Pipeline`.

## 0.25.0 (2025-06-06)

### Features Added

- Added `#[safe]` attribute helper for `SafeDebug` derive macro to show or hide types and members as appropriate.
- Added `Page` trait to facilitate the `ItemIterator`.
- Added `PageIterator` to asynchronously iterate all pages.

### Breaking Changes

- A `Pager` now asynchronously iterates over items across all pages. Call `Pager::into_pages()` to get a `PageIterator` to asynchronously iterate over all pages.
- Removed `AccessToken::is_expired()`.
- Renamed `PagerResult::Continue` to `More` and its `continuation` field to `next`.
- Renamed `PagerResult::Complete` to `Done`.
- Renamed `PageStream` to `ItemIterator`.

### Bugs Fixed

- `BearerTokenCredentialPolicy` returns an error when a proactive token refresh attempt fails

## 0.24.0 (2025-05-02)

### Features Added

- Added `TaskSpawner` abstraction to spawn asynchronous tasks for different async runtimes. Defaults to optional `tokio` runtime.

### Breaking Changes

- `PagerResult` always returns items of type `T` instead of `Response<T>`.

### Other Changes

- Deriving `SafeDebug` formats non-exhaustive types by default. Enable `debug` feature to format normal `Debug` output.
- Updated dependencies.

## 0.23.0 (2025-04-08)

### Features Added

- Added `azure_core::process::Executor` to run commands asynchronously.
  The `tokio` feature is disabled by default so `std::process::Command` is used; otherwise, if enabled, `tokio::process::Command` is used.
- Added `http` module containing all functions, modules, and types from `typespec_client_core::http`.
- Added `azure_core::http::policies::ClientRequestIdPolicy` to every pipeline. Client libraries can add with custom header name instead.
- Moved `Pager` from `typespec_client_core::http` to `azure_core::http` module since it is Azure-specific.
- Re-exported `Body`, `Request`, and `RequestContent` from `http::request` module.
- Re-exported `create_enum`, `create_extensible_enum` macros from `typespec_client_core`.
- Re-exported `Model` and `Response` from `http::response` module.

### Breaking Changes

- Removed `azure_core::credentials::TokenCredential::clear_cache()`
- Consolidated all the `tokio` features into a single feature named `tokio`. Traits remain separate but `tokio` support is enabled with a single feature.
- Moved `AppendToUrlQuery` type under `http` module.
- Moved `ByteStream` and `SeekableStream` types under `stream` module.
- Moved `ClientMethodOptions` type under `http` module.
- Moved `ClientOptions` type under `http` module.
- Moved `Context` type under `http` module.
- Moved `Etag` type under `http` module.
- Moved `ExponentialRetryOptions` type under `http` module.
- Moved `FixedRetryOptions` type under `http` module.
- Moved `headers` module under `http` module.
- Moved `HttpClient` type under `http` module.
- Moved `LeaseAction` type under `http` module.
- Moved `LeaseDuration` type under `http` module.
- Moved `LeaseState` type under `http` module.
- Moved `LeaseStatus` type under `http` module.
- Moved `Method` type under `http` module.
- Moved `new_http_client` function under `http` module.
- Moved `Pipeline` type under `http` module.
- Moved `policies` module under `http` module.
- Moved `request` module under `http` module.
- Moved `response` module under `http` module.
- Moved `RetryOptions` type under `http` module.
- Moved `StatusCode` type under `http` module.
- Moved `TelemetryOptions` type under `http` module.
- Moved `TransportOptions` type under `http` module.
- Moved `Url` type under `http` module.
- Removed `content_type` module.
- Removed `EMPTY_BODY` constant from root.
- Removed `future!()` macro.
- Removed `Header` re-export from `http` module. It is still defined in the `http::headers` module.
- Removed `parsing` module.
- Removed `query_param` module.
- Removed `RequestId` type alias from root.
- Removed `SessionToken` type alias from root.
- Renamed `lro` module to `http::poller` module.
- Renamed `lro` module types with prefix "Lro" to prefix "Poller".
- Renamed `tokio` module to `fs` since it contained only the `typespec_client_core::fs` module members.

## 0.22.0 (2025-02-18)

### Features Added

- Initial supported release.

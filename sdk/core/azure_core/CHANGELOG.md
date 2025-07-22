# Release History

## 0.27.0 (Unreleased)

### Features Added

### Breaking Changes

### Bugs Fixed

### Other Changes

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

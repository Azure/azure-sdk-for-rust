# Release History

## 0.10.0 (Unreleased)

### Features Added

### Breaking Changes

- Added `S` type parameter to `xml::from_xml` congruent with `json::from_json()`.
- Removed `ErrorKind::MockFramework`.
- Removed `xml::read_xml_str()`.
- Renamed `xml::read_xml()` to `xml::from_xml()` congruent with `json::from_json()`.

### Bugs Fixed

### Other Changes

## 0.9.1 (2025-10-06)

### Bugs Fixed

- Fix feature documentation ([#3118](https://github.com/Azure/azure-sdk-for-rust/issues/3118))

## 0.9.0 (2025-10-03)

### Features Added

- Added `Error::with_error_fn()`.
- Added `http::response::ResponseBody`.
- Added `RawResponse::deconstruct()`.
- Added `ResponseBody::from_bytes()`.

### Breaking Changes

- Changed `RawResponse::body()` to return a `&ResponseBody` instead of `&Bytes`. `ResponseBody` wraps `&Bytes`, and implements `AsRef<[u8]>` and `Deref<Target = [u8]>`.
- Changed `RawResponse::into_body()` to return a `ResponseBody` instead of `Bytes`. `ResponseBody` wraps `&Bytes`, and implements `AsRef<[u8]>` and `Deref<Target = [u8]>`.
- Changed `RawResponse::json()` from `async` to a sync function. The body was already buffered.
- Changed `RawResponse::xml()` from `async` to a sync function. The body was already buffered.
- Moved `AsHeaders`, `FromHeaders`, `Header`, `Headers`, `HeaderName`, and `HeaderValue` to `http::headers` module to align with `typespec_client_core`.
- Removed `ErrorKind::http_response()`. Construct an `ErrorResponse::HttpResponse` variant instead.
- Renamed a number of construction functions for `Error` to align with [guidelines](https://azure.github.io/azure-sdk/rust_introduction.html)
  - Renamed `Error::full()` to `Error::with_error()`.
  - Renamed `Error::with_message()` to `Error::with_message_fn()`.
  - Renamed `Error::message()` to `Error::with_message()`.
  - Renamed `Error::with_context()` to `Error::with_context_fn()`.
  - Renamed `Error::context()` to `Error::with_context()`.
  - Renamed `ResultExt::map_kind()` to `ResultExt::with_kind()`.
  - Renamed `ResultExt::with_context()` to `ResultExt::with_context_fn()`.
  - Renamed `ResultExt::context()` to `ResultExt::with_context()`.

### Other Changes

- Made `http::headers` a public module to align with `typespec_client_core`.
- Made `http::response` a public module to align with `typespec_client_core`.

## 0.8.0 (2025-09-11)

### Features Added

- Added `Bytes` (moved from `typespec_client_core`).
- Added `HeaderName::is_standard()`.
- Added `json` module if the `json` feature is enabled (moved from `typespec_client_core`).
- Added `RawResponse` to `ErrorKind::HttpResponse` that contains the HTTP status code, headers, and complete error response body.
- Added `xml` feature.
- Added `xml` module if the `xml` feature is enabled (moved from `typespec_client_core`).

### Other Changes

- Updated dependencies.

## 0.7.0 (2025-08-01)

### Breaking Changes

- `http::Request::method()` returns a copy of the underlying `Method` rather than a reference to the underlying `Method`.

## 0.6.0 (2025-07-10)

### Breaking Changes

- Minimum supported Rust version (MSRV) is now 1.85.

## 0.5.0 (2025-06-06)

## Other Changes

- Updated dependencies.

## 0.4.0 (2025-05-02)

### Other Changes

- Updated dependencies.

## 0.3.0 (2025-04-08)

### Breaking Changes

- Removed `http-types` dependency and implemented `StatusCode` instead.

## 0.2.0 (2025-02-18)

### Features Added

- Initial supported release.

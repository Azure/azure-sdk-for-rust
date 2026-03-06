# Release History

## 0.4.0 (Unreleased)

### Features Added

- Default Azure Storage logging configuration (allowed headers and query parameters) is now automatically applied to all Queue clients.
- Added support for `set_access_policy` and `get_access_policy` to `QueueClient`.

### Breaking Changes

- Changed `QueueClient::set_metadata()` `metadata` parameter from owned `HashMap<String, String>` to `&HashMap<String, String>`.
- Support for `wasm32-unknown-unknown` has been removed ([#3377](https://github.com/Azure/azure-sdk-for-rust/issues/3377))
- `SentMessage` ownership semantics changed, and code that previously moved fields like `message_id`/`pop_receipt` may require `clone()` (or borrowing) updates.

### Bugs Fixed

- Fixed an issue where user-provided `per_try_policies` in `ClientOptions` were ignored when constructing any Queue Storage client.

### Other Changes

## 0.3.0 (2026-02-11)

### Features Added

- Added support for queue client construction directly from URLs: `QueueClient::from_url()`
- Added support for SAS (shared access signature) URLs via the new `from_url()` methods.
- Added `continuation_token` to `PagerOptions` for methods that return a `Pager`.

### Breaking Changes

- Changed `QueueClient::set_access_policy()` return type from `Response<QueueClientSetAccessPolicyResult, NoFormat>` to `Response<(), NoFormat>`.
- Changed our minimum supported Rust version (MSRV) from 1.85 to 1.88.
- Removed the `queue_name()` accessor on `QueueClient`.
- Removed the `endpoint` struct field on all clients, as this value is now returned directly from the underlying generated client.
- Changed the `queue_name` parameter from owned `String` to `&str` reference on `QueueClient::new()`.
- The `credential` parameter is now `Option<Arc<dyn TokenCredential>>` on `new()` and `from_url()` client constructors, allowing for construction of public access clients and clients using SAS tokens.
- Changed `QueueServiceClient::queue_client()` to return `Result<QueueClient>` instead of `QueueClient`.
- Removed `Pager::with_continuation_token()` for methods that return a `Pager`.

## 0.2.0 (2025-11-11)

### Breaking Changes

- Changed `QueueServiceClientListQueuesOptions::method_options` from `ClientMethodOptions` to `PagerOptions`
- Renamed `Response<T, F>::into_body(self) -> Result<Response<T>>` to `into_model(self) -> Result<Response<T>>`. `into_body(self)` now returns a `ResponseBody`.

## 0.1.0 (2025-10-15)

### Features Added

- Initial supported release.

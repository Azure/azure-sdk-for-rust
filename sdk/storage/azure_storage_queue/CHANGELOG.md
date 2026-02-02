# Release History

## 0.4.0 (Unreleased)

### Features Added

- Added support for client construction directly from URLs:
  - `QueueClient::from_url()`
  - `QueueServiceClient::from_url()`
- Added support for SAS (shared access signature) URLs via the new `from_url()` methods.

### Breaking Changes

- Removed the `queue_name()` accessor on `QueueClient`.
- Removed the `endpoint` struct field on all clients, as this value is now returned directly from the underlying generated client.
- Changed the `queue_name` parameter from owned `String` to `&str` reference on `QueueClient::new()`.
- The `credential` parameter is now `Option<Arc<dyn TokenCredential>>` on `new()` and `from_url()` client constructors, allowing for construction of public access clients and clients using SAS tokens.
- Changed `QueueServiceClient::queue_client()` to return `Result<QueueClient>` instead of `QueueClient`.

### Bugs Fixed

### Other Changes

## 0.3.0 (Unreleased)

### Features Added

- Added `continuation_token` to `PagerOptions` for methods that return a `Pager`.

### Breaking Changes

- Removed `Pager::with_continuation_token()` for methods that return a `Pager`.

### Bugs Fixed

### Other Changes

## 0.2.0 (2025-11-11)

### Breaking Changes

- Changed `QueueServiceClientListQueuesOptions::method_options` from `ClientMethodOptions` to `PagerOptions`
- Renamed `Response<T, F>::into_body(self) -> Result<Response<T>>` to `into_model(self) -> Result<Response<T>>`. `into_body(self)` now returns a `ResponseBody`.

## 0.1.0 (2025-10-15)

### Features Added

- Initial supported release.

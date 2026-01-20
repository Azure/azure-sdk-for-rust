# Release History

## 0.8.0 (Unreleased)

### Features Added

- Added support for `set_access_policy` to `BlobContainerClient`.
- Added support for `get_access_policy` to `BlobContainerClient`.
- Added support for `set_legal_hold` to `BlobClient`.
- Added support for `set_immutability_policy` to `BlobClient`.
- Added support for `delete_immutability_policy` to `BlobClient`.
- Added support for `undelete` to `BlobClient`.
- Added snapshot and versioning support for blobs with convenience methods `with_snapshot` and `with_version` to `BlobClient`.

### Breaking Changes

- Changed conversion implementation from `BlobTags` to `HashMap<String, String>` from `TryFrom` to `From`.
- Added `continuation_token` to `PagerOptions` for methods that return a `Pager`.
- Renamed `content_length` to `size` for `PageBlobClient`'s `create()` method.
- Removed `Pager::with_continuation_token()` for methods that return a `Pager`.
- Changed `BlobClient`'s `set_tags` parameter `tags` type from `HashMap<String, String>` to `RequestContent<BlobTags, XmlFormat>`.

### Bugs Fixed

### Other Changes

## 0.7.0 (2025-11-11)

### Features Added

- Added support for client construction directly from URLs:
  - `AppendBlobClient::from_url()`
  - `BlobClient::from_url()`
  - `BlobContainerClient::from_url()`
  - `BlockBlobClient::from_url()`
  - `PageBlobClient::from_url()`
- Added support for SAS (shared access signature) URLs via the new `from_url()` methods.

### Breaking Changes

- Changed the following options structs' `method_options` from `ClientMethodOptions` to `PagerOptions`:
  - `BlobContainerClientListBlobFlatSegmentOptions`
  - `BlobContainerClientListBlobHierarchySegmentOptions`
  - `BlobServiceClientListContainersSegmentOptions`

- Removed the `container_name()` and `blob_name()` accessors on relevant clients.
- Removed the `endpoint` struct field on all clients, as this value is now returned directly from the underlying generated client.
- Changed the `container_name` and `blob_name` parameters from owned `String` to `&str` reference on relevant client constructor methods (`new()`).
- The `credential` parameter is now `Optional` on `new()` client constructors, allowing for construction of public access clients.
- Renamed `Response<T, F>::into_body(self) -> Result<Response<T>>` to `into_model(self) -> Result<Response<T>>`. `into_body(self)` now returns a `ResponseBody`.

## 0.6.0 (2025-10-06)

### Features Added

- Added support for `exists` to `BlobClient` and `BlobContainerClient`.

### Breaking Changes

- Changed `BlobClient::download()` to return an `AsyncResponse` instead of a `Response`. This allows for streaming large payloads outside the internal HTTP pipeline.
- Client methods that return a `Response<T>>` asynchronously buffer the entire model within the internal pipeline, so `into_body()` and other methods on the response are no longer async.

## 0.5.0 (2025-09-15)

### Features Added

- Added support for `set_properties` to `BlobServiceClient`.
- Added support for `filter_blobs` to `BlobContainerClient` and `BlobServiceClient`.
- Added support for `set_tags` and `get_tags` to `BlobClient`.
- Added support for `get_account_info` to `BlobClient`, `BlobContainerClient`, and `BlobServiceClient`.
- Added support for `upload_blob_from_url` to `BlockBlobClient`.
- Added support for `get_page_ranges`, `update_sequence_number`, and `upload_pages_from_url` to `PageBlobClient`.
- Added support for `find_blobs_by_tags` to `BlobContainerClient` and `BlobServiceClient`.

### Breaking Changes

- Made `metadata` a required parameter for `set_metadata` for `BlobContainerClient` and `BlobClient`.

## 0.4.0 (2025-08-05)

### Bugs Fixed

- Fixed an issue where the `Etag` field in `BlobPropertiesInternal` was not deserialized properly.
- Re-exported previously inaccessible models referenced in options bags and response models for currently implemented features.

### Breaking Changes

- Changed `RequestContent<Bytes>` request methods to `RequestContent<T, NoFormat>` to provide parity with `Response<T, NoFormat>`.
- Changed `RequestContent<T>` XML models in client methods to `RequestContent<T, XmlFormat>` to provide parity with `Response<T, XmlFormat>`.

## 0.3.0 (2025-07-15)

### Features Added

- Added lease support (`acquire_lease`, `break_lease`, `change_lease`, `release_lease`, and `renew_lease`) to `BlobContainerClient` and `BlobClient`.
- Added two new blob client types, `AppendBlobClient` and `PageBlobClient`.
- Added support for `list_containers` to `BlobServiceClient`.

## 0.2.0 (2025-06-10)

### Features Added

- Added a new blob client type, `BlockBlobClient`.
- Added navigation methods to access sub-clients from existing clients:
  - `BlobServiceClient::blob_container_client()`
  - `BlobContainerClient::blob_client()`
  - `BlobClient::block_blob_client()`

- Added support for `list_blobs`, `set_metadata` to `BlobContainerClient`.
- Added support for `set_metadata`, `set_properties`, and `set_tier` to `BlobClient`.

### Breaking Changes

- Moved `commit_block_list`, `get_block_list`, and `stage_block` from `BlobClient` to `BlockBlobClient`.

### Bugs Fixed

- Fixed an issue where the blob type string would appear as a query parameter in the request URL for certain APIs extraneously.

## 0.1.0 (2025-04-08)

### Features Added

- Initial supported release.

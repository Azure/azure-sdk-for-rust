# Release History

## 0.5.0 (2025-09-15)

### Features Added

* Added support for `set_properties` to `BlobServiceClient`.
* Added support for `filter_blobs` to `BlobContainerClient` and `BlobServiceClient`.
* Added support for `set_tags` and `get_tags` to `BlobClient`.
* Added support for `get_account_info` to `BlobClient`, `BlobContainerClient`, and `BlobServiceClient`.
* Added support for `upload_blob_from_url` to `BlockBlobClient`.
* Added support for `get_page_ranges`, `update_sequence_number`, and `upload_pages_from_url` to `PageBlobClient`.
* Added support for `find_blobs_by_tags` to `BlobContainerClient` and `BlobServiceClient`.

### Breaking Changes

* Made `metadata` a required parameter for `set_metadata` for `BlobContainerClient` and `BlobClient`.

## 0.4.0 (2025-08-05)

### Bugs Fixed

* Fixed an issue where the `Etag` field in `BlobPropertiesInternal` was not deserialized properly.
* Re-exported previously inaccessible models referenced in options bags and response models for currently implemented features.

### Breaking Changes

* Changed `RequestContent<Bytes>` request methods to `RequestContent<T, NoFormat>` to provide parity with `Response<T, NoFormat>`.
* Changed `RequestContent<T>` XML models in client methods to `RequestContent<T, XmlFormat>` to provide parity with `Response<T, XmlFormat>`.

## 0.3.0 (2025-07-15)

### Features Added

* Added lease support (`acquire_lease`, `break_lease`, `change_lease`, `release_lease`, and `renew_lease`) to `BlobContainerClient` and `BlobClient`.
* Added two new blob client types, `AppendBlobClient` and `PageBlobClient`.
* Added support for `list_containers` to `BlobServiceClient`.

## 0.2.0 (2025-06-10)

### Features Added

* Added a new blob client type, `BlockBlobClient`.
* Added navigation methods to access sub-clients from existing clients:
  * `BlobServiceClient::blob_container_client()`
  * `BlobContainerClient::blob_client()`
  * `BlobClient::block_blob_client()`

* Added support for `list_blobs`, `set_metadata` to `BlobContainerClient`.
* Added support for `set_metadata`, `set_properties`, and `set_tier` to `BlobClient`.

### Breaking Changes

* Moved `commit_block_list`, `get_block_list`, and `stage_block` from `BlobClient` to `BlockBlobClient`.

### Bugs Fixed

* Fixed an issue where the blob type string would appear as a query parameter in the request URL for certain APIs extraneously.

## 0.1.0 (2025-04-08)

### Features Added

* Initial supported release.

# Release History

## 0.3.0 (Unreleased)

### Features Added

### Breaking Changes

### Bugs Fixed

### Other Changes

## 0.2.0 (2025-06-10)

### Features Added

* Added a new blob client type, `BlockBlobClient`.
* Added navigation methods to access sub-clients from existing clients:
  * `BlobServiceClient::blob_container_client()`
  * `BlobContainerClient::blob_client()`
  * `BlobClient::block_blob_client()`

* Added support for `list_blobs`, `set_metadata` to `ContainerClient`.
* Added support for `set_metadata`, `set_properties`, and `set_tier` to `BlobClient`.

### Breaking Changes

* Moved `commit_block_list`, `get_block_list`, and `stage_block` from `BlobClient` to `BlockBlobClient`.

### Bugs Fixed

* Fixed an issue where the blob type string would appear as a query parameter in the request URL for certain APIs extraneously.

## 0.1.0 (2025-04-08)

### Features Added

* Initial supported release.

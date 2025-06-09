# Release History

## 0.2.0 (2025-06-09)

### Features Added

* Added navigation methods to access sub-clients from existing clients.
* Added a new blob client type, `BlockBlobClient`.
* Added support for `list_blobs`, `set_metadata` to `ContainerClient`.
* Added support for `set_metadata`, `set_properties`, and `set_tier` to `BlobClient`.

### Breaking Changes

* Moved `commit_block_list`, `get_block_list`, and `stage_block` from `BlobClient` to `BlockBlobClient`.

### Bugs Fixed

* Fixed an issue where blob type would be present in RequestURI for certain APIs extraneously.

## 0.1.0 (2025-04-08)

### Features Added

* Initial supported release.

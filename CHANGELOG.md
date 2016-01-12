# Change Log


## [0.0.7](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.0.7) (yet-to-publish)

**Implemented features:**
* Added support for ```filter``` in list_blobs. Now you can filter the blobs to find specifying the starting string.

** Removed methods:**
* Removed ```ListBlobOptions::new``` as it was just useless boilerplate.

## [0.0.6](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.0.6) (2016-01-12)

**Implemented features:**
* Added support for max_results in list_blobs. Now you can limit how many blobs could be returned by a single call.
* Added support for next_marker in list_blobs. Now you can continue enumerating your blobs in subsequent calls.
* Added put page for page blobs.
* Added clear page for page blobs.

**Refactoring:**
* Added page constraints (512-bytes aligned).
* Most methods moved from storage::Client to correct structs (ie storage::container::Container and storage::blob::Blob).
* Moved list_blobs options in a separate structure (```azure::storage::blob::ListBlobOptions```).

## [0.0.5](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.0.5) (2016-01-05)

**Implemented features:**
* List blobs
* Get blob
* Put blob

## [0.0.4](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.0.4) (2015-12-15)

**Implemented features:**
* Create container
* Delete container

## [0.0.3](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.0.3) (2015-12-14)

Initial release

**Implemented features:**
* Shared key authentication
* List containers

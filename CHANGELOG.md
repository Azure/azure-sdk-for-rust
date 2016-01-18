git# Change Log

## [0.1.1](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.0.7) (2016-01-18)

**Implemented features:**
* Lease blob (https://msdn.microsoft.com/library/azure/ee691972.aspx).

**Refactoring:**
* Renamed ```azure::core::lease_id``` module in ```azure::core::lease```.
* Moved lease enumerations in ```azure::core::lease``` module.

**Bugfixes:**
* Added the non-doc option for the bin test file:
```rust
[[bin]]
name = "main"
doc = false
```

## [0.1.0](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.0.8) (2015-01-16)

**Implemented features:**
* Added crate exports in ```lib.rs```.

## [0.0.7](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.0.7) (2015-01-14)

**Implemented features:**
* Added support for ```put_block``` in ```Blob```.
* Added support for ```filter```  in ```Blob::list```. Now you can filter the blobs to find specifying the starting string.
* Added support for ```timeout``` in ```Blob::list```.
* Added support for ```timeout```, ```prefix``` and ```max_results```  in ```Container::list```.
* Added support for ```max_results``` in ```Container::list```. Now you can limit how many containers could be returned by a single call.
* Added support for ```next_marker``` in ```Container::list```. Now you can continue enumerating your containers in subsequent calls.

**Refactoring:**
* Moved ```Container::list``` options in a separate structure (```azure::storage::container::ListContainerOptions```).
* Moved ```Blob::put_page``` options in a separate structure (```azure::storage::blob::PutPageOptions```).

**Bugfixes:**
* Corrected the format bug in ```azure::core::range::Range``` and ```azure::core::range::ba512_range::BA512Range```. Previously the string returned was
formatted as ```{}/{}``` which is invalid for the ```x-ms-range``` header. Now the format is ```bytes={}-{}``` as expected. I still need to figure out if
  I need to change the ```FromStr``` trait too to match the change.

**Removed methods:**
* Removed ```ListBlobOptions::new``` as it was just useless boilerplate.

## [0.0.6](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.0.6) (2016-01-12)

**Implemented features:**
* Added support for max_results in ```Blob::list```. Now you can limit how many blobs could be returned by a single call.
* Added support for next_marker in ```Blob::list```. Now you can continue enumerating your blobs in subsequent calls.
* Added ```put page``` for page blobs.
* Added ```clear page``` for page blobs.

**Refactoring:**
* Added page constraints (512-bytes aligned).
* Most methods moved from ```storage::Client``` to correct structs (ie ```storage::container::Container``` and ```storage::blob::Blob```).
* Moved ```Blob::list``` options in a separate structure (```azure::storage::blob::ListBlobOptions```).

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

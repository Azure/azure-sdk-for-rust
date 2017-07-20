## [0.4.4](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.4.4) (2017-07-20)

**Implemented features:**

* Added Cosmos query document (both a plan ```JSON``` and ```DeserializeOwned``` implementing structs).
* Added more examples.

## [0.4.3](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.4.3) (2017-07-07)

**Breaking changes:**

* Changed return type from tuple to single struct to make methods more ergonomic. Issue [https://github.com/MindFlavor/AzureSDKForRust/issues/26](https://github.com/MindFlavor/AzureSDKForRust/issues/26).

## [0.4.2](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.4.2) (2017-07-05)

**Bugfix:**

* Corrected a type mistake in multiple ```AsRef<str>``` parameters.
* Implemented ```AsRef<str>``` in Cosmos document and collection.
* Fixed type mismatch in examples (in README.md too).

## [0.4.1](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.4.1) (2017-07-05)

**Implemented features:**

* Added Cosmos add document (both a plan ```&str``` and ```Serialize``` implementing struct).
* Added Cosmos list documents method.
* Added Cosmos get document method.
* Added more examples.

## [0.4.0](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.4.0) (2017-07-02)

### Migrated all code to asynchronous hyper using Futures 

**Breaking changes**

* Almost everything is now a future. So whenever you had a ```Result<A,AzureError>``` now you have ```impl FutureResult<A,AzureError>```.

**Updated references to bleeding edge**

**TODO**

* Test the table code.
* Perform the E2E test.

## [0.3.1](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.3.1) (2017-06-10)

**Implemented features:**

* Added Cosmos' collection structures to interact with Cosmos collections.
* Added Cosmos' create collection method.
* Added Cosmos' list collections method.
* Added Cosmos' get collection method.
* Added Cosmos' delete collection method.
* Added Cosmos' replace collection method.

**Updated references to bleeding edge**

## [0.3.0](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.3.0) (2017-06-05)

**Implemented features:**

* Added Cosmos support. Right now only database commands are implemented but the authentication frawework is in place. Both ```master keys``` and ```resource tokens``` are supported (see [https://docs.microsoft.com/it-it/rest/api/documentdb/access-control-on-documentdb-resources?redirectedfrom=MSDN](https://docs.microsoft.com/it-it/rest/api/documentdb/access-control-on-documentdb-resources?redirectedfrom=MSDN) for more details). 

* Added Cosmos' client struct to interact with a Cosmos instance.
* Added Cosmos' create database method.
* Added Cosmos' list databases method.
* Added Cosmos' get database method.
* Added Cosmos' delete database method.

## [0.2.1](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.2.1) (2017-05-04)

**Bugfix:**
* Corrected a bug in ```BlockBlob``` put command that would not allow sending data. For more details see [karataliu](https://github.com/karataliu)'s pull request [#16](https://github.com/MindFlavor/AzureSDKForRust/pull/16).

**Enhancements:**
* Expanded ```BlockBlob``` end-to-end tests.

## [0.2.0](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.2.0) (2017-05-04)

**Refactoring:**
* Upgraded all the references to the state of the art. This includes Hyper with external TLS support.
* Removed ```rustc-serialize``` crate as it's now officially deprecated. Azure SDK for Rust now uses [Serde](https://github.com/serde-rs/serde). This closes issue [12](https://github.com/MindFlavor/AzureSDKForRust/issues/12).
* Added more end-to-end tests.
* Clippy'd all the code.

## 0.1.4-beta (2017-04-24)

**Implemented features:**
* Added azure table support thanks to this huge [pull request](https://github.com/MindFlavor/AzureSDKForRust/pull/11) from [Dong Liu](https://github.com/karataliu). 
* Added end-to-end tests.

## [0.1.3](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.1.3) (2016-02-05)

**Implemented features:**
* Added Event Hub Send Event REST API implementation (as for [https://msdn.microsoft.com/en-us/library/azure/dn790664.aspx](https://msdn.microsoft.com/en-us/library/azure/dn790664.aspx)).

**Refactoring:**
* Corrected  ```azure::core::errors::AzureError``` to include the ``std::io::Error`` and ```xml::BuilderError``` instead of their ```to_string()``` result.
* Changed ```UnexpectedResult``` enum of ```azure::core::errors::AzureError``` to ```UnexpectedHTTPResult``` which holds an instance of  ```azure::core::errors::UnexpectedHTTPResult``` instead of a tuple for better field description.

## [0.1.2](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.1.2) (2016-01-18)

**Bugfixes:**
* Added ```std::ops::DerefMut``` trait to ```azure::core::incompletevector::IncompleteVector```.
* Corrected [README](README.md) sample.

**Refactoring:**
* Renamed ```azure::core::ba512_range::BA512Range::len()``` function in ```azure::core::ba512_range::BA512Range::size()``` for better linting (len requires is_empty
  and was therefore misleading).

## [0.1.1](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.1.1) (2016-01-18)

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

## [0.1.0](https://github.com/MindFlavor/AzureSDKForRust/releases/tag/0.1.0) (2015-01-16)

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

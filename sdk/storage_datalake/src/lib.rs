/*!
# The Storage Datalake crate

`azure-storage-datalake` offers functionality needed to interact with an ADLS Gen2 storage account from Rust.
As an abstraction over the [Azure Data Lake Store REST API](https://docs.microsoft.com/en-us/rest/api/storageservices/data-lake-storage-gen2), anything that is possible through that Rest API
should also be possible with this crate.

## Examples

TODO
*/
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_core;

pub mod clients;
pub mod file_system;
pub mod operations;
pub mod prelude;
mod properties;
pub mod request_options;
mod util;

pub use azure_core::error::{Error, Result};
pub use file_system::FileSystem;
pub use properties::Properties;

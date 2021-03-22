#![recursion_limit = "256"]
#![allow(clippy::needless_lifetimes)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate azure_core;
#[macro_use]
extern crate quick_error;

#[cfg(feature = "blob_storage")]
pub mod blob_storage;
pub mod core;
#[cfg(feature = "data_lake_storage")]
pub mod data_lake_storage;
#[cfg(feature = "queue_storage")]
pub mod queue_storage;
#[cfg(feature = "storage_account")]
pub mod storage_account;
#[cfg(feature = "table_storage")]
pub mod table_storage;

pub use crate::core::*;
#[cfg(feature = "blob_storage")]
pub use blob_storage::*;
#[cfg(feature = "data_lake_storage")]
pub use data_lake_storage::*;
#[cfg(feature = "queue_storage")]
pub use queue_storage::*;
#[cfg(feature = "storage_account")]
pub use storage_account::*;
#[cfg(feature = "table_storage")]
pub use table_storage::*;

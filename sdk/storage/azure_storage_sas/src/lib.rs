#![doc = include_str!("../README.md")]

mod key;
pub(crate) mod resource;
pub(crate) mod sas;

#[cfg(test)]
pub(crate) mod test_utils;

pub use azure_storage_blob::models::UserDelegationKey;
pub use resource::{blob, file, queue, table, Resource};
pub use sas::builder::UserDelegationSasBuilder;
pub use sas::SignedProtocol;

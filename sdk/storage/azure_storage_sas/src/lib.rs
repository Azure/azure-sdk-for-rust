#![doc = include_str!("../README.md")]

mod key;
pub(crate) mod resource;
pub(crate) mod sas;

#[cfg(test)]
pub(crate) mod test_utils;

pub mod error;
pub use key::{UserDelegationKey, UserDelegationKeyFetcher};
pub use resource::{blob, file, queue, table, Resource};
pub use sas::builder::UserDelegationSasBuilder;
pub use sas::SignedProtocol;

/// Re-export of the [`time`] crate.
pub use time;
/// Re-export of the [`url`] crate.
pub use url;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(all(feature = "v1", not(feature = "no-default-tag")))]
pub use v1::{models, Client, ClientBuilder};
#[cfg(feature = "20210519")]
pub mod v20210519;
#[cfg(all(feature = "20210519", not(feature = "no-default-tag")))]
pub use v20210519::{models, Client, ClientBuilder};
#[cfg(feature = "20171001")]
pub mod v20171001;
#[cfg(all(feature = "20171001", not(feature = "no-default-tag")))]
pub use v20171001::{models, Client, ClientBuilder};

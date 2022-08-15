#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2019-04-01")]
pub mod package_2019_04_01;
#[cfg(all(feature = "package-2019-04-01", not(feature = "no-default-tag")))]
pub use package_2019_04_01::{models, Client, ClientBuilder};

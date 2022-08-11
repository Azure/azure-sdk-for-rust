#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2018-03-01-beta")]
pub mod package_2018_03_01_beta;
#[cfg(all(feature = "package-2018-03-01-beta", not(feature = "no-default-tag")))]
pub use package_2018_03_01_beta::{models, Client, ClientBuilder};

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-03-08")]
pub mod package_2021_03_08;
#[cfg(all(feature = "package-2021-03-08", not(feature = "no-default-tag")))]
pub use package_2021_03_08::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-01-11")]
pub mod package_2021_01_11;
#[cfg(all(feature = "package-2021-01-11", not(feature = "no-default-tag")))]
pub use package_2021_01_11::{models, Client, ClientBuilder};

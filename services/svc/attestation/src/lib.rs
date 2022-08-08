#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2020-10-01")]
pub mod package_2020_10_01;
#[cfg(all(feature = "package-2020-10-01", not(feature = "no-default-tag")))]
pub use package_2020_10_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-09-01")]
pub mod package_2018_09_01;
#[cfg(all(feature = "package-2018-09-01", not(feature = "no-default-tag")))]
pub use package_2018_09_01::{models, Client, ClientBuilder};

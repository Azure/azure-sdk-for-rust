#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2021-12-01")]
pub mod package_2021_12_01;
#[cfg(all(feature = "package-2021-12-01", not(feature = "no-default-tag")))]
pub use package_2021_12_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-06-01")]
pub mod package_2021_06_01;
#[cfg(all(feature = "package-2021-06-01", not(feature = "no-default-tag")))]
pub use package_2021_06_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-03-20")]
pub mod package_2020_03_20;
#[cfg(all(feature = "package-2020-03-20", not(feature = "no-default-tag")))]
pub use package_2020_03_20::{models, Client, ClientBuilder};

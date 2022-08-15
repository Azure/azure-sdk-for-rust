#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2020-06")]
pub mod package_2020_06;
#[cfg(all(feature = "package-2020-06", not(feature = "no-default-tag")))]
pub use package_2020_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-01")]
pub mod package_2020_01;
#[cfg(all(feature = "package-2020-01", not(feature = "no-default-tag")))]
pub use package_2020_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-09")]
pub mod package_2018_09;
#[cfg(all(feature = "package-2018-09", not(feature = "no-default-tag")))]
pub use package_2018_09::{models, Client, ClientBuilder};

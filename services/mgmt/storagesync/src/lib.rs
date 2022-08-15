#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2020-09-01")]
pub mod package_2020_09_01;
#[cfg(all(feature = "package-2020-09-01", not(feature = "no-default-tag")))]
pub use package_2020_09_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-03-01")]
pub mod package_2020_03_01;
#[cfg(all(feature = "package-2020-03-01", not(feature = "no-default-tag")))]
pub use package_2020_03_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-10-01")]
pub mod package_2019_10_01;
#[cfg(all(feature = "package-2019-10-01", not(feature = "no-default-tag")))]
pub use package_2019_10_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-06-01")]
pub mod package_2019_06_01;
#[cfg(all(feature = "package-2019-06-01", not(feature = "no-default-tag")))]
pub use package_2019_06_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-03-01")]
pub mod package_2019_03_01;
#[cfg(all(feature = "package-2019-03-01", not(feature = "no-default-tag")))]
pub use package_2019_03_01::{models, Client, ClientBuilder};

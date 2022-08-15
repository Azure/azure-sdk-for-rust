#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-05")]
pub mod package_2021_05;
#[cfg(all(feature = "package-2021-05", not(feature = "no-default-tag")))]
pub use package_2021_05::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-03")]
pub mod package_2021_03;
#[cfg(all(feature = "package-2021-03", not(feature = "no-default-tag")))]
pub use package_2021_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-01")]
pub mod package_2020_01;
#[cfg(all(feature = "package-2020-01", not(feature = "no-default-tag")))]
pub use package_2020_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2017-06")]
pub mod package_2017_06;
#[cfg(all(feature = "package-2017-06", not(feature = "no-default-tag")))]
pub use package_2017_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-2017-01")]
pub mod package_2017_01;
#[cfg(all(feature = "package-2017-01", not(feature = "no-default-tag")))]
pub use package_2017_01::{models, Client, ClientBuilder};

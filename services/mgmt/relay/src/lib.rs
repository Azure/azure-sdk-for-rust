#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-11-01")]
pub mod package_2021_11_01;
#[cfg(all(feature = "package-2021-11-01", not(feature = "no-default-tag")))]
pub use package_2021_11_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-01-preview")]
pub mod package_2018_01_preview;
#[cfg(all(feature = "package-2018-01-preview", not(feature = "no-default-tag")))]
pub use package_2018_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2017-04")]
pub mod package_2017_04;
#[cfg(all(feature = "package-2017-04", not(feature = "no-default-tag")))]
pub use package_2017_04::{models, Client, ClientBuilder};
#[cfg(feature = "package-2016-07")]
pub mod package_2016_07;
#[cfg(all(feature = "package-2016-07", not(feature = "no-default-tag")))]
pub use package_2016_07::{models, Client, ClientBuilder};

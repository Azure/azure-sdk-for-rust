#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2021-06")]
pub mod package_2021_06;
#[cfg(all(feature = "package-2021-06", not(feature = "no-default-tag")))]
pub use package_2021_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-04")]
pub mod package_2021_04;
#[cfg(all(feature = "package-2021-04", not(feature = "no-default-tag")))]
pub use package_2021_04::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-10")]
pub mod package_2020_10;
#[cfg(all(feature = "package-2020-10", not(feature = "no-default-tag")))]
pub use package_2020_10::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-06")]
pub mod package_2020_06;
#[cfg(all(feature = "package-2020-06", not(feature = "no-default-tag")))]
pub use package_2020_06::{models, Client, ClientBuilder};

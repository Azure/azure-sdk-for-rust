#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-05")]
pub mod package_2022_05;
#[cfg(all(feature = "package-2022-05", not(feature = "no-default-tag")))]
pub use package_2022_05::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-06")]
pub mod package_2021_06;
#[cfg(all(feature = "package-2021-06", not(feature = "no-default-tag")))]
pub use package_2021_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-11")]
pub mod package_2020_11;
#[cfg(all(feature = "package-2020-11", not(feature = "no-default-tag")))]
pub use package_2020_11::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-05")]
pub mod package_2020_05;
#[cfg(all(feature = "package-2020-05", not(feature = "no-default-tag")))]
pub use package_2020_05::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-04")]
pub mod package_2020_04;
#[cfg(all(feature = "package-2020-04", not(feature = "no-default-tag")))]
pub use package_2020_04::{models, Client, ClientBuilder};

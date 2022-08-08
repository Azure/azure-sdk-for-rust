#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2022-03")]
pub mod package_2022_03;
#[cfg(all(feature = "package-2022-03", not(feature = "no-default-tag")))]
pub use package_2022_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-12")]
pub mod package_2021_12;
#[cfg(all(feature = "package-2021-12", not(feature = "no-default-tag")))]
pub use package_2021_12::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-06-01")]
pub mod package_2021_06_01;
#[cfg(all(feature = "package-2021-06-01", not(feature = "no-default-tag")))]
pub use package_2021_06_01::{models, Client, ClientBuilder};

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2022-01-01")]
pub mod package_2022_01_01;
#[cfg(all(feature = "package-2022-01-01", not(feature = "no-default-tag")))]
pub use package_2022_01_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-06-01")]
pub mod package_2021_06_01;
#[cfg(all(feature = "package-2021-06-01", not(feature = "no-default-tag")))]
pub use package_2021_06_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-01-01")]
pub mod package_2021_01_01;
#[cfg(all(feature = "package-2021-01-01", not(feature = "no-default-tag")))]
pub use package_2021_01_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-10-01")]
pub mod package_2020_10_01;
#[cfg(all(feature = "package-2020-10-01", not(feature = "no-default-tag")))]
pub use package_2020_10_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-04-01")]
pub mod package_2020_04_01;
#[cfg(all(feature = "package-2020-04-01", not(feature = "no-default-tag")))]
pub use package_2020_04_01::{models, Client, ClientBuilder};

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2021-10-01")]
pub mod package_2021_10_01;
#[cfg(all(feature = "package-2021-10-01", not(feature = "no-default-tag")))]
pub use package_2021_10_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-05-01")]
pub mod package_2021_05_01;
#[cfg(all(feature = "package-2021-05-01", not(feature = "no-default-tag")))]
pub use package_2021_05_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-03-01")]
pub mod package_2021_03_01;
#[cfg(all(feature = "package-2021-03-01", not(feature = "no-default-tag")))]
pub use package_2021_03_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-02-01")]
pub mod package_2021_02_01;
#[cfg(all(feature = "package-2021-02-01", not(feature = "no-default-tag")))]
pub use package_2021_02_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-01-01")]
pub mod package_2021_01_01;
#[cfg(all(feature = "package-2021-01-01", not(feature = "no-default-tag")))]
pub use package_2021_01_01::{models, Client, ClientBuilder};

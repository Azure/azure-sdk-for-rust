#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2021-10")]
pub mod package_2021_10;
#[cfg(all(feature = "package-2021-10", not(feature = "no-default-tag")))]
pub use package_2021_10::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-09")]
pub mod package_2021_09;
#[cfg(all(feature = "package-2021-09", not(feature = "no-default-tag")))]
pub use package_2021_09::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-07")]
pub mod package_2021_07;
#[cfg(all(feature = "package-2021-07", not(feature = "no-default-tag")))]
pub use package_2021_07::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-03")]
pub mod package_2021_03;
#[cfg(all(feature = "package-2021-03", not(feature = "no-default-tag")))]
pub use package_2021_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-11")]
pub mod package_2020_11;
#[cfg(all(feature = "package-2020-11", not(feature = "no-default-tag")))]
pub use package_2020_11::{models, Client, ClientBuilder};

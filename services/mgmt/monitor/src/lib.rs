#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2021-04")]
pub mod package_2021_04;
#[cfg(all(feature = "package-2021-04", not(feature = "no-default-tag")))]
pub use package_2021_04::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-03")]
pub mod package_2020_03;
#[cfg(all(feature = "package-2020-03", not(feature = "no-default-tag")))]
pub use package_2020_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-11")]
pub mod package_2019_11;
#[cfg(all(feature = "package-2019-11", not(feature = "no-default-tag")))]
pub use package_2019_11::{models, Client, ClientBuilder};

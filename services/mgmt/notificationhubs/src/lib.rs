#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2017-04")]
pub mod package_2017_04;
#[cfg(all(feature = "package-2017-04", not(feature = "no-default-tag")))]
pub use package_2017_04::{models, Client, ClientBuilder};
#[cfg(feature = "package-2016-03")]
pub mod package_2016_03;
#[cfg(all(feature = "package-2016-03", not(feature = "no-default-tag")))]
pub use package_2016_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-2014-09")]
pub mod package_2014_09;
#[cfg(all(feature = "package-2014-09", not(feature = "no-default-tag")))]
pub use package_2014_09::{models, Client, ClientBuilder};

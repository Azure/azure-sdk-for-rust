#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2018-09")]
pub mod package_2018_09;
#[cfg(all(feature = "package-2018-09", not(feature = "no-default-tag")))]
pub use package_2018_09::{models, Client, ClientBuilder};

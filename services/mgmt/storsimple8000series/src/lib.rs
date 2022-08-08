#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2017-06")]
pub mod package_2017_06;
#[cfg(all(feature = "package-2017-06", not(feature = "no-default-tag")))]
pub use package_2017_06::{models, Client, ClientBuilder};

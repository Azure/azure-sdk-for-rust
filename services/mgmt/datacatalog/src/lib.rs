#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2016-03-30")]
pub mod package_2016_03_30;
#[cfg(all(feature = "package-2016-03-30", not(feature = "no-default-tag")))]
pub use package_2016_03_30::{models, Client, ClientBuilder};

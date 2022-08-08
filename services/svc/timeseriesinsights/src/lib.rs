#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2020-07-31")]
pub mod package_2020_07_31;
#[cfg(all(feature = "package-2020-07-31", not(feature = "no-default-tag")))]
pub use package_2020_07_31::{models, Client, ClientBuilder};

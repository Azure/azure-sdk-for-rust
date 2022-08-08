#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2018-06-01-preview")]
pub mod package_2018_06_01_preview;
#[cfg(all(feature = "package-2018-06-01-preview", not(feature = "no-default-tag")))]
pub use package_2018_06_01_preview::{models, Client, ClientBuilder};

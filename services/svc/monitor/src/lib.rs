#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2018-09-preview")]
pub mod package_2018_09_preview;
#[cfg(all(feature = "package-2018-09-preview", not(feature = "no-default-tag")))]
pub use package_2018_09_preview::{models, Client, ClientBuilder};

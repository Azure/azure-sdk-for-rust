#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2016-11")]
pub mod package_2016_11;
#[cfg(all(feature = "package-2016-11", not(feature = "no-default-tag")))]
pub use package_2016_11::{models, Client, ClientBuilder};
#[cfg(feature = "package-2015-10-preview")]
pub mod package_2015_10_preview;
#[cfg(all(feature = "package-2015-10-preview", not(feature = "no-default-tag")))]
pub use package_2015_10_preview::{models, Client, ClientBuilder};

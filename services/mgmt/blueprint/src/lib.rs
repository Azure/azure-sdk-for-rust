#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2018-11-preview")]
pub mod package_2018_11_preview;
#[cfg(all(feature = "package-2018-11-preview", not(feature = "no-default-tag")))]
pub use package_2018_11_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2017-11-preview")]
pub mod package_2017_11_preview;
#[cfg(all(feature = "package-2017-11-preview", not(feature = "no-default-tag")))]
pub use package_2017_11_preview::{models, Client, ClientBuilder};

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2020-01-13-preview")]
pub mod package_2020_01_13_preview;
#[cfg(all(feature = "package-2020-01-13-preview", not(feature = "no-default-tag")))]
pub use package_2020_01_13_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-08-31-preview")]
pub mod package_2018_08_31_preview;
#[cfg(all(feature = "package-2018-08-31-preview", not(feature = "no-default-tag")))]
pub use package_2018_08_31_preview::{models, Client, ClientBuilder};

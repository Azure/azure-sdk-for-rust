#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2022-04-01-preview")]
pub mod package_2022_04_01_preview;
#[cfg(all(feature = "package-2022-04-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_04_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-04-01")]
pub mod package_2018_04_01;
#[cfg(all(feature = "package-2018-04-01", not(feature = "no-default-tag")))]
pub use package_2018_04_01::{models, Client, ClientBuilder};

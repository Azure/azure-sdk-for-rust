#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2022-03-01")]
pub mod package_2022_03_01;
#[cfg(all(feature = "package-2022-03-01", not(feature = "no-default-tag")))]
pub use package_2022_03_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-04-04-preview")]
pub mod package_2021_04_04_preview;
#[cfg(all(feature = "package-2021-04-04-preview", not(feature = "no-default-tag")))]
pub use package_2021_04_04_preview::{models, Client, ClientBuilder};

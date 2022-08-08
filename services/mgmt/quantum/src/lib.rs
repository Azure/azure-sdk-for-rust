#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2022-01-10-preview")]
pub mod package_2022_01_10_preview;
#[cfg(all(feature = "package-2022-01-10-preview", not(feature = "no-default-tag")))]
pub use package_2022_01_10_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-11-04-preview")]
pub mod package_2019_11_04_preview;
#[cfg(all(feature = "package-2019-11-04-preview", not(feature = "no-default-tag")))]
pub use package_2019_11_04_preview::{models, Client, ClientBuilder};

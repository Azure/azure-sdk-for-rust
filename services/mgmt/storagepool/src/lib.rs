#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2021-08-01")]
pub mod package_2021_08_01;
#[cfg(all(feature = "package-2021-08-01", not(feature = "no-default-tag")))]
pub use package_2021_08_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-04-01-preview")]
pub mod package_2021_04_01_preview;
#[cfg(all(feature = "package-2021-04-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_04_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-03-15-preview")]
pub mod package_2020_03_15_preview;
#[cfg(all(feature = "package-2020-03-15-preview", not(feature = "no-default-tag")))]
pub use package_2020_03_15_preview::{models, Client, ClientBuilder};

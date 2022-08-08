#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2022-05-01")]
pub mod package_2022_05_01;
#[cfg(all(feature = "package-2022-05-01", not(feature = "no-default-tag")))]
pub use package_2022_05_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-01-01-preview")]
pub mod package_2022_01_01_preview;
#[cfg(all(feature = "package-2022-01-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_01_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-11-01-preview")]
pub mod package_2021_11_01_preview;
#[cfg(all(feature = "package-2021-11-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_11_01_preview::{models, Client, ClientBuilder};

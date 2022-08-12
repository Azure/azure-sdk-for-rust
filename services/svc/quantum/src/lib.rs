#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-11-01-preview")]
pub mod package_2021_11_01_preview;
#[cfg(all(feature = "package-2021-11-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_11_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-05-06-preview")]
pub mod package_2021_05_06_preview;
#[cfg(all(feature = "package-2021-05-06-preview", not(feature = "no-default-tag")))]
pub use package_2021_05_06_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-11-04-preview")]
pub mod package_2019_11_04_preview;
#[cfg(all(feature = "package-2019-11-04-preview", not(feature = "no-default-tag")))]
pub use package_2019_11_04_preview::{models, Client, ClientBuilder};

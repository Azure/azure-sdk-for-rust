#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-09-01-preview")]
pub mod package_2021_09_01_preview;
#[cfg(all(feature = "package-2021-09-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_09_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-06-01-preview")]
pub mod package_2021_06_01_preview;
#[cfg(all(feature = "package-2021-06-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_06_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-05-01-preview")]
pub mod package_2021_05_01_preview;
#[cfg(all(feature = "package-2021-05-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_05_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-11-20")]
pub mod package_2020_11_20;
#[cfg(all(feature = "package-2020-11-20", not(feature = "no-default-tag")))]
pub use package_2020_11_20::{models, Client, ClientBuilder};

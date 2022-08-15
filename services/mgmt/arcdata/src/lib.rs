#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2022-03")]
pub mod package_preview_2022_03;
#[cfg(all(feature = "package-preview-2022-03", not(feature = "no-default-tag")))]
pub use package_preview_2022_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2021-07-01")]
pub mod package_preview_2021_07_01;
#[cfg(all(feature = "package-preview-2021-07-01", not(feature = "no-default-tag")))]
pub use package_preview_2021_07_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2021-06-01")]
pub mod package_preview_2021_06_01;
#[cfg(all(feature = "package-preview-2021-06-01", not(feature = "no-default-tag")))]
pub use package_preview_2021_06_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-11-01")]
pub mod package_2021_11_01;
#[cfg(all(feature = "package-2021-11-01", not(feature = "no-default-tag")))]
pub use package_2021_11_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-08-01")]
pub mod package_2021_08_01;
#[cfg(all(feature = "package-2021-08-01", not(feature = "no-default-tag")))]
pub use package_2021_08_01::{models, Client, ClientBuilder};

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2022-01")]
pub mod package_preview_2022_01;
#[cfg(all(feature = "package-preview-2022-01", not(feature = "no-default-tag")))]
pub use package_preview_2022_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2021-09-30")]
pub mod package_preview_2021_09_30;
#[cfg(all(feature = "package-preview-2021-09-30", not(feature = "no-default-tag")))]
pub use package_preview_2021_09_30::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-11-30")]
pub mod package_2018_11_30;
#[cfg(all(feature = "package-2018-11-30", not(feature = "no-default-tag")))]
pub use package_2018_11_30::{models, Client, ClientBuilder};
#[cfg(feature = "package-2015-08-31-preview")]
pub mod package_2015_08_31_preview;
#[cfg(all(feature = "package-2015-08-31-preview", not(feature = "no-default-tag")))]
pub use package_2015_08_31_preview::{models, Client, ClientBuilder};

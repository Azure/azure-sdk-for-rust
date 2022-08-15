#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2021-08")]
pub mod package_preview_2021_08;
#[cfg(all(feature = "package-preview-2021-08", not(feature = "no-default-tag")))]
pub use package_preview_2021_08::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2021-01")]
pub mod package_preview_2021_01;
#[cfg(all(feature = "package-preview-2021-01", not(feature = "no-default-tag")))]
pub use package_preview_2021_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2019-05")]
pub mod package_preview_2019_05;
#[cfg(all(feature = "package-preview-2019-05", not(feature = "no-default-tag")))]
pub use package_preview_2019_05::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-08")]
pub mod package_2021_08;
#[cfg(all(feature = "package-2021-08", not(feature = "no-default-tag")))]
pub use package_2021_08::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-06-preview")]
pub mod package_2019_06_preview;
#[cfg(all(feature = "package-2019-06-preview", not(feature = "no-default-tag")))]
pub use package_2019_06_preview::{models, Client, ClientBuilder};

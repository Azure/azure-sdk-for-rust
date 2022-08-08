#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-preview-2021-12")]
pub mod package_preview_2021_12;
#[cfg(all(feature = "package-preview-2021-12", not(feature = "no-default-tag")))]
pub use package_preview_2021_12::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2021-04")]
pub mod package_preview_2021_04;
#[cfg(all(feature = "package-preview-2021-04", not(feature = "no-default-tag")))]
pub use package_preview_2021_04::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2021-01")]
pub mod package_preview_2021_01;
#[cfg(all(feature = "package-preview-2021-01", not(feature = "no-default-tag")))]
pub use package_preview_2021_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2020-06")]
pub mod package_preview_2020_06;
#[cfg(all(feature = "package-preview-2020-06", not(feature = "no-default-tag")))]
pub use package_preview_2020_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2019-12")]
pub mod package_preview_2019_12;
#[cfg(all(feature = "package-preview-2019-12", not(feature = "no-default-tag")))]
pub use package_preview_2019_12::{models, Client, ClientBuilder};

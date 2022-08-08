#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-preview-2021-06")]
pub mod package_preview_2021_06;
#[cfg(all(feature = "package-preview-2021-06", not(feature = "no-default-tag")))]
pub use package_preview_2021_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2021-03")]
pub mod package_preview_2021_03;
#[cfg(all(feature = "package-preview-2021-03", not(feature = "no-default-tag")))]
pub use package_preview_2021_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2020-09")]
pub mod package_preview_2020_09;
#[cfg(all(feature = "package-preview-2020-09", not(feature = "no-default-tag")))]
pub use package_preview_2020_09::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2020-04")]
pub mod package_preview_2020_04;
#[cfg(all(feature = "package-preview-2020-04", not(feature = "no-default-tag")))]
pub use package_preview_2020_04::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-03")]
pub mod package_2021_03;
#[cfg(all(feature = "package-2021-03", not(feature = "no-default-tag")))]
pub use package_2021_03::{models, Client, ClientBuilder};

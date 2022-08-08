#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-preview-2022-07")]
pub mod package_preview_2022_07;
#[cfg(all(feature = "package-preview-2022-07", not(feature = "no-default-tag")))]
pub use package_preview_2022_07::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2022-06")]
pub mod package_preview_2022_06;
#[cfg(all(feature = "package-preview-2022-06", not(feature = "no-default-tag")))]
pub use package_preview_2022_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2022-05")]
pub mod package_preview_2022_05;
#[cfg(all(feature = "package-preview-2022-05", not(feature = "no-default-tag")))]
pub use package_preview_2022_05::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2022-04")]
pub mod package_preview_2022_04;
#[cfg(all(feature = "package-preview-2022-04", not(feature = "no-default-tag")))]
pub use package_preview_2022_04::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2022-01")]
pub mod package_preview_2022_01;
#[cfg(all(feature = "package-preview-2022-01", not(feature = "no-default-tag")))]
pub use package_preview_2022_01::{models, Client, ClientBuilder};

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2022-05")]
pub mod package_preview_2022_05;
#[cfg(all(feature = "package-preview-2022-05", not(feature = "no-default-tag")))]
pub use package_preview_2022_05::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2021-12")]
pub mod package_preview_2021_12;
#[cfg(all(feature = "package-preview-2021-12", not(feature = "no-default-tag")))]
pub use package_preview_2021_12::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2021-06")]
pub mod package_preview_2021_06;
#[cfg(all(feature = "package-preview-2021-06", not(feature = "no-default-tag")))]
pub use package_preview_2021_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2021-05")]
pub mod package_preview_2021_05;
#[cfg(all(feature = "package-preview-2021-05", not(feature = "no-default-tag")))]
pub use package_preview_2021_05::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2021-04")]
pub mod package_preview_2021_04;
#[cfg(all(feature = "package-preview-2021-04", not(feature = "no-default-tag")))]
pub use package_preview_2021_04::{models, Client, ClientBuilder};

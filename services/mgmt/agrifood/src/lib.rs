#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2023-06")]
pub mod package_preview_2023_06;
#[cfg(all(feature = "package-preview-2023-06", not(feature = "no-default-tag")))]
pub use package_preview_2023_06::*;
#[cfg(feature = "package-preview-2021-09")]
pub mod package_preview_2021_09;
#[cfg(all(feature = "package-preview-2021-09", not(feature = "no-default-tag")))]
pub use package_preview_2021_09::*;
#[cfg(feature = "package-2020-05-12-preview")]
pub mod package_2020_05_12_preview;
#[cfg(all(feature = "package-2020-05-12-preview", not(feature = "no-default-tag")))]
pub use package_2020_05_12_preview::*;

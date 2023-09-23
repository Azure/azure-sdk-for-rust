#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2022-12")]
pub mod package_preview_2022_12;
#[cfg(all(feature = "package-preview-2022-12", not(feature = "no-default-tag")))]
pub use package_preview_2022_12::*;
#[cfg(feature = "package-2023-01")]
pub mod package_2023_01;
#[cfg(all(feature = "package-2023-01", not(feature = "no-default-tag")))]
pub use package_2023_01::*;
#[cfg(feature = "package-2021-11-20-preview")]
pub mod package_2021_11_20_preview;
#[cfg(all(feature = "package-2021-11-20-preview", not(feature = "no-default-tag")))]
pub use package_2021_11_20_preview::*;

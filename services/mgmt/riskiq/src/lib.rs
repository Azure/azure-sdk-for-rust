#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2023-04")]
pub mod package_preview_2023_04;
#[cfg(all(feature = "package-preview-2023-04", not(feature = "no-default-tag")))]
pub use package_preview_2023_04::*;
#[cfg(feature = "package-preview-2022-04-01")]
pub mod package_preview_2022_04_01;
#[cfg(all(feature = "package-preview-2022-04-01", not(feature = "no-default-tag")))]
pub use package_preview_2022_04_01::*;

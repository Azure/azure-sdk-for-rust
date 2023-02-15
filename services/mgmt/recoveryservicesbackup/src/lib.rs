#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2022-09")]
pub mod package_preview_2022_09;
#[cfg(all(feature = "package-preview-2022-09", not(feature = "no-default-tag")))]
pub use package_preview_2022_09::*;
#[cfg(feature = "package-2023-01")]
pub mod package_2023_01;
#[cfg(all(feature = "package-2023-01", not(feature = "no-default-tag")))]
pub use package_2023_01::*;
#[cfg(feature = "package-2022-10")]
pub mod package_2022_10;
#[cfg(all(feature = "package-2022-10", not(feature = "no-default-tag")))]
pub use package_2022_10::*;
#[cfg(feature = "package-2022-09-preview")]
pub mod package_2022_09_preview;
#[cfg(all(feature = "package-2022-09-preview", not(feature = "no-default-tag")))]
pub use package_2022_09_preview::*;
#[cfg(feature = "package-2022-06-01-preview")]
pub mod package_2022_06_01_preview;
#[cfg(all(feature = "package-2022-06-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_06_01_preview::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2023-09")]
pub mod package_preview_2023_09;
#[cfg(all(feature = "package-preview-2023-09", not(feature = "no-default-tag")))]
pub use package_preview_2023_09::*;
#[cfg(feature = "package-2023-04-15-preview")]
pub mod package_2023_04_15_preview;
#[cfg(all(feature = "package-2023-04-15-preview", not(feature = "no-default-tag")))]
pub use package_2023_04_15_preview::*;
#[cfg(feature = "package-2023-04-01-preview")]
pub mod package_2023_04_01_preview;
#[cfg(all(feature = "package-2023-04-01-preview", not(feature = "no-default-tag")))]
pub use package_2023_04_01_preview::*;
#[cfg(feature = "package-2022-10-01-preview")]
pub mod package_2022_10_01_preview;
#[cfg(all(feature = "package-2022-10-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_10_01_preview::*;
#[cfg(feature = "package-2022-07-01-preview")]
pub mod package_2022_07_01_preview;
#[cfg(all(feature = "package-2022-07-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_07_01_preview::*;

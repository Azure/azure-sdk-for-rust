#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2023-06")]
pub mod package_preview_2023_06;
#[cfg(all(feature = "package-preview-2023-06", not(feature = "no-default-tag")))]
pub use package_preview_2023_06::*;
#[cfg(feature = "package-preview-2023-01")]
pub mod package_preview_2023_01;
#[cfg(all(feature = "package-preview-2023-01", not(feature = "no-default-tag")))]
pub use package_preview_2023_01::*;
#[cfg(feature = "package-2022-09-08-preview")]
pub mod package_2022_09_08_preview;
#[cfg(all(feature = "package-2022-09-08-preview", not(feature = "no-default-tag")))]
pub use package_2022_09_08_preview::*;
#[cfg(feature = "package-2022-05-13")]
pub mod package_2022_05_13;
#[cfg(all(feature = "package-2022-05-13", not(feature = "no-default-tag")))]
pub use package_2022_05_13::*;
#[cfg(feature = "package-2021-05-13-preview")]
pub mod package_2021_05_13_preview;
#[cfg(all(feature = "package-2021-05-13-preview", not(feature = "no-default-tag")))]
pub use package_2021_05_13_preview::*;

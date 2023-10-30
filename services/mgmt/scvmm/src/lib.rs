#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2023-04")]
pub mod package_preview_2023_04;
#[cfg(all(feature = "package-preview-2023-04", not(feature = "no-default-tag")))]
pub use package_preview_2023_04::*;
#[cfg(feature = "package-2023-10")]
pub mod package_2023_10;
#[cfg(all(feature = "package-2023-10", not(feature = "no-default-tag")))]
pub use package_2023_10::*;
#[cfg(feature = "package-2022-05-21-preview")]
pub mod package_2022_05_21_preview;
#[cfg(all(feature = "package-2022-05-21-preview", not(feature = "no-default-tag")))]
pub use package_2022_05_21_preview::*;
#[cfg(feature = "package-2020-06-05-preview")]
pub mod package_2020_06_05_preview;
#[cfg(all(feature = "package-2020-06-05-preview", not(feature = "no-default-tag")))]
pub use package_2020_06_05_preview::*;

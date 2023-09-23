#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2023-03")]
pub mod package_preview_2023_03;
#[cfg(all(feature = "package-preview-2023-03", not(feature = "no-default-tag")))]
pub use package_preview_2023_03::*;
#[cfg(feature = "package-2023-10")]
pub mod package_2023_10;
#[cfg(all(feature = "package-2023-10", not(feature = "no-default-tag")))]
pub use package_2023_10::*;
#[cfg(feature = "package-2022-07-15-preview")]
pub mod package_2022_07_15_preview;
#[cfg(all(feature = "package-2022-07-15-preview", not(feature = "no-default-tag")))]
pub use package_2022_07_15_preview::*;
#[cfg(feature = "package-2022-01-10-preview")]
pub mod package_2022_01_10_preview;
#[cfg(all(feature = "package-2022-01-10-preview", not(feature = "no-default-tag")))]
pub use package_2022_01_10_preview::*;
#[cfg(feature = "package-2020-10-01-preview")]
pub mod package_2020_10_01_preview;
#[cfg(all(feature = "package-2020-10-01-preview", not(feature = "no-default-tag")))]
pub use package_2020_10_01_preview::*;

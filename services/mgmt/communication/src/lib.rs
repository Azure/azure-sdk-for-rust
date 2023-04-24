#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2023-03")]
pub mod package_preview_2023_03;
#[cfg(all(feature = "package-preview-2023-03", not(feature = "no-default-tag")))]
pub use package_preview_2023_03::*;
#[cfg(feature = "package-preview-2022-07")]
pub mod package_preview_2022_07;
#[cfg(all(feature = "package-preview-2022-07", not(feature = "no-default-tag")))]
pub use package_preview_2022_07::*;
#[cfg(feature = "package-2023-03")]
pub mod package_2023_03;
#[cfg(all(feature = "package-2023-03", not(feature = "no-default-tag")))]
pub use package_2023_03::*;
#[cfg(feature = "package-2021-10-01-preview")]
pub mod package_2021_10_01_preview;
#[cfg(all(feature = "package-2021-10-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_10_01_preview::*;
#[cfg(feature = "package-2020-08-20-preview")]
pub mod package_2020_08_20_preview;
#[cfg(all(feature = "package-2020-08-20-preview", not(feature = "no-default-tag")))]
pub use package_2020_08_20_preview::*;

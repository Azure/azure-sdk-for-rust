#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2023-02")]
pub mod package_preview_2023_02;
#[cfg(all(feature = "package-preview-2023-02", not(feature = "no-default-tag")))]
pub use package_preview_2023_02::*;
#[cfg(feature = "package-preview-2022-08")]
pub mod package_preview_2022_08;
#[cfg(all(feature = "package-preview-2022-08", not(feature = "no-default-tag")))]
pub use package_preview_2022_08::*;
#[cfg(feature = "package-preview-2022-03")]
pub mod package_preview_2022_03;
#[cfg(all(feature = "package-preview-2022-03", not(feature = "no-default-tag")))]
pub use package_preview_2022_03::*;
#[cfg(feature = "package-2023-10-01-preview")]
pub mod package_2023_10_01_preview;
#[cfg(all(feature = "package-2023-10-01-preview", not(feature = "no-default-tag")))]
pub use package_2023_10_01_preview::*;
#[cfg(feature = "package-2023-09")]
pub mod package_2023_09;
#[cfg(all(feature = "package-2023-09", not(feature = "no-default-tag")))]
pub use package_2023_09::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2021-09")]
pub mod package_preview_2021_09;
#[cfg(all(feature = "package-preview-2021-09", not(feature = "no-default-tag")))]
pub use package_preview_2021_09::*;
#[cfg(feature = "package-2023-08-22")]
pub mod package_2023_08_22;
#[cfg(all(feature = "package-2023-08-22", not(feature = "no-default-tag")))]
pub use package_2023_08_22::*;
#[cfg(feature = "package-2021-12-01")]
pub mod package_2021_12_01;
#[cfg(all(feature = "package-2021-12-01", not(feature = "no-default-tag")))]
pub use package_2021_12_01::*;
#[cfg(feature = "package-2021-03-01-preview")]
pub mod package_2021_03_01_preview;
#[cfg(all(feature = "package-2021-03-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_03_01_preview::*;
#[cfg(feature = "package-2020-03-01-preview")]
pub mod package_2020_03_01_preview;
#[cfg(all(feature = "package-2020-03-01-preview", not(feature = "no-default-tag")))]
pub use package_2020_03_01_preview::*;

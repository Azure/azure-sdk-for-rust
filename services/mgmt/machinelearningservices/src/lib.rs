#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2022-06")]
pub mod package_preview_2022_06;
#[cfg(all(feature = "package-preview-2022-06", not(feature = "no-default-tag")))]
pub use package_preview_2022_06::*;
#[cfg(feature = "package-preview-2020-05")]
pub mod package_preview_2020_05;
#[cfg(all(feature = "package-preview-2020-05", not(feature = "no-default-tag")))]
pub use package_preview_2020_05::*;
#[cfg(feature = "package-2022-05-01")]
pub mod package_2022_05_01;
#[cfg(all(feature = "package-2022-05-01", not(feature = "no-default-tag")))]
pub use package_2022_05_01::*;
#[cfg(feature = "package-2022-02-01-preview")]
pub mod package_2022_02_01_preview;
#[cfg(all(feature = "package-2022-02-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_02_01_preview::*;
#[cfg(feature = "package-2022-01-01-preview")]
pub mod package_2022_01_01_preview;
#[cfg(all(feature = "package-2022-01-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_01_01_preview::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2023-10")]
pub mod package_preview_2023_10;
#[cfg(all(feature = "package-preview-2023-10", not(feature = "no-default-tag")))]
pub use package_preview_2023_10::*;
#[cfg(feature = "package-preview-2023-07")]
pub mod package_preview_2023_07;
#[cfg(all(feature = "package-preview-2023-07", not(feature = "no-default-tag")))]
pub use package_preview_2023_07::*;
#[cfg(feature = "package-preview-2022-10")]
pub mod package_preview_2022_10;
#[cfg(all(feature = "package-preview-2022-10", not(feature = "no-default-tag")))]
pub use package_preview_2022_10::*;
#[cfg(feature = "package-preview-2022-05-01")]
pub mod package_preview_2022_05_01;
#[cfg(all(feature = "package-preview-2022-05-01", not(feature = "no-default-tag")))]
pub use package_preview_2022_05_01::*;
#[cfg(feature = "package-2022-10")]
pub mod package_2022_10;
#[cfg(all(feature = "package-2022-10", not(feature = "no-default-tag")))]
pub use package_2022_10::*;

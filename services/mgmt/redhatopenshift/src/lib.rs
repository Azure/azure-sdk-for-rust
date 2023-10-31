#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2023-07")]
pub mod package_preview_2023_07;
#[cfg(all(feature = "package-preview-2023-07", not(feature = "no-default-tag")))]
pub use package_preview_2023_07::*;
#[cfg(feature = "package-2023-09-04")]
pub mod package_2023_09_04;
#[cfg(all(feature = "package-2023-09-04", not(feature = "no-default-tag")))]
pub use package_2023_09_04::*;
#[cfg(feature = "package-2023-04-01")]
pub mod package_2023_04_01;
#[cfg(all(feature = "package-2023-04-01", not(feature = "no-default-tag")))]
pub use package_2023_04_01::*;
#[cfg(feature = "package-2022-09-04")]
pub mod package_2022_09_04;
#[cfg(all(feature = "package-2022-09-04", not(feature = "no-default-tag")))]
pub use package_2022_09_04::*;
#[cfg(feature = "package-2022-04-01")]
pub mod package_2022_04_01;
#[cfg(all(feature = "package-2022-04-01", not(feature = "no-default-tag")))]
pub use package_2022_04_01::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2019-05")]
pub mod package_preview_2019_05;
#[cfg(all(feature = "package-preview-2019-05", not(feature = "no-default-tag")))]
pub use package_preview_2019_05::*;
#[cfg(feature = "package-preview-2019-04")]
pub mod package_preview_2019_04;
#[cfg(all(feature = "package-preview-2019-04", not(feature = "no-default-tag")))]
pub use package_preview_2019_04::*;
#[cfg(feature = "package-preview-2018-11")]
pub mod package_preview_2018_11;
#[cfg(all(feature = "package-preview-2018-11", not(feature = "no-default-tag")))]
pub use package_preview_2018_11::*;
#[cfg(feature = "package-2023-03")]
pub mod package_2023_03;
#[cfg(all(feature = "package-2023-03", not(feature = "no-default-tag")))]
pub use package_2023_03::*;
#[cfg(feature = "package-2022-09")]
pub mod package_2022_09;
#[cfg(all(feature = "package-2022-09", not(feature = "no-default-tag")))]
pub use package_2022_09::*;

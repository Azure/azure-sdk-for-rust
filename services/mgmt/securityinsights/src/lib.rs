#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2023-02")]
pub mod package_preview_2023_02;
#[cfg(all(feature = "package-preview-2023-02", not(feature = "no-default-tag")))]
pub use package_preview_2023_02::*;
#[cfg(feature = "package-preview-2022-12")]
pub mod package_preview_2022_12;
#[cfg(all(feature = "package-preview-2022-12", not(feature = "no-default-tag")))]
pub use package_preview_2022_12::*;
#[cfg(feature = "package-preview-2022-11")]
pub mod package_preview_2022_11;
#[cfg(all(feature = "package-preview-2022-11", not(feature = "no-default-tag")))]
pub use package_preview_2022_11::*;
#[cfg(feature = "package-preview-2022-10")]
pub mod package_preview_2022_10;
#[cfg(all(feature = "package-preview-2022-10", not(feature = "no-default-tag")))]
pub use package_preview_2022_10::*;
#[cfg(feature = "package-preview-2022-09")]
pub mod package_preview_2022_09;
#[cfg(all(feature = "package-preview-2022-09", not(feature = "no-default-tag")))]
pub use package_preview_2022_09::*;

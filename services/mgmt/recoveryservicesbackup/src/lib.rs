#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2022-09")]
pub mod package_preview_2022_09;
#[cfg(all(feature = "package-preview-2022-09", not(feature = "no-default-tag")))]
pub use package_preview_2022_09::*;
#[cfg(feature = "package-passivestamp-2023-01-15")]
pub mod package_passivestamp_2023_01_15;
#[cfg(all(feature = "package-passivestamp-2023-01-15", not(feature = "no-default-tag")))]
pub use package_passivestamp_2023_01_15::*;
#[cfg(feature = "package-2023-02")]
pub mod package_2023_02;
#[cfg(all(feature = "package-2023-02", not(feature = "no-default-tag")))]
pub use package_2023_02::*;
#[cfg(feature = "package-2023-01")]
pub mod package_2023_01;
#[cfg(all(feature = "package-2023-01", not(feature = "no-default-tag")))]
pub use package_2023_01::*;
#[cfg(feature = "package-2022-10")]
pub mod package_2022_10;
#[cfg(all(feature = "package-2022-10", not(feature = "no-default-tag")))]
pub use package_2022_10::*;

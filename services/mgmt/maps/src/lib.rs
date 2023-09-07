#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2021-12")]
pub mod package_preview_2021_12;
#[cfg(all(feature = "package-preview-2021-12", not(feature = "no-default-tag")))]
pub use package_preview_2021_12::*;
#[cfg(feature = "package-preview-2021-07")]
pub mod package_preview_2021_07;
#[cfg(all(feature = "package-preview-2021-07", not(feature = "no-default-tag")))]
pub use package_preview_2021_07::*;
#[cfg(feature = "package-preview-2020-02")]
pub mod package_preview_2020_02;
#[cfg(all(feature = "package-preview-2020-02", not(feature = "no-default-tag")))]
pub use package_preview_2020_02::*;
#[cfg(feature = "package-2023-06")]
pub mod package_2023_06;
#[cfg(all(feature = "package-2023-06", not(feature = "no-default-tag")))]
pub use package_2023_06::*;
#[cfg(feature = "package-2021-02")]
pub mod package_2021_02;
#[cfg(all(feature = "package-2021-02", not(feature = "no-default-tag")))]
pub use package_2021_02::*;

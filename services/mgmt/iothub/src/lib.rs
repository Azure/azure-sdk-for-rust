#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2022-11")]
pub mod package_preview_2022_11;
#[cfg(all(feature = "package-preview-2022-11", not(feature = "no-default-tag")))]
pub use package_preview_2022_11::*;
#[cfg(feature = "package-preview-2022-04-30")]
pub mod package_preview_2022_04_30;
#[cfg(all(feature = "package-preview-2022-04-30", not(feature = "no-default-tag")))]
pub use package_preview_2022_04_30::*;
#[cfg(feature = "package-preview-2021-07-02")]
pub mod package_preview_2021_07_02;
#[cfg(all(feature = "package-preview-2021-07-02", not(feature = "no-default-tag")))]
pub use package_preview_2021_07_02::*;
#[cfg(feature = "package-preview-2021-03")]
pub mod package_preview_2021_03;
#[cfg(all(feature = "package-preview-2021-03", not(feature = "no-default-tag")))]
pub use package_preview_2021_03::*;
#[cfg(feature = "package-preview-2021-02")]
pub mod package_preview_2021_02;
#[cfg(all(feature = "package-preview-2021-02", not(feature = "no-default-tag")))]
pub use package_preview_2021_02::*;

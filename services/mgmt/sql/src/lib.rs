#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2022-08")]
pub mod package_preview_2022_08;
#[cfg(all(feature = "package-preview-2022-08", not(feature = "no-default-tag")))]
pub use package_preview_2022_08::*;
#[cfg(feature = "package-preview-2022-05")]
pub mod package_preview_2022_05;
#[cfg(all(feature = "package-preview-2022-05", not(feature = "no-default-tag")))]
pub use package_preview_2022_05::*;
#[cfg(feature = "package-preview-2021-11")]
pub mod package_preview_2021_11;
#[cfg(all(feature = "package-preview-2021-11", not(feature = "no-default-tag")))]
pub use package_preview_2021_11::*;
#[cfg(feature = "package-composite-v5")]
pub mod package_composite_v5;
#[cfg(all(feature = "package-composite-v5", not(feature = "no-default-tag")))]
pub use package_composite_v5::*;
#[cfg(feature = "package-composite-v4")]
pub mod package_composite_v4;
#[cfg(all(feature = "package-composite-v4", not(feature = "no-default-tag")))]
pub use package_composite_v4::*;

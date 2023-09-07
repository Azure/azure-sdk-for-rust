#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-7_5-preview_1")]
pub mod package_preview_7_5_preview_1;
#[cfg(all(feature = "package-preview-7_5-preview_1", not(feature = "no-default-tag")))]
pub use package_preview_7_5_preview_1::*;
#[cfg(feature = "package-preview-7_4-preview_1")]
pub mod package_preview_7_4_preview_1;
#[cfg(all(feature = "package-preview-7_4-preview_1", not(feature = "no-default-tag")))]
pub use package_preview_7_4_preview_1::*;
#[cfg(feature = "package-preview-7_3-preview")]
pub mod package_preview_7_3_preview;
#[cfg(all(feature = "package-preview-7_3-preview", not(feature = "no-default-tag")))]
pub use package_preview_7_3_preview::*;
#[cfg(feature = "package-7_4")]
pub mod package_7_4;
#[cfg(all(feature = "package-7_4", not(feature = "no-default-tag")))]
pub use package_7_4::*;
#[cfg(feature = "package-7_3")]
pub mod package_7_3;
#[cfg(all(feature = "package-7_3", not(feature = "no-default-tag")))]
pub use package_7_3::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2022-10")]
pub mod package_preview_2022_10;
#[cfg(all(feature = "package-preview-2022-10", not(feature = "without_tag_import")))]
pub use package_preview_2022_10::*;
#[cfg(feature = "package-preview-2022-04")]
pub mod package_preview_2022_04;
#[cfg(all(feature = "package-preview-2022-04", not(feature = "without_tag_import")))]
pub use package_preview_2022_04::*;
#[cfg(feature = "package-2023-08")]
pub mod package_2023_08;
#[cfg(all(feature = "package-2023-08", not(feature = "without_tag_import")))]
pub use package_2023_08::*;

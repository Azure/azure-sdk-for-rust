#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2022-11")]
pub mod package_preview_2022_11;
#[cfg(all(feature = "package-preview-2022-11", not(feature = "without_tag_import")))]
pub use package_preview_2022_11::*;
#[cfg(feature = "package-2023-04")]
pub mod package_2023_04;
#[cfg(all(feature = "package-2023-04", not(feature = "without_tag_import")))]
pub use package_2023_04::*;
#[cfg(feature = "package-2021-12-01-preview")]
pub mod package_2021_12_01_preview;
#[cfg(all(feature = "package-2021-12-01-preview", not(feature = "without_tag_import")))]
pub use package_2021_12_01_preview::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2023-10-01-preview")]
pub mod package_2023_10_01_preview;
#[cfg(feature = "package-preview-2022-03")]
pub mod package_preview_2022_03;
#[cfg(feature = "package-preview-2022-08")]
pub mod package_preview_2022_08;
#[cfg(feature = "package-preview-2023-02")]
pub mod package_preview_2023_02;
#[cfg(feature = "package-preview-2024-03")]
pub mod package_preview_2024_03;
#[cfg(all(feature = "default_tag", feature = "package-preview-2024-03"))]
pub use package_preview_2024_03::*;

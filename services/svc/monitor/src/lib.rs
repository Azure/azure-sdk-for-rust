#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2018-09-preview")]
pub mod package_2018_09_preview;
#[cfg(feature = "package-2023-10")]
pub mod package_2023_10;
#[cfg(feature = "package-2024-02")]
pub mod package_2024_02;
#[cfg(feature = "package-preview-2023-03")]
pub mod package_preview_2023_03;
#[cfg(feature = "package-preview-2023-05")]
pub mod package_preview_2023_05;
#[cfg(all(feature = "default_tag", feature = "package-2024-02"))]
pub use package_2024_02::*;

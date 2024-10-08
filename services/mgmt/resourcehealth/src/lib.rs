#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2024-02")]
pub mod package_2024_02;
#[cfg(feature = "package-preview-2022-05-01")]
pub mod package_preview_2022_05_01;
#[cfg(feature = "package-preview-2022-10")]
pub mod package_preview_2022_10;
#[cfg(feature = "package-preview-2023-07")]
pub mod package_preview_2023_07;
#[cfg(feature = "package-preview-2023-10")]
pub mod package_preview_2023_10;
#[cfg(all(feature = "default_tag", feature = "package-2024-02"))]
pub use package_2024_02::*;

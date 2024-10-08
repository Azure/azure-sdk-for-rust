#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-preview-2021-12")]
pub mod package_preview_2021_12;
#[cfg(feature = "package-preview-2023-08")]
pub mod package_preview_2023_08;
#[cfg(feature = "package-preview-2023-12")]
pub mod package_preview_2023_12;
#[cfg(feature = "package-preview-2024-01")]
pub mod package_preview_2024_01;
#[cfg(feature = "package-preview-2024-07")]
pub mod package_preview_2024_07;
#[cfg(all(feature = "default_tag", feature = "package-preview-2024-07"))]
pub use package_preview_2024_07::*;

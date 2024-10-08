#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2024-08-01-preview")]
pub mod package_2024_08_01_preview;
#[cfg(feature = "package-2024-09-01-preview")]
pub mod package_2024_09_01_preview;
#[cfg(feature = "package-2024-10-01-preview")]
pub mod package_2024_10_01_preview;
#[cfg(feature = "package-preview-2022-11")]
pub mod package_preview_2022_11;
#[cfg(feature = "package-preview-2024-05")]
pub mod package_preview_2024_05;
#[cfg(all(feature = "default_tag", feature = "package-2024-10-01-preview"))]
pub use package_2024_10_01_preview::*;

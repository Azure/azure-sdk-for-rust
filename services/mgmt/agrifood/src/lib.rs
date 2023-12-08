#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2020-05-12-preview")]
pub mod package_2020_05_12_preview;
#[cfg(feature = "package-preview-2021-09")]
pub mod package_preview_2021_09;
#[cfg(feature = "package-preview-2023-06")]
pub mod package_preview_2023_06;
#[cfg(all(feature = "default_tag", feature = "package-preview-2023-06"))]
pub use package_preview_2023_06::*;

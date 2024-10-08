#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2024-05")]
pub mod package_2024_05;
#[cfg(feature = "package-2024-05-preview")]
pub mod package_2024_05_preview;
#[cfg(feature = "package-2024-08")]
pub mod package_2024_08;
#[cfg(feature = "package-2024-11")]
pub mod package_2024_11;
#[cfg(feature = "package-2025-01")]
pub mod package_2025_01;
#[cfg(all(feature = "default_tag", feature = "package-2025-01"))]
pub use package_2025_01::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2022-08-01")]
pub mod package_2022_08_01;
#[cfg(feature = "package-2024-01")]
pub mod package_2024_01;
#[cfg(feature = "package-2024-04-01-preview")]
pub mod package_2024_04_01_preview;
#[cfg(feature = "package-2024-06-01-preview")]
pub mod package_2024_06_01_preview;
#[cfg(feature = "package-2024-09-23-preview")]
pub mod package_2024_09_23_preview;
#[cfg(all(feature = "default_tag", feature = "package-2024-01"))]
pub use package_2024_01::*;

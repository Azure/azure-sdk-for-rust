#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2020-05")]
pub mod package_2020_05;
#[cfg(feature = "package-2020-09-preview")]
pub mod package_2020_09_preview;
#[cfg(feature = "package-2020-11-preview")]
pub mod package_2020_11_preview;
#[cfg(feature = "package-2021-10")]
pub mod package_2021_10;
#[cfg(feature = "package-2024-04")]
pub mod package_2024_04;
#[cfg(all(feature = "default_tag", feature = "package-2024-04"))]
pub use package_2024_04::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2019-01-01-preview")]
pub mod package_2019_01_01_preview;
#[cfg(feature = "package-2020-05-01-preview")]
pub mod package_2020_05_01_preview;
#[cfg(feature = "package-2021-04-01")]
pub mod package_2021_04_01;
#[cfg(feature = "package-preview-2023-01")]
pub mod package_preview_2023_01;
#[cfg(feature = "package-preview-2023-05")]
pub mod package_preview_2023_05;
#[cfg(all(feature = "default_tag", feature = "package-2021-04-01"))]
pub use package_2021_04_01::*;

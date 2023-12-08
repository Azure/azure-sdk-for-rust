#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2021-04-01-preview")]
pub mod package_2021_04_01_preview;
#[cfg(feature = "package-2021-10-01")]
pub mod package_2021_10_01;
#[cfg(feature = "package-2022-05-01-preview")]
pub mod package_2022_05_01_preview;
#[cfg(feature = "package-2022-10-01-preview")]
pub mod package_2022_10_01_preview;
#[cfg(feature = "package-preview-2023-11")]
pub mod package_preview_2023_11;
#[cfg(all(feature = "default_tag", feature = "package-2021-10-01"))]
pub use package_2021_10_01::*;

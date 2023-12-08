#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2021-10-01")]
pub mod package_2021_10_01;
#[cfg(feature = "package-2022-08-17-preview")]
pub mod package_2022_08_17_preview;
#[cfg(feature = "package-2022-09-25-preview")]
pub mod package_2022_09_25_preview;
#[cfg(feature = "package-2023-01-01-preview")]
pub mod package_2023_01_01_preview;
#[cfg(feature = "package-2023-05-01-preview")]
pub mod package_2023_05_01_preview;
#[cfg(all(feature = "default_tag", feature = "package-2021-10-01"))]
pub use package_2021_10_01::*;

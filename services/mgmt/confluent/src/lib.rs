#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2020-03-01-preview")]
pub mod package_2020_03_01_preview;
#[cfg(feature = "package-2021-03-01-preview")]
pub mod package_2021_03_01_preview;
#[cfg(feature = "package-2021-12-01")]
pub mod package_2021_12_01;
#[cfg(feature = "package-2023-08-22")]
pub mod package_2023_08_22;
#[cfg(feature = "package-preview-2021-09")]
pub mod package_preview_2021_09;
#[cfg(all(feature = "default_tag", feature = "package-2023-08-22"))]
pub use package_2023_08_22::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2019-11-04-preview")]
pub mod package_2019_11_04_preview;
#[cfg(feature = "package-2021-05-06-preview")]
pub mod package_2021_05_06_preview;
#[cfg(feature = "package-2021-11-01-preview")]
pub mod package_2021_11_01_preview;
#[cfg(feature = "package-2022-09-12-preview")]
pub mod package_2022_09_12_preview;
#[cfg(all(feature = "default_tag", feature = "package-2022-09-12-preview"))]
pub use package_2022_09_12_preview::*;

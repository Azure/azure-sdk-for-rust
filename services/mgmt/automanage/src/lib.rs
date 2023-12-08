#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2020-06-30-preview")]
pub mod package_2020_06_30_preview;
#[cfg(feature = "package-2021-04-30-preview")]
pub mod package_2021_04_30_preview;
#[cfg(feature = "package-2022-05")]
pub mod package_2022_05;
#[cfg(all(feature = "default_tag", feature = "package-2022-05"))]
pub use package_2022_05::*;

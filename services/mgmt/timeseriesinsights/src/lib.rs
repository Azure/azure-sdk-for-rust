#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2017-11-15")]
pub mod package_2017_11_15;
#[cfg(feature = "package-2018-08-preview")]
pub mod package_2018_08_preview;
#[cfg(feature = "package-2020-05-15")]
pub mod package_2020_05_15;
#[cfg(feature = "package-preview-2021-03")]
pub mod package_preview_2021_03;
#[cfg(feature = "package-preview-2021-06")]
pub mod package_preview_2021_06;
#[cfg(all(feature = "default_tag", feature = "package-2020-05-15"))]
pub use package_2020_05_15::*;

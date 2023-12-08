#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2018-06-01")]
pub mod package_2018_06_01;
#[cfg(feature = "package-2018-06-01-preview")]
pub mod package_2018_06_01_preview;
#[cfg(feature = "package-2018-06-01-privatepreview")]
pub mod package_2018_06_01_privatepreview;
#[cfg(feature = "package-2020-01-01")]
pub mod package_2020_01_01;
#[cfg(feature = "package-2020-01-01-privatepreview")]
pub mod package_2020_01_01_privatepreview;
#[cfg(all(feature = "default_tag", feature = "package-2020-01-01"))]
pub use package_2020_01_01::*;

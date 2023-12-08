#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2016-01")]
pub mod package_2016_01;
#[cfg(feature = "package-2017-06-01")]
pub mod package_2017_06_01;
#[cfg(feature = "package-2022-06")]
pub mod package_2022_06;
#[cfg(feature = "package-preview-2020-06")]
pub mod package_preview_2020_06;
#[cfg(all(feature = "default_tag", feature = "package-2022-06"))]
pub use package_2022_06::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2017-03-01-preview")]
pub mod package_2017_03_01_preview;
#[cfg(feature = "package-preview-2019-07")]
pub mod package_preview_2019_07;
#[cfg(all(feature = "default_tag", feature = "package-preview-2019-07"))]
pub use package_preview_2019_07::*;

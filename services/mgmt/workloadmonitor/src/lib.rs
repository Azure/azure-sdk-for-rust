#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2018-08-31-preview")]
pub mod package_2018_08_31_preview;
#[cfg(feature = "package-2020-01-13-preview")]
pub mod package_2020_01_13_preview;
#[cfg(all(feature = "default_tag", feature = "package-2020-01-13-preview"))]
pub use package_2020_01_13_preview::*;

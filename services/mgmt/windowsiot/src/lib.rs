#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2018-02-preview")]
pub mod package_2018_02_preview;
#[cfg(feature = "package-2019-06")]
pub mod package_2019_06;
#[cfg(all(feature = "default_tag", feature = "package-2019-06"))]
pub use package_2019_06::*;

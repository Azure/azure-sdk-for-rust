#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2018-03")]
pub mod package_2018_03;
#[cfg(all(feature = "package-2018-03", not(feature = "without_tag_import")))]
pub use package_2018_03::*;
#[cfg(feature = "package-2017-05")]
pub mod package_2017_05;
#[cfg(all(feature = "package-2017-05", not(feature = "without_tag_import")))]
pub use package_2017_05::*;

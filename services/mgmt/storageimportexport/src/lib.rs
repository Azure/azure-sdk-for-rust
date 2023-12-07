#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-preview-2021-01")]
pub mod package_preview_2021_01;
#[cfg(all(feature = "package-preview-2021-01", not(feature = "without_tag_import")))]
pub use package_preview_2021_01::*;
#[cfg(feature = "package-2020-08")]
pub mod package_2020_08;
#[cfg(all(feature = "package-2020-08", not(feature = "without_tag_import")))]
pub use package_2020_08::*;
#[cfg(feature = "package-2016-11")]
pub mod package_2016_11;
#[cfg(all(feature = "package-2016-11", not(feature = "without_tag_import")))]
pub use package_2016_11::*;

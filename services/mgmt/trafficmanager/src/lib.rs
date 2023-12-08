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
#[cfg(feature = "package-2018-04")]
pub mod package_2018_04;
#[cfg(feature = "package-2018-08")]
pub mod package_2018_08;
#[cfg(feature = "package-2022-04")]
pub mod package_2022_04;
#[cfg(feature = "package-preview-2022-04")]
pub mod package_preview_2022_04;
#[cfg(all(feature = "default_tag", feature = "package-2022-04"))]
pub use package_2022_04::*;

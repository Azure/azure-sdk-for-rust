#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2020-06")]
pub mod package_2020_06;
#[cfg(feature = "package-2020-10")]
pub mod package_2020_10;
#[cfg(feature = "package-2021-04")]
pub mod package_2021_04;
#[cfg(feature = "package-2021-06")]
pub mod package_2021_06;
#[cfg(all(feature = "default_tag", feature = "package-2021-06"))]
pub use package_2021_06::*;

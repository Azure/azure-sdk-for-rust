#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2019-10-01")]
pub mod package_2019_10_01;
#[cfg(feature = "package-2020-03-01")]
pub mod package_2020_03_01;
#[cfg(feature = "package-2020-09-01")]
pub mod package_2020_09_01;
#[cfg(feature = "package-2022-06-01")]
pub mod package_2022_06_01;
#[cfg(feature = "package-2022-09")]
pub mod package_2022_09;
#[cfg(all(feature = "default_tag", feature = "package-2022-09"))]
pub use package_2022_09::*;

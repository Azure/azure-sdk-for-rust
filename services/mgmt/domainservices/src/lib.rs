#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2020-01")]
pub mod package_2020_01;
#[cfg(feature = "package-2021-03")]
pub mod package_2021_03;
#[cfg(feature = "package-2021-05")]
pub mod package_2021_05;
#[cfg(feature = "package-2022-09")]
pub mod package_2022_09;
#[cfg(feature = "package-2022-12")]
pub mod package_2022_12;
#[cfg(all(feature = "default_tag", feature = "package-2022-12"))]
pub use package_2022_12::*;

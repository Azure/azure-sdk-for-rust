#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2021-10")]
pub mod package_2021_10;
#[cfg(feature = "package-2022-03")]
pub mod package_2022_03;
#[cfg(feature = "package-2022-04")]
pub mod package_2022_04;
#[cfg(feature = "package-2022-09")]
pub mod package_2022_09;
#[cfg(feature = "package-2023-03")]
pub mod package_2023_03;
#[cfg(all(feature = "default_tag", feature = "package-2023-03"))]
pub use package_2023_03::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2018-11-20")]
pub mod package_2018_11_20;
#[cfg(feature = "package-2020-06-25")]
pub mod package_2020_06_25;
#[cfg(feature = "package-2021-01-25")]
pub mod package_2021_01_25;
#[cfg(feature = "package-2022-01-25")]
pub mod package_2022_01_25;
#[cfg(feature = "package-2024-04-05")]
pub mod package_2024_04_05;
#[cfg(all(feature = "default_tag", feature = "package-2024-04-05"))]
pub use package_2024_04_05::*;

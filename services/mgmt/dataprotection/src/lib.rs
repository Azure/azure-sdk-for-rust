#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2023-05")]
pub mod package_2023_05;
#[cfg(feature = "package-2023-11")]
pub mod package_2023_11;
#[cfg(feature = "package-2023-12")]
pub mod package_2023_12;
#[cfg(feature = "package-2024-03")]
pub mod package_2024_03;
#[cfg(feature = "package-2024-04")]
pub mod package_2024_04;
#[cfg(all(feature = "default_tag", feature = "package-2024-04"))]
pub use package_2024_04::*;

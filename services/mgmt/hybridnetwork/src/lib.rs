#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2020-01-01-preview")]
pub mod package_2020_01_01_preview;
#[cfg(feature = "package-2021-05-01")]
pub mod package_2021_05_01;
#[cfg(feature = "package-2022-01-01-preview")]
pub mod package_2022_01_01_preview;
#[cfg(feature = "package-2023-09-01")]
pub mod package_2023_09_01;
#[cfg(feature = "package-2024-04-15")]
pub mod package_2024_04_15;
#[cfg(all(feature = "default_tag", feature = "package-2024-04-15"))]
pub use package_2024_04_15::*;

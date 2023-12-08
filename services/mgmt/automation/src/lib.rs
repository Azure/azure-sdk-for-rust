#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2022-01-31")]
pub mod package_2022_01_31;
#[cfg(feature = "package-2022-02-22")]
pub mod package_2022_02_22;
#[cfg(feature = "package-2022-08-08")]
pub mod package_2022_08_08;
#[cfg(feature = "package-2023-05-15-preview")]
pub mod package_2023_05_15_preview;
#[cfg(feature = "package-2023-11-01")]
pub mod package_2023_11_01;
#[cfg(all(feature = "default_tag", feature = "package-2023-11-01"))]
pub use package_2023_11_01::*;

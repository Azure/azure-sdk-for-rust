#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2020-12-08-preview")]
pub mod package_2020_12_08_preview;
#[cfg(feature = "package-2021-06-10")]
pub mod package_2021_06_10;
#[cfg(feature = "package-2021-08-24")]
pub mod package_2021_08_24;
#[cfg(feature = "package-2022-08-08")]
pub mod package_2022_08_08;
#[cfg(feature = "package-2023-05-01")]
pub mod package_2023_05_01;
#[cfg(all(feature = "default_tag", feature = "package-2023-05-01"))]
pub use package_2023_05_01::*;

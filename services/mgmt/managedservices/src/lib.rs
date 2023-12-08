#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2019-06")]
pub mod package_2019_06;
#[cfg(feature = "package-2019-09")]
pub mod package_2019_09;
#[cfg(feature = "package-2020-02-preview")]
pub mod package_2020_02_preview;
#[cfg(feature = "package-2022-10")]
pub mod package_2022_10;
#[cfg(feature = "package-preview-2022-01")]
pub mod package_preview_2022_01;
#[cfg(all(feature = "default_tag", feature = "package-2022-10"))]
pub use package_2022_10::*;

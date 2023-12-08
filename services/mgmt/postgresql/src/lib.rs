#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-flexibleserver-2022-03-preview")]
pub mod package_flexibleserver_2022_03_preview;
#[cfg(feature = "package-flexibleserver-2022-03-privatepreview")]
pub mod package_flexibleserver_2022_03_privatepreview;
#[cfg(feature = "package-flexibleserver-2022-12-01")]
pub mod package_flexibleserver_2022_12_01;
#[cfg(feature = "package-flexibleserver-2023-03-01-preview")]
pub mod package_flexibleserver_2023_03_01_preview;
#[cfg(feature = "package-preview-2023-06")]
pub mod package_preview_2023_06;
#[cfg(all(feature = "default_tag", feature = "package-flexibleserver-2022-12-01"))]
pub use package_flexibleserver_2022_12_01::*;

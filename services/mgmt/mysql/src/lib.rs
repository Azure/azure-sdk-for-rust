#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-flexibleserver-2023-12-01-preview")]
pub mod package_flexibleserver_2023_12_01_preview;
#[cfg(feature = "package-flexibleserver-2023-12-30")]
pub mod package_flexibleserver_2023_12_30;
#[cfg(feature = "package-flexibleserver-2024-01-01")]
pub mod package_flexibleserver_2024_01_01;
#[cfg(feature = "package-flexibleserver-2024-02-01-preview")]
pub mod package_flexibleserver_2024_02_01_preview;
#[cfg(feature = "package-flexibleserver-2024-06-01-preview")]
pub mod package_flexibleserver_2024_06_01_preview;
#[cfg(all(feature = "default_tag", feature = "package-flexibleserver-2024-01-01"))]
pub use package_flexibleserver_2024_01_01::*;

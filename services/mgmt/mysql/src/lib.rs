#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-flexibleserver-2023-06-01-preview")]
pub mod package_flexibleserver_2023_06_01_preview;
#[cfg(feature = "package-flexibleserver-2023-06-01-preview-new")]
pub mod package_flexibleserver_2023_06_01_preview_new;
#[cfg(feature = "package-flexibleserver-2023-06-30")]
pub mod package_flexibleserver_2023_06_30;
#[cfg(feature = "package-flexibleserver-2023-06-30-privatelink")]
pub mod package_flexibleserver_2023_06_30_privatelink;
#[cfg(feature = "package-flexibleserver-2023-10-01-preview")]
pub mod package_flexibleserver_2023_10_01_preview;
#[cfg(all(feature = "default_tag", feature = "package-flexibleserver-2023-06-30-privatelink"))]
pub use package_flexibleserver_2023_06_30_privatelink::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-composite-v1")]
pub mod package_composite_v1;
#[cfg(feature = "package-composite-v2")]
pub mod package_composite_v2;
#[cfg(feature = "package-kusto-pool-2021-04-preview")]
pub mod package_kusto_pool_2021_04_preview;
#[cfg(feature = "package-preview-2021-06")]
pub mod package_preview_2021_06;
#[cfg(feature = "package-sqlGen3-2020-04-01-preview")]
pub mod package_sqlgen3_2020_04_01_preview;
#[cfg(all(feature = "default_tag", feature = "package-composite-v2"))]
pub use package_composite_v2::*;

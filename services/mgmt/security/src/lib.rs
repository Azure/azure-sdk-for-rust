#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2023-11-15")]
pub mod package_2023_11_15;
#[cfg(feature = "package-2024-01")]
pub mod package_2024_01;
#[cfg(feature = "package-composite-v1")]
pub mod package_composite_v1;
#[cfg(feature = "package-composite-v2")]
pub mod package_composite_v2;
#[cfg(feature = "package-dotnet-sdk")]
pub mod package_dotnet_sdk;
#[cfg(all(feature = "default_tag", feature = "package-dotnet-sdk"))]
pub use package_dotnet_sdk::*;

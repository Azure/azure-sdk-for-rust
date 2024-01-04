#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2023-06")]
pub mod package_2023_06;
#[cfg(feature = "package-2023-07-preview")]
pub mod package_2023_07_preview;
#[cfg(feature = "package-2023-08-preview")]
pub mod package_2023_08_preview;
#[cfg(feature = "package-2023-09")]
pub mod package_2023_09;
#[cfg(feature = "profile-hybrid-2020-09-01")]
pub mod profile_hybrid_2020_09_01;
#[cfg(all(feature = "default_tag", feature = "package-2023-09"))]
pub use package_2023_09::*;

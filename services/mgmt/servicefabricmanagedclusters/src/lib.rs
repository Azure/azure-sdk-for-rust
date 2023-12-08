#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2022-08-preview")]
pub mod package_2022_08_preview;
#[cfg(feature = "package-2022-10-preview")]
pub mod package_2022_10_preview;
#[cfg(feature = "package-2023-02-preview")]
pub mod package_2023_02_preview;
#[cfg(feature = "package-2023-03-preview")]
pub mod package_2023_03_preview;
#[cfg(feature = "package-2023-07-preview")]
pub mod package_2023_07_preview;
#[cfg(all(feature = "default_tag", feature = "package-2023-07-preview"))]
pub use package_2023_07_preview::*;

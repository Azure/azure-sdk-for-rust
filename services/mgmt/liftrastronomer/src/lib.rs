#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2023-08-01")]
pub mod package_2023_08_01;
#[cfg(feature = "package-2023-08-01-preview")]
pub mod package_2023_08_01_preview;
#[cfg(feature = "package-2024-08-27")]
pub mod package_2024_08_27;
#[cfg(feature = "package-2024-08-27-preview")]
pub mod package_2024_08_27_preview;
#[cfg(all(feature = "default_tag", feature = "package-2024-08-27"))]
pub use package_2024_08_27::*;

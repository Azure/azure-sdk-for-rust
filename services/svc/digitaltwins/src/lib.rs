#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2021-06-30-preview")]
pub mod package_2021_06_30_preview;
#[cfg(feature = "package-2022-05-31")]
pub mod package_2022_05_31;
#[cfg(feature = "package-2023-06-30")]
pub mod package_2023_06_30;
#[cfg(feature = "package-2023-10-31")]
pub mod package_2023_10_31;
#[cfg(feature = "package-preview-2023-02-27")]
pub mod package_preview_2023_02_27;
#[cfg(all(feature = "default_tag", feature = "package-2023-10-31"))]
pub use package_2023_10_31::*;

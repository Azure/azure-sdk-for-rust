#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2020-12-preview")]
pub mod package_2020_12_preview;
#[cfg(feature = "package-2021-12")]
pub mod package_2021_12;
#[cfg(feature = "package-2022-05-preview")]
pub mod package_2022_05_preview;
#[cfg(feature = "package-2024-02")]
pub mod package_2024_02;
#[cfg(all(feature = "default_tag", feature = "package-2024-02"))]
pub use package_2024_02::*;

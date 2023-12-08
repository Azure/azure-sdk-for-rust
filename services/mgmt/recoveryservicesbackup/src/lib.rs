#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2023-02")]
pub mod package_2023_02;
#[cfg(feature = "package-2023-04")]
pub mod package_2023_04;
#[cfg(feature = "package-2023-06")]
pub mod package_2023_06;
#[cfg(feature = "package-passivestamp-2023-01-15")]
pub mod package_passivestamp_2023_01_15;
#[cfg(feature = "package-preview-2022-09")]
pub mod package_preview_2022_09;
#[cfg(all(feature = "default_tag", feature = "package-2023-06"))]
pub use package_2023_06::*;

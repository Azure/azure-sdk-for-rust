#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-preview-2023-07")]
pub mod package_preview_2023_07;
#[cfg(all(feature = "package-preview-2023-07", not(feature = "without_tag_import")))]
pub use package_preview_2023_07::*;
#[cfg(feature = "package-2023-10")]
pub mod package_2023_10;
#[cfg(all(feature = "package-2023-10", not(feature = "without_tag_import")))]
pub use package_2023_10::*;
#[cfg(feature = "package-2023-03-01")]
pub mod package_2023_03_01;
#[cfg(all(feature = "package-2023-03-01", not(feature = "without_tag_import")))]
pub use package_2023_03_01::*;
#[cfg(feature = "package-2022-07-01-preview")]
pub mod package_2022_07_01_preview;
#[cfg(all(feature = "package-2022-07-01-preview", not(feature = "without_tag_import")))]
pub use package_2022_07_01_preview::*;

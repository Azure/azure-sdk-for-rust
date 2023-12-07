#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-preview-2022-01")]
pub mod package_preview_2022_01;
#[cfg(all(feature = "package-preview-2022-01", not(feature = "without_tag_import")))]
pub use package_preview_2022_01::*;
#[cfg(feature = "package-preview-2021-09-30")]
pub mod package_preview_2021_09_30;
#[cfg(all(feature = "package-preview-2021-09-30", not(feature = "without_tag_import")))]
pub use package_preview_2021_09_30::*;
#[cfg(feature = "package-2023-07-31-preview")]
pub mod package_2023_07_31_preview;
#[cfg(all(feature = "package-2023-07-31-preview", not(feature = "without_tag_import")))]
pub use package_2023_07_31_preview::*;
#[cfg(feature = "package-2023-01-31")]
pub mod package_2023_01_31;
#[cfg(all(feature = "package-2023-01-31", not(feature = "without_tag_import")))]
pub use package_2023_01_31::*;
#[cfg(feature = "package-2018-11-30")]
pub mod package_2018_11_30;
#[cfg(all(feature = "package-2018-11-30", not(feature = "without_tag_import")))]
pub use package_2018_11_30::*;

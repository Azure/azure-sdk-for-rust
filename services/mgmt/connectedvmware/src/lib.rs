#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-preview-2023-03")]
pub mod package_preview_2023_03;
#[cfg(all(feature = "package-preview-2023-03", not(feature = "without_tag_import")))]
pub use package_preview_2023_03::*;
#[cfg(feature = "package-2023-10")]
pub mod package_2023_10;
#[cfg(all(feature = "package-2023-10", not(feature = "without_tag_import")))]
pub use package_2023_10::*;
#[cfg(feature = "package-2022-07-15-preview")]
pub mod package_2022_07_15_preview;
#[cfg(all(feature = "package-2022-07-15-preview", not(feature = "without_tag_import")))]
pub use package_2022_07_15_preview::*;
#[cfg(feature = "package-2022-01-10-preview")]
pub mod package_2022_01_10_preview;
#[cfg(all(feature = "package-2022-01-10-preview", not(feature = "without_tag_import")))]
pub use package_2022_01_10_preview::*;
#[cfg(feature = "package-2020-10-01-preview")]
pub mod package_2020_10_01_preview;
#[cfg(all(feature = "package-2020-10-01-preview", not(feature = "without_tag_import")))]
pub use package_2020_10_01_preview::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2023-07-01")]
pub mod package_2023_07_01;
#[cfg(all(feature = "package-2023-07-01", not(feature = "without_tag_import")))]
pub use package_2023_07_01::*;
#[cfg(feature = "package-2022-12-01-preview")]
pub mod package_2022_12_01_preview;
#[cfg(all(feature = "package-2022-12-01-preview", not(feature = "without_tag_import")))]
pub use package_2022_12_01_preview::*;
#[cfg(feature = "package-2022-10-01")]
pub mod package_2022_10_01;
#[cfg(all(feature = "package-2022-10-01", not(feature = "without_tag_import")))]
pub use package_2022_10_01::*;
#[cfg(feature = "package-2022-04-01-preview")]
pub mod package_2022_04_01_preview;
#[cfg(all(feature = "package-2022-04-01-preview", not(feature = "without_tag_import")))]
pub use package_2022_04_01_preview::*;
#[cfg(feature = "package-2020-03-01-preview")]
pub mod package_2020_03_01_preview;
#[cfg(all(feature = "package-2020-03-01-preview", not(feature = "without_tag_import")))]
pub use package_2020_03_01_preview::*;

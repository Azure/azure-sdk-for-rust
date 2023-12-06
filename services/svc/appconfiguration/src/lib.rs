#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2023-11-01")]
pub mod package_2023_11_01;
#[cfg(all(feature = "package-2023-11-01", not(feature = "without_tag_import")))]
pub use package_2023_11_01::*;
#[cfg(feature = "package-2023-10-01")]
pub mod package_2023_10_01;
#[cfg(all(feature = "package-2023-10-01", not(feature = "without_tag_import")))]
pub use package_2023_10_01::*;
#[cfg(feature = "package-2023-05-01-preview")]
pub mod package_2023_05_01_preview;
#[cfg(all(feature = "package-2023-05-01-preview", not(feature = "without_tag_import")))]
pub use package_2023_05_01_preview::*;
#[cfg(feature = "package-2022-11-01-preview")]
pub mod package_2022_11_01_preview;
#[cfg(all(feature = "package-2022-11-01-preview", not(feature = "without_tag_import")))]
pub use package_2022_11_01_preview::*;
#[cfg(feature = "package-2019-07")]
pub mod package_2019_07;
#[cfg(all(feature = "package-2019-07", not(feature = "without_tag_import")))]
pub use package_2019_07::*;

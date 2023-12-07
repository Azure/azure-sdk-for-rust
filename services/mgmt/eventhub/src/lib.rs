#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2024-01")]
pub mod package_2024_01;
#[cfg(all(feature = "package-2024-01", not(feature = "without_tag_import")))]
pub use package_2024_01::*;
#[cfg(feature = "package-2023-01-preview")]
pub mod package_2023_01_preview;
#[cfg(all(feature = "package-2023-01-preview", not(feature = "without_tag_import")))]
pub use package_2023_01_preview::*;
#[cfg(feature = "package-2022-10-preview")]
pub mod package_2022_10_preview;
#[cfg(all(feature = "package-2022-10-preview", not(feature = "without_tag_import")))]
pub use package_2022_10_preview::*;
#[cfg(feature = "package-2022-01-preview")]
pub mod package_2022_01_preview;
#[cfg(all(feature = "package-2022-01-preview", not(feature = "without_tag_import")))]
pub use package_2022_01_preview::*;
#[cfg(feature = "package-2021-11")]
pub mod package_2021_11;
#[cfg(all(feature = "package-2021-11", not(feature = "without_tag_import")))]
pub use package_2021_11::*;

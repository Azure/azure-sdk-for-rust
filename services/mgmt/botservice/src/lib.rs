#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-preview-2023-09")]
pub mod package_preview_2023_09;
#[cfg(all(feature = "package-preview-2023-09", not(feature = "without_tag_import")))]
pub use package_preview_2023_09::*;
#[cfg(feature = "package-preview-2022-06")]
pub mod package_preview_2022_06;
#[cfg(all(feature = "package-preview-2022-06", not(feature = "without_tag_import")))]
pub use package_preview_2022_06::*;
#[cfg(feature = "package-preview-2021-05")]
pub mod package_preview_2021_05;
#[cfg(all(feature = "package-preview-2021-05", not(feature = "without_tag_import")))]
pub use package_preview_2021_05::*;
#[cfg(feature = "package-2022-09")]
pub mod package_2022_09;
#[cfg(all(feature = "package-2022-09", not(feature = "without_tag_import")))]
pub use package_2022_09::*;
#[cfg(feature = "package-2021-03-01")]
pub mod package_2021_03_01;
#[cfg(all(feature = "package-2021-03-01", not(feature = "without_tag_import")))]
pub use package_2021_03_01::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-preview-2022-10")]
pub mod package_preview_2022_10;
#[cfg(all(feature = "package-preview-2022-10", not(feature = "without_tag_import")))]
pub use package_preview_2022_10::*;
#[cfg(feature = "package-preview-2022-01")]
pub mod package_preview_2022_01;
#[cfg(all(feature = "package-preview-2022-01", not(feature = "without_tag_import")))]
pub use package_preview_2022_01::*;
#[cfg(feature = "package-preview-2021-06")]
pub mod package_preview_2021_06;
#[cfg(all(feature = "package-preview-2021-06", not(feature = "without_tag_import")))]
pub use package_preview_2021_06::*;
#[cfg(feature = "package-2023-02")]
pub mod package_2023_02;
#[cfg(all(feature = "package-2023-02", not(feature = "without_tag_import")))]
pub use package_2023_02::*;
#[cfg(feature = "package-2022-12")]
pub mod package_2022_12;
#[cfg(all(feature = "package-2022-12", not(feature = "without_tag_import")))]
pub use package_2022_12::*;

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
#[cfg(feature = "package-preview-2022-11")]
pub mod package_preview_2022_11;
#[cfg(all(feature = "package-preview-2022-11", not(feature = "without_tag_import")))]
pub use package_preview_2022_11::*;
#[cfg(feature = "package-preview-2022-05")]
pub mod package_preview_2022_05;
#[cfg(all(feature = "package-preview-2022-05", not(feature = "without_tag_import")))]
pub use package_preview_2022_05::*;
#[cfg(feature = "package-2023-05")]
pub mod package_2023_05;
#[cfg(all(feature = "package-2023-05", not(feature = "without_tag_import")))]
pub use package_2023_05::*;
#[cfg(feature = "package-2021-06")]
pub mod package_2021_06;
#[cfg(all(feature = "package-2021-06", not(feature = "without_tag_import")))]
pub use package_2021_06::*;

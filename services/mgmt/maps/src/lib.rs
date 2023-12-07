#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-preview-2023-08")]
pub mod package_preview_2023_08;
#[cfg(all(feature = "package-preview-2023-08", not(feature = "without_tag_import")))]
pub use package_preview_2023_08::*;
#[cfg(feature = "package-preview-2021-12")]
pub mod package_preview_2021_12;
#[cfg(all(feature = "package-preview-2021-12", not(feature = "without_tag_import")))]
pub use package_preview_2021_12::*;
#[cfg(feature = "package-preview-2021-07")]
pub mod package_preview_2021_07;
#[cfg(all(feature = "package-preview-2021-07", not(feature = "without_tag_import")))]
pub use package_preview_2021_07::*;
#[cfg(feature = "package-preview-2020-02")]
pub mod package_preview_2020_02;
#[cfg(all(feature = "package-preview-2020-02", not(feature = "without_tag_import")))]
pub use package_preview_2020_02::*;
#[cfg(feature = "package-2023-06")]
pub mod package_2023_06;
#[cfg(all(feature = "package-2023-06", not(feature = "without_tag_import")))]
pub use package_2023_06::*;

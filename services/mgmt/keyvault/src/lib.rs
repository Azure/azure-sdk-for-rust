#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "profile-hybrid-2020-09-01")]
pub mod profile_hybrid_2020_09_01;
#[cfg(all(feature = "profile-hybrid-2020-09-01", not(feature = "without_tag_import")))]
pub use profile_hybrid_2020_09_01::*;
#[cfg(feature = "package-preview-2022-02")]
pub mod package_preview_2022_02;
#[cfg(all(feature = "package-preview-2022-02", not(feature = "without_tag_import")))]
pub use package_preview_2022_02::*;
#[cfg(feature = "package-preview-2021-11")]
pub mod package_preview_2021_11;
#[cfg(all(feature = "package-preview-2021-11", not(feature = "without_tag_import")))]
pub use package_preview_2021_11::*;
#[cfg(feature = "package-preview-2021-06")]
pub mod package_preview_2021_06;
#[cfg(all(feature = "package-preview-2021-06", not(feature = "without_tag_import")))]
pub use package_preview_2021_06::*;
#[cfg(feature = "package-preview-2021-04-full")]
pub mod package_preview_2021_04_full;
#[cfg(all(feature = "package-preview-2021-04-full", not(feature = "without_tag_import")))]
pub use package_preview_2021_04_full::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-preview-7_5-preview_1")]
pub mod package_preview_7_5_preview_1;
#[cfg(all(feature = "package-preview-7_5-preview_1", not(feature = "without_tag_import")))]
pub use package_preview_7_5_preview_1::*;
#[cfg(feature = "package-preview-7_4-preview_1")]
pub mod package_preview_7_4_preview_1;
#[cfg(all(feature = "package-preview-7_4-preview_1", not(feature = "without_tag_import")))]
pub use package_preview_7_4_preview_1::*;
#[cfg(feature = "package-preview-7_3-preview")]
pub mod package_preview_7_3_preview;
#[cfg(all(feature = "package-preview-7_3-preview", not(feature = "without_tag_import")))]
pub use package_preview_7_3_preview::*;
#[cfg(feature = "package-7_4")]
pub mod package_7_4;
#[cfg(all(feature = "package-7_4", not(feature = "without_tag_import")))]
pub use package_7_4::*;
#[cfg(feature = "package-7_3")]
pub mod package_7_3;
#[cfg(all(feature = "package-7_3", not(feature = "without_tag_import")))]
pub use package_7_3::*;

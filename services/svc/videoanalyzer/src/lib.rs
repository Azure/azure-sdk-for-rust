#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-ava-edge-1-0-0-preview")]
pub mod package_ava_edge_1_0_0_preview;
#[cfg(feature = "package-preview-1_1_0")]
pub mod package_preview_1_1_0;
#[cfg(all(feature = "default_tag", feature = "package-preview-1_1_0"))]
pub use package_preview_1_1_0::*;

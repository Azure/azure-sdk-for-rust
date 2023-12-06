#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-preview-2023-11")]
pub mod package_preview_2023_11;
#[cfg(all(feature = "package-preview-2023-11", not(feature = "without_tag_import")))]
pub use package_preview_2023_11::*;

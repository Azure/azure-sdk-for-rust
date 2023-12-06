#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2023-10-01-preview")]
pub mod package_2023_10_01_preview;
#[cfg(all(feature = "package-2023-10-01-preview", not(feature = "without_tag_import")))]
pub use package_2023_10_01_preview::*;

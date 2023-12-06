#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-01-03-preview")]
pub mod package_2021_01_03_preview;
#[cfg(all(feature = "package-2021-01-03-preview", not(feature = "without_tag_import")))]
pub use package_2021_01_03_preview::*;

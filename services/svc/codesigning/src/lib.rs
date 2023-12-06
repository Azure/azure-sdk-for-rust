#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "2023-06-15-preview")]
pub mod v2023_06_15_preview;
#[cfg(all(feature = "2023-06-15-preview", not(feature = "without_tag_import")))]
pub use v2023_06_15_preview::*;

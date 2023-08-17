#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "2023-06-15-preview")]
pub mod v2023_06_15_preview;
#[cfg(all(feature = "2023-06-15-preview", not(feature = "no-default-tag")))]
pub use v2023_06_15_preview::*;

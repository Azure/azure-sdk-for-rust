#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(all(feature = "v1", not(feature = "without_tag_import")))]
pub use v1::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "1_6")]
pub mod v1_6;
#[cfg(all(feature = "1_6", not(feature = "without_tag_import")))]
pub use v1_6::*;

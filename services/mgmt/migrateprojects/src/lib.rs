#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2018-09")]
pub mod package_2018_09;
#[cfg(all(feature = "package-2018-09", not(feature = "without_tag_import")))]
pub use package_2018_09::*;

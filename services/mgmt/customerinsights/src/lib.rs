#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2017-04")]
pub mod package_2017_04;
#[cfg(all(feature = "package-2017-04", not(feature = "without_tag_import")))]
pub use package_2017_04::*;
#[cfg(feature = "package-2017-01")]
pub mod package_2017_01;
#[cfg(all(feature = "package-2017-01", not(feature = "without_tag_import")))]
pub use package_2017_01::*;

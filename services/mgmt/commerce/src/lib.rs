#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "profile-hybrid-2020-09-01")]
pub mod profile_hybrid_2020_09_01;
#[cfg(all(feature = "profile-hybrid-2020-09-01", not(feature = "without_tag_import")))]
pub use profile_hybrid_2020_09_01::*;
#[cfg(feature = "package-2015-06-preview")]
pub mod package_2015_06_preview;
#[cfg(all(feature = "package-2015-06-preview", not(feature = "without_tag_import")))]
pub use package_2015_06_preview::*;

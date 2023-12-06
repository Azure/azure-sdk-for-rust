#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2019-06")]
pub mod package_2019_06;
#[cfg(all(feature = "package-2019-06", not(feature = "without_tag_import")))]
pub use package_2019_06::*;
#[cfg(feature = "package-2018-02-preview")]
pub mod package_2018_02_preview;
#[cfg(all(feature = "package-2018-02-preview", not(feature = "without_tag_import")))]
pub use package_2018_02_preview::*;

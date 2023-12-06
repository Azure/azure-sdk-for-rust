#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2019-11")]
pub mod package_preview_2019_11;
#[cfg(all(feature = "package-preview-2019-11", not(feature = "without_tag_import")))]
pub use package_preview_2019_11::*;
#[cfg(feature = "package-2016-11")]
pub mod package_2016_11;
#[cfg(all(feature = "package-2016-11", not(feature = "without_tag_import")))]
pub use package_2016_11::*;
#[cfg(feature = "package-2015-10-preview")]
pub mod package_2015_10_preview;
#[cfg(all(feature = "package-2015-10-preview", not(feature = "without_tag_import")))]
pub use package_2015_10_preview::*;

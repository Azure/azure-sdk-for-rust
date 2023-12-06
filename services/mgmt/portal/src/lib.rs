#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2020-09-01-preview")]
pub mod package_2020_09_01_preview;
#[cfg(all(feature = "package-2020-09-01-preview", not(feature = "without_tag_import")))]
pub use package_2020_09_01_preview::*;
#[cfg(feature = "package-2019-01-01-preview")]
pub mod package_2019_01_01_preview;
#[cfg(all(feature = "package-2019-01-01-preview", not(feature = "without_tag_import")))]
pub use package_2019_01_01_preview::*;
#[cfg(feature = "package-2018-10-01-preview")]
pub mod package_2018_10_01_preview;
#[cfg(all(feature = "package-2018-10-01-preview", not(feature = "without_tag_import")))]
pub use package_2018_10_01_preview::*;
#[cfg(feature = "package-2015-08-01-preview")]
pub mod package_2015_08_01_preview;
#[cfg(all(feature = "package-2015-08-01-preview", not(feature = "without_tag_import")))]
pub use package_2015_08_01_preview::*;

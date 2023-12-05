#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2020-07")]
pub mod package_preview_2020_07;
#[cfg(all(feature = "package-preview-2020-07", not(feature = "without_tag_import")))]
pub use package_preview_2020_07::*;
#[cfg(feature = "package-preview-2020-03")]
pub mod package_preview_2020_03;
#[cfg(all(feature = "package-preview-2020-03", not(feature = "without_tag_import")))]
pub use package_preview_2020_03::*;
#[cfg(feature = "package-preview-2017-04")]
pub mod package_preview_2017_04;
#[cfg(all(feature = "package-preview-2017-04", not(feature = "without_tag_import")))]
pub use package_preview_2017_04::*;
#[cfg(feature = "package-2020-03")]
pub mod package_2020_03;
#[cfg(all(feature = "package-2020-03", not(feature = "without_tag_import")))]
pub use package_2020_03::*;
#[cfg(feature = "package-2017-04-01")]
pub mod package_2017_04_01;
#[cfg(all(feature = "package-2017-04-01", not(feature = "without_tag_import")))]
pub use package_2017_04_01::*;

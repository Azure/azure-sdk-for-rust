#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-preview-2019-07-19")]
pub mod package_preview_2019_07_19;
#[cfg(all(feature = "package-preview-2019-07-19", not(feature = "without_tag_import")))]
pub use package_preview_2019_07_19::*;
#[cfg(feature = "package-preview-2019-04")]
pub mod package_preview_2019_04;
#[cfg(all(feature = "package-preview-2019-04", not(feature = "without_tag_import")))]
pub use package_preview_2019_04::*;
#[cfg(feature = "package-2022-11")]
pub mod package_2022_11;
#[cfg(all(feature = "package-2022-11", not(feature = "without_tag_import")))]
pub use package_2022_11::*;
#[cfg(feature = "package-2022-03")]
pub mod package_2022_03;
#[cfg(all(feature = "package-2022-03", not(feature = "without_tag_import")))]
pub use package_2022_03::*;
#[cfg(feature = "package-2021-07-01")]
pub mod package_2021_07_01;
#[cfg(all(feature = "package-2021-07-01", not(feature = "without_tag_import")))]
pub use package_2021_07_01::*;

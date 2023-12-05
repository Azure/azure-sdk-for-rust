#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-10")]
pub mod package_2021_10;
#[cfg(all(feature = "package-2021-10", not(feature = "without_tag_import")))]
pub use package_2021_10::*;
#[cfg(feature = "package-2020-09")]
pub mod package_2020_09;
#[cfg(all(feature = "package-2020-09", not(feature = "without_tag_import")))]
pub use package_2020_09::*;
#[cfg(feature = "package-2020-01")]
pub mod package_2020_01;
#[cfg(all(feature = "package-2020-01", not(feature = "without_tag_import")))]
pub use package_2020_01::*;
#[cfg(feature = "package-2019-10-preview")]
pub mod package_2019_10_preview;
#[cfg(all(feature = "package-2019-10-preview", not(feature = "without_tag_import")))]
pub use package_2019_10_preview::*;
#[cfg(feature = "package-2019-03-preview")]
pub mod package_2019_03_preview;
#[cfg(all(feature = "package-2019-03-preview", not(feature = "without_tag_import")))]
pub use package_2019_03_preview::*;

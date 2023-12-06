#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2021-08-01")]
pub mod package_2021_08_01;
#[cfg(all(feature = "package-2021-08-01", not(feature = "without_tag_import")))]
pub use package_2021_08_01::*;
#[cfg(feature = "package-2021-04-01-preview")]
pub mod package_2021_04_01_preview;
#[cfg(all(feature = "package-2021-04-01-preview", not(feature = "without_tag_import")))]
pub use package_2021_04_01_preview::*;
#[cfg(feature = "package-2020-03-15-preview")]
pub mod package_2020_03_15_preview;
#[cfg(all(feature = "package-2020-03-15-preview", not(feature = "without_tag_import")))]
pub use package_2020_03_15_preview::*;

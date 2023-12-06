#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-04-01-preview")]
pub mod package_2022_04_01_preview;
#[cfg(all(feature = "package-2022-04-01-preview", not(feature = "without_tag_import")))]
pub use package_2022_04_01_preview::*;
#[cfg(feature = "package-2020-12-16-preview")]
pub mod package_2020_12_16_preview;
#[cfg(all(feature = "package-2020-12-16-preview", not(feature = "without_tag_import")))]
pub use package_2020_12_16_preview::*;

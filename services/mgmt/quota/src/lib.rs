#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-03-15-preview")]
pub mod package_2021_03_15_preview;
#[cfg(all(feature = "package-2021-03-15-preview", not(feature = "no-default-tag")))]
pub use package_2021_03_15_preview::*;
#[cfg(feature = "package-2021-03-15")]
pub mod package_2021_03_15;
#[cfg(all(feature = "package-2021-03-15", not(feature = "no-default-tag")))]
pub use package_2021_03_15::*;

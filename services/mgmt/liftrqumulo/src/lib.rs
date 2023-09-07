#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-10-12-preview")]
pub mod package_2022_10_12_preview;
#[cfg(all(feature = "package-2022-10-12-preview", not(feature = "no-default-tag")))]
pub use package_2022_10_12_preview::*;
#[cfg(feature = "package-2022-10")]
pub mod package_2022_10;
#[cfg(all(feature = "package-2022-10", not(feature = "no-default-tag")))]
pub use package_2022_10::*;
#[cfg(feature = "package-2022-06-27-preview")]
pub mod package_2022_06_27_preview;
#[cfg(all(feature = "package-2022-06-27-preview", not(feature = "no-default-tag")))]
pub use package_2022_06_27_preview::*;

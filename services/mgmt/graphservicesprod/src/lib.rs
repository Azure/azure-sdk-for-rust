#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-04-13")]
pub mod package_2023_04_13;
#[cfg(all(feature = "package-2023-04-13", not(feature = "no-default-tag")))]
pub use package_2023_04_13::*;
#[cfg(feature = "package-2022-09-22-preview")]
pub mod package_2022_09_22_preview;
#[cfg(all(feature = "package-2022-09-22-preview", not(feature = "no-default-tag")))]
pub use package_2022_09_22_preview::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-04-03")]
pub mod package_2023_04_03;
#[cfg(all(feature = "package-2023-04-03", not(feature = "no-default-tag")))]
pub use package_2023_04_03::*;
#[cfg(feature = "package-2023-01-31")]
pub mod package_2023_01_31;
#[cfg(all(feature = "package-2023-01-31", not(feature = "no-default-tag")))]
pub use package_2023_01_31::*;
#[cfg(feature = "package-2022-12-01-preview")]
pub mod package_2022_12_01_preview;
#[cfg(all(feature = "package-2022-12-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_12_01_preview::*;

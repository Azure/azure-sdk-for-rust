#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-05-01")]
pub mod package_2023_05_01;
#[cfg(all(feature = "package-2023-05-01", not(feature = "no-default-tag")))]
pub use package_2023_05_01::*;
#[cfg(feature = "package-2023-02-01")]
pub mod package_2023_02_01;
#[cfg(all(feature = "package-2023-02-01", not(feature = "no-default-tag")))]
pub use package_2023_02_01::*;
#[cfg(feature = "package-2022-04-01-preview")]
pub mod package_2022_04_01_preview;
#[cfg(all(feature = "package-2022-04-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_04_01_preview::*;
#[cfg(feature = "package-2018-04-01")]
pub mod package_2018_04_01;
#[cfg(all(feature = "package-2018-04-01", not(feature = "no-default-tag")))]
pub use package_2018_04_01::*;

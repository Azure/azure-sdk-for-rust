#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-03-01")]
pub mod package_2023_03_01;
#[cfg(all(feature = "package-2023-03-01", not(feature = "no-default-tag")))]
pub use package_2023_03_01::*;
#[cfg(feature = "package-2022-05-01")]
pub mod package_2022_05_01;
#[cfg(all(feature = "package-2022-05-01", not(feature = "no-default-tag")))]
pub use package_2022_05_01::*;
#[cfg(feature = "package-2022-03-01-preview")]
pub mod package_2022_03_01_preview;
#[cfg(all(feature = "package-2022-03-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_03_01_preview::*;
#[cfg(feature = "package-2021-10-01-preview")]
pub mod package_2021_10_01_preview;
#[cfg(all(feature = "package-2021-10-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_10_01_preview::*;
#[cfg(feature = "package-2021-03-01-preview")]
pub mod package_2021_03_01_preview;
#[cfg(all(feature = "package-2021-03-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_03_01_preview::*;

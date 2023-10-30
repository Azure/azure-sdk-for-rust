#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2024-01")]
pub mod package_2024_01;
#[cfg(all(feature = "package-2024-01", not(feature = "no-default-tag")))]
pub use package_2024_01::*;
#[cfg(feature = "package-2022-08-01")]
pub mod package_2022_08_01;
#[cfg(all(feature = "package-2022-08-01", not(feature = "no-default-tag")))]
pub use package_2022_08_01::*;
#[cfg(feature = "package-2022-07-20-preview")]
pub mod package_2022_07_20_preview;
#[cfg(all(feature = "package-2022-07-20-preview", not(feature = "no-default-tag")))]
pub use package_2022_07_20_preview::*;
#[cfg(feature = "package-2022-04-13-preview")]
pub mod package_2022_04_13_preview;
#[cfg(all(feature = "package-2022-04-13-preview", not(feature = "no-default-tag")))]
pub use package_2022_04_13_preview::*;
#[cfg(feature = "package-2021-11-10-preview")]
pub mod package_2021_11_10_preview;
#[cfg(all(feature = "package-2021-11-10-preview", not(feature = "no-default-tag")))]
pub use package_2021_11_10_preview::*;

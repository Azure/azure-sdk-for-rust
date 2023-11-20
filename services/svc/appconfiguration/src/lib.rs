#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-11-01")]
pub mod package_2023_11_01;
#[cfg(all(feature = "package-2023-11-01", not(feature = "no-default-tag")))]
pub use package_2023_11_01::*;
#[cfg(feature = "package-2023-10-01")]
pub mod package_2023_10_01;
#[cfg(all(feature = "package-2023-10-01", not(feature = "no-default-tag")))]
pub use package_2023_10_01::*;
#[cfg(feature = "package-2023-05-01-preview")]
pub mod package_2023_05_01_preview;
#[cfg(all(feature = "package-2023-05-01-preview", not(feature = "no-default-tag")))]
pub use package_2023_05_01_preview::*;
#[cfg(feature = "package-2022-11-01-preview")]
pub mod package_2022_11_01_preview;
#[cfg(all(feature = "package-2022-11-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_11_01_preview::*;
#[cfg(feature = "package-2019-07")]
pub mod package_2019_07;
#[cfg(all(feature = "package-2019-07", not(feature = "no-default-tag")))]
pub use package_2019_07::*;

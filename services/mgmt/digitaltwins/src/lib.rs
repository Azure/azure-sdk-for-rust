#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-05")]
pub mod package_2022_05;
#[cfg(all(feature = "package-2022-05", not(feature = "no-default-tag")))]
pub use package_2022_05::*;
#[cfg(feature = "package-2021-06-30-preview")]
pub mod package_2021_06_30_preview;
#[cfg(all(feature = "package-2021-06-30-preview", not(feature = "no-default-tag")))]
pub use package_2021_06_30_preview::*;
#[cfg(feature = "package-2020-12")]
pub mod package_2020_12;
#[cfg(all(feature = "package-2020-12", not(feature = "no-default-tag")))]
pub use package_2020_12::*;
#[cfg(feature = "package-2020-10")]
pub mod package_2020_10;
#[cfg(all(feature = "package-2020-10", not(feature = "no-default-tag")))]
pub use package_2020_10::*;
#[cfg(feature = "package-2020-03-01-preview")]
pub mod package_2020_03_01_preview;
#[cfg(all(feature = "package-2020-03-01-preview", not(feature = "no-default-tag")))]
pub use package_2020_03_01_preview::*;

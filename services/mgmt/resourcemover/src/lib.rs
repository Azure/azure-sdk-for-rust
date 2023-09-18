#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-08-01")]
pub mod package_2023_08_01;
#[cfg(all(feature = "package-2023-08-01", not(feature = "no-default-tag")))]
pub use package_2023_08_01::*;
#[cfg(feature = "package-2022-08-01")]
pub mod package_2022_08_01;
#[cfg(all(feature = "package-2022-08-01", not(feature = "no-default-tag")))]
pub use package_2022_08_01::*;
#[cfg(feature = "package-2021-08-01")]
pub mod package_2021_08_01;
#[cfg(all(feature = "package-2021-08-01", not(feature = "no-default-tag")))]
pub use package_2021_08_01::*;
#[cfg(feature = "package-2021-01-01")]
pub mod package_2021_01_01;
#[cfg(all(feature = "package-2021-01-01", not(feature = "no-default-tag")))]
pub use package_2021_01_01::*;
#[cfg(feature = "package-2019-10-01-preview")]
pub mod package_2019_10_01_preview;
#[cfg(all(feature = "package-2019-10-01-preview", not(feature = "no-default-tag")))]
pub use package_2019_10_01_preview::*;

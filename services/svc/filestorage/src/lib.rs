#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-01")]
pub mod package_2023_01;
#[cfg(all(feature = "package-2023-01", not(feature = "no-default-tag")))]
pub use package_2023_01::*;
#[cfg(feature = "package-2022-11")]
pub mod package_2022_11;
#[cfg(all(feature = "package-2022-11", not(feature = "no-default-tag")))]
pub use package_2022_11::*;
#[cfg(feature = "package-2021-12")]
pub mod package_2021_12;
#[cfg(all(feature = "package-2021-12", not(feature = "no-default-tag")))]
pub use package_2021_12::*;
#[cfg(feature = "package-2021-06")]
pub mod package_2021_06;
#[cfg(all(feature = "package-2021-06", not(feature = "no-default-tag")))]
pub use package_2021_06::*;
#[cfg(feature = "package-2021-04")]
pub mod package_2021_04;
#[cfg(all(feature = "package-2021-04", not(feature = "no-default-tag")))]
pub use package_2021_04::*;

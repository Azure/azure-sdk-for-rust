#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-03-01")]
pub mod package_2023_03_01;
#[cfg(all(feature = "package-2023-03-01", not(feature = "no-default-tag")))]
pub use package_2023_03_01::*;
#[cfg(feature = "package-2022-11-01")]
pub mod package_2022_11_01;
#[cfg(all(feature = "package-2022-11-01", not(feature = "no-default-tag")))]
pub use package_2022_11_01::*;
#[cfg(feature = "package-2022-09-04")]
pub mod package_2022_09_04;
#[cfg(all(feature = "package-2022-09-04", not(feature = "no-default-tag")))]
pub use package_2022_09_04::*;
#[cfg(feature = "package-2022-08-01")]
pub mod package_2022_08_01;
#[cfg(all(feature = "package-2022-08-01", not(feature = "no-default-tag")))]
pub use package_2022_08_01::*;
#[cfg(feature = "package-2022-07-02")]
pub mod package_2022_07_02;
#[cfg(all(feature = "package-2022-07-02", not(feature = "no-default-tag")))]
pub use package_2022_07_02::*;

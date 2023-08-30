#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-04")]
pub mod package_2023_04;
#[cfg(all(feature = "package-2023-04", not(feature = "no-default-tag")))]
pub use package_2023_04::*;
#[cfg(feature = "package-2021-04")]
pub mod package_2021_04;
#[cfg(all(feature = "package-2021-04", not(feature = "no-default-tag")))]
pub use package_2021_04::*;
#[cfg(feature = "package-2020-10")]
pub mod package_2020_10;
#[cfg(all(feature = "package-2020-10", not(feature = "no-default-tag")))]
pub use package_2020_10::*;
#[cfg(feature = "package-2020-05")]
pub mod package_2020_05;
#[cfg(all(feature = "package-2020-05", not(feature = "no-default-tag")))]
pub use package_2020_05::*;
#[cfg(feature = "package-2020-02")]
pub mod package_2020_02;
#[cfg(all(feature = "package-2020-02", not(feature = "no-default-tag")))]
pub use package_2020_02::*;

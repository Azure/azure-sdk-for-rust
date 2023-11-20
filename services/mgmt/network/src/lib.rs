#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "profile-hybrid-2020-09-01")]
pub mod profile_hybrid_2020_09_01;
#[cfg(all(feature = "profile-hybrid-2020-09-01", not(feature = "no-default-tag")))]
pub use profile_hybrid_2020_09_01::*;
#[cfg(feature = "package-2023-05")]
pub mod package_2023_05;
#[cfg(all(feature = "package-2023-05", not(feature = "no-default-tag")))]
pub use package_2023_05::*;
#[cfg(feature = "package-2023-04")]
pub mod package_2023_04;
#[cfg(all(feature = "package-2023-04", not(feature = "no-default-tag")))]
pub use package_2023_04::*;
#[cfg(feature = "package-2023-02")]
pub mod package_2023_02;
#[cfg(all(feature = "package-2023-02", not(feature = "no-default-tag")))]
pub use package_2023_02::*;
#[cfg(feature = "package-2022-11")]
pub mod package_2022_11;
#[cfg(all(feature = "package-2022-11", not(feature = "no-default-tag")))]
pub use package_2022_11::*;

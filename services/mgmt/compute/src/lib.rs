#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-07-01")]
pub mod package_2023_07_01;
#[cfg(all(feature = "package-2023-07-01", not(feature = "without_tag_import")))]
pub use package_2023_07_01::*;
#[cfg(feature = "package-2023-04-02")]
pub mod package_2023_04_02;
#[cfg(all(feature = "package-2023-04-02", not(feature = "without_tag_import")))]
pub use package_2023_04_02::*;
#[cfg(feature = "package-2023-03-01")]
pub mod package_2023_03_01;
#[cfg(all(feature = "package-2023-03-01", not(feature = "without_tag_import")))]
pub use package_2023_03_01::*;
#[cfg(feature = "package-2023-01-02")]
pub mod package_2023_01_02;
#[cfg(all(feature = "package-2023-01-02", not(feature = "without_tag_import")))]
pub use package_2023_01_02::*;
#[cfg(feature = "package-2022-11-01")]
pub mod package_2022_11_01;
#[cfg(all(feature = "package-2022-11-01", not(feature = "without_tag_import")))]
pub use package_2022_11_01::*;

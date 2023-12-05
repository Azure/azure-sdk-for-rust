#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-09")]
pub mod package_2022_09;
#[cfg(all(feature = "package-2022-09", not(feature = "without_tag_import")))]
pub use package_2022_09::*;
#[cfg(feature = "package-2022-06-01")]
pub mod package_2022_06_01;
#[cfg(all(feature = "package-2022-06-01", not(feature = "without_tag_import")))]
pub use package_2022_06_01::*;
#[cfg(feature = "package-2020-09-01")]
pub mod package_2020_09_01;
#[cfg(all(feature = "package-2020-09-01", not(feature = "without_tag_import")))]
pub use package_2020_09_01::*;
#[cfg(feature = "package-2020-03-01")]
pub mod package_2020_03_01;
#[cfg(all(feature = "package-2020-03-01", not(feature = "without_tag_import")))]
pub use package_2020_03_01::*;
#[cfg(feature = "package-2019-10-01")]
pub mod package_2019_10_01;
#[cfg(all(feature = "package-2019-10-01", not(feature = "without_tag_import")))]
pub use package_2019_10_01::*;

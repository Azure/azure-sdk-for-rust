#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-10-01")]
pub mod package_2022_10_01;
#[cfg(all(feature = "package-2022-10-01", not(feature = "without_tag_import")))]
pub use package_2022_10_01::*;
#[cfg(feature = "package-2022-06-01")]
pub mod package_2022_06_01;
#[cfg(all(feature = "package-2022-06-01", not(feature = "without_tag_import")))]
pub use package_2022_06_01::*;
#[cfg(feature = "package-2022-01-01")]
pub mod package_2022_01_01;
#[cfg(all(feature = "package-2022-01-01", not(feature = "without_tag_import")))]
pub use package_2022_01_01::*;
#[cfg(feature = "package-2021-06-01")]
pub mod package_2021_06_01;
#[cfg(all(feature = "package-2021-06-01", not(feature = "without_tag_import")))]
pub use package_2021_06_01::*;
#[cfg(feature = "package-2021-01-01")]
pub mod package_2021_01_01;
#[cfg(all(feature = "package-2021-01-01", not(feature = "without_tag_import")))]
pub use package_2021_01_01::*;

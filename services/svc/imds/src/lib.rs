#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-12-13")]
pub mod package_2021_12_13;
#[cfg(all(feature = "package-2021-12-13", not(feature = "without_tag_import")))]
pub use package_2021_12_13::*;
#[cfg(feature = "package-2021-11-15")]
pub mod package_2021_11_15;
#[cfg(all(feature = "package-2021-11-15", not(feature = "without_tag_import")))]
pub use package_2021_11_15::*;
#[cfg(feature = "package-2021-11-01")]
pub mod package_2021_11_01;
#[cfg(all(feature = "package-2021-11-01", not(feature = "without_tag_import")))]
pub use package_2021_11_01::*;
#[cfg(feature = "package-2021-10-01")]
pub mod package_2021_10_01;
#[cfg(all(feature = "package-2021-10-01", not(feature = "without_tag_import")))]
pub use package_2021_10_01::*;
#[cfg(feature = "package-2021-05-01")]
pub mod package_2021_05_01;
#[cfg(all(feature = "package-2021-05-01", not(feature = "without_tag_import")))]
pub use package_2021_05_01::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2022-12")]
pub mod package_2022_12;
#[cfg(all(feature = "package-2022-12", not(feature = "without_tag_import")))]
pub use package_2022_12::*;
#[cfg(feature = "package-2022-09")]
pub mod package_2022_09;
#[cfg(all(feature = "package-2022-09", not(feature = "without_tag_import")))]
pub use package_2022_09::*;
#[cfg(feature = "package-2021-05")]
pub mod package_2021_05;
#[cfg(all(feature = "package-2021-05", not(feature = "without_tag_import")))]
pub use package_2021_05::*;
#[cfg(feature = "package-2021-03")]
pub mod package_2021_03;
#[cfg(all(feature = "package-2021-03", not(feature = "without_tag_import")))]
pub use package_2021_03::*;
#[cfg(feature = "package-2020-01")]
pub mod package_2020_01;
#[cfg(all(feature = "package-2020-01", not(feature = "without_tag_import")))]
pub use package_2020_01::*;

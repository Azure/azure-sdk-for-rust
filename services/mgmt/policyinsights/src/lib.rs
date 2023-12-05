#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-03")]
pub mod package_2023_03;
#[cfg(all(feature = "package-2023-03", not(feature = "without_tag_import")))]
pub use package_2023_03::*;
#[cfg(feature = "package-2022-09")]
pub mod package_2022_09;
#[cfg(all(feature = "package-2022-09", not(feature = "without_tag_import")))]
pub use package_2022_09::*;
#[cfg(feature = "package-2022-04")]
pub mod package_2022_04;
#[cfg(all(feature = "package-2022-04", not(feature = "without_tag_import")))]
pub use package_2022_04::*;
#[cfg(feature = "package-2022-03")]
pub mod package_2022_03;
#[cfg(all(feature = "package-2022-03", not(feature = "without_tag_import")))]
pub use package_2022_03::*;
#[cfg(feature = "package-2021-10")]
pub mod package_2021_10;
#[cfg(all(feature = "package-2021-10", not(feature = "without_tag_import")))]
pub use package_2021_10::*;

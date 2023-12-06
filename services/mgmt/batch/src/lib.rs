#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2023-05")]
pub mod package_2023_05;
#[cfg(all(feature = "package-2023-05", not(feature = "without_tag_import")))]
pub use package_2023_05::*;
#[cfg(feature = "package-2022-10")]
pub mod package_2022_10;
#[cfg(all(feature = "package-2022-10", not(feature = "without_tag_import")))]
pub use package_2022_10::*;
#[cfg(feature = "package-2022-06")]
pub mod package_2022_06;
#[cfg(all(feature = "package-2022-06", not(feature = "without_tag_import")))]
pub use package_2022_06::*;
#[cfg(feature = "package-2022-01")]
pub mod package_2022_01;
#[cfg(all(feature = "package-2022-01", not(feature = "without_tag_import")))]
pub use package_2022_01::*;
#[cfg(feature = "package-2021-06")]
pub mod package_2021_06;
#[cfg(all(feature = "package-2021-06", not(feature = "without_tag_import")))]
pub use package_2021_06::*;

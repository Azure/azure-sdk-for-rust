#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2022-05")]
pub mod package_2022_05;
#[cfg(all(feature = "package-2022-05", not(feature = "without_tag_import")))]
pub use package_2022_05::*;
#[cfg(feature = "package-2021-06")]
pub mod package_2021_06;
#[cfg(all(feature = "package-2021-06", not(feature = "without_tag_import")))]
pub use package_2021_06::*;
#[cfg(feature = "package-2020-11")]
pub mod package_2020_11;
#[cfg(all(feature = "package-2020-11", not(feature = "without_tag_import")))]
pub use package_2020_11::*;
#[cfg(feature = "package-2020-05")]
pub mod package_2020_05;
#[cfg(all(feature = "package-2020-05", not(feature = "without_tag_import")))]
pub use package_2020_05::*;
#[cfg(feature = "package-2020-04")]
pub mod package_2020_04;
#[cfg(all(feature = "package-2020-04", not(feature = "without_tag_import")))]
pub use package_2020_04::*;

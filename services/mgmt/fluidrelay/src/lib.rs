#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2022-06-01")]
pub mod package_2022_06_01;
#[cfg(all(feature = "package-2022-06-01", not(feature = "without_tag_import")))]
pub use package_2022_06_01::*;
#[cfg(feature = "package-2022-05-26")]
pub mod package_2022_05_26;
#[cfg(all(feature = "package-2022-05-26", not(feature = "without_tag_import")))]
pub use package_2022_05_26::*;
#[cfg(feature = "package-2022-05-11")]
pub mod package_2022_05_11;
#[cfg(all(feature = "package-2022-05-11", not(feature = "without_tag_import")))]
pub use package_2022_05_11::*;
#[cfg(feature = "package-2022-04-21")]
pub mod package_2022_04_21;
#[cfg(all(feature = "package-2022-04-21", not(feature = "without_tag_import")))]
pub use package_2022_04_21::*;
#[cfg(feature = "package-2022-02-15")]
pub mod package_2022_02_15;
#[cfg(all(feature = "package-2022-02-15", not(feature = "without_tag_import")))]
pub use package_2022_02_15::*;

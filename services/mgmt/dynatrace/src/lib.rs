#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2023-04-27")]
pub mod package_2023_04_27;
#[cfg(all(feature = "package-2023-04-27", not(feature = "without_tag_import")))]
pub use package_2023_04_27::*;
#[cfg(feature = "package-2021-09-01-preview")]
pub mod package_2021_09_01_preview;
#[cfg(all(feature = "package-2021-09-01-preview", not(feature = "without_tag_import")))]
pub use package_2021_09_01_preview::*;
#[cfg(feature = "package-2021-09-01")]
pub mod package_2021_09_01;
#[cfg(all(feature = "package-2021-09-01", not(feature = "without_tag_import")))]
pub use package_2021_09_01::*;

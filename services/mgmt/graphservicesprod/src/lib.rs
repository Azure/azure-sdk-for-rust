#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2023-04-13")]
pub mod package_2023_04_13;
#[cfg(all(feature = "package-2023-04-13", not(feature = "without_tag_import")))]
pub use package_2023_04_13::*;
#[cfg(feature = "package-2022-09-22-preview")]
pub mod package_2022_09_22_preview;
#[cfg(all(feature = "package-2022-09-22-preview", not(feature = "without_tag_import")))]
pub use package_2022_09_22_preview::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-07")]
pub mod package_2022_07;
#[cfg(all(feature = "package-2022-07", not(feature = "without_tag_import")))]
pub use package_2022_07::*;
#[cfg(feature = "package-2020-04-preview")]
pub mod package_2020_04_preview;
#[cfg(all(feature = "package-2020-04-preview", not(feature = "without_tag_import")))]
pub use package_2020_04_preview::*;

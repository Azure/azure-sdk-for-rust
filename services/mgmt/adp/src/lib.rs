#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2021-11-01-preview")]
pub mod package_2021_11_01_preview;
#[cfg(all(feature = "package-2021-11-01-preview", not(feature = "without_tag_import")))]
pub use package_2021_11_01_preview::*;
#[cfg(feature = "package-2021-02-01-preview")]
pub mod package_2021_02_01_preview;
#[cfg(all(feature = "package-2021-02-01-preview", not(feature = "without_tag_import")))]
pub use package_2021_02_01_preview::*;
#[cfg(feature = "package-2020-07-01-preview")]
pub mod package_2020_07_01_preview;
#[cfg(all(feature = "package-2020-07-01-preview", not(feature = "without_tag_import")))]
pub use package_2020_07_01_preview::*;

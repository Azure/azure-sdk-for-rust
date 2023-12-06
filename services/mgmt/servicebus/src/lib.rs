#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2022-10-preview")]
pub mod package_2022_10_preview;
#[cfg(all(feature = "package-2022-10-preview", not(feature = "without_tag_import")))]
pub use package_2022_10_preview::*;
#[cfg(feature = "package-2022-01-preview")]
pub mod package_2022_01_preview;
#[cfg(all(feature = "package-2022-01-preview", not(feature = "without_tag_import")))]
pub use package_2022_01_preview::*;
#[cfg(feature = "package-2021-11")]
pub mod package_2021_11;
#[cfg(all(feature = "package-2021-11", not(feature = "without_tag_import")))]
pub use package_2021_11::*;
#[cfg(feature = "package-2021-06-preview")]
pub mod package_2021_06_preview;
#[cfg(all(feature = "package-2021-06-preview", not(feature = "without_tag_import")))]
pub use package_2021_06_preview::*;
#[cfg(feature = "package-2021-01-preview")]
pub mod package_2021_01_preview;
#[cfg(all(feature = "package-2021-01-preview", not(feature = "without_tag_import")))]
pub use package_2021_01_preview::*;

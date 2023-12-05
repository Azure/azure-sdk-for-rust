#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-03")]
pub mod package_2023_03;
#[cfg(all(feature = "package-2023-03", not(feature = "without_tag_import")))]
pub use package_2023_03::*;
#[cfg(feature = "package-2022-05-01-preview")]
pub mod package_2022_05_01_preview;
#[cfg(all(feature = "package-2022-05-01-preview", not(feature = "without_tag_import")))]
pub use package_2022_05_01_preview::*;
#[cfg(feature = "package-2021-10-06-preview")]
pub mod package_2021_10_06_preview;
#[cfg(all(feature = "package-2021-10-06-preview", not(feature = "without_tag_import")))]
pub use package_2021_10_06_preview::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2023-11")]
pub mod package_preview_2023_11;
#[cfg(all(feature = "package-preview-2023-11", not(feature = "without_tag_import")))]
pub use package_preview_2023_11::*;
#[cfg(feature = "package-2022-10-01-preview")]
pub mod package_2022_10_01_preview;
#[cfg(all(feature = "package-2022-10-01-preview", not(feature = "without_tag_import")))]
pub use package_2022_10_01_preview::*;
#[cfg(feature = "package-2022-05-01-preview")]
pub mod package_2022_05_01_preview;
#[cfg(all(feature = "package-2022-05-01-preview", not(feature = "without_tag_import")))]
pub use package_2022_05_01_preview::*;
#[cfg(feature = "package-2021-10-01")]
pub mod package_2021_10_01;
#[cfg(all(feature = "package-2021-10-01", not(feature = "without_tag_import")))]
pub use package_2021_10_01::*;
#[cfg(feature = "package-2021-04-01-preview")]
pub mod package_2021_04_01_preview;
#[cfg(all(feature = "package-2021-04-01-preview", not(feature = "without_tag_import")))]
pub use package_2021_04_01_preview::*;

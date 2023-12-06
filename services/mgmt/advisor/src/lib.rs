#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2023-01")]
pub mod package_2023_01;
#[cfg(all(feature = "package-2023-01", not(feature = "without_tag_import")))]
pub use package_2023_01::*;
#[cfg(feature = "package-2022-10")]
pub mod package_2022_10;
#[cfg(all(feature = "package-2022-10", not(feature = "without_tag_import")))]
pub use package_2022_10::*;
#[cfg(feature = "package-2022-09")]
pub mod package_2022_09;
#[cfg(all(feature = "package-2022-09", not(feature = "without_tag_import")))]
pub use package_2022_09::*;
#[cfg(feature = "package-2022-02-preview")]
pub mod package_2022_02_preview;
#[cfg(all(feature = "package-2022-02-preview", not(feature = "without_tag_import")))]
pub use package_2022_02_preview::*;
#[cfg(feature = "package-2020-07-preview")]
pub mod package_2020_07_preview;
#[cfg(all(feature = "package-2020-07-preview", not(feature = "without_tag_import")))]
pub use package_2020_07_preview::*;

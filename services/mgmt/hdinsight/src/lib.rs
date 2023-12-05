#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-08-preview")]
pub mod package_2023_08_preview;
#[cfg(all(feature = "package-2023-08-preview", not(feature = "without_tag_import")))]
pub use package_2023_08_preview::*;
#[cfg(feature = "package-2023-04-preview")]
pub mod package_2023_04_preview;
#[cfg(all(feature = "package-2023-04-preview", not(feature = "without_tag_import")))]
pub use package_2023_04_preview::*;
#[cfg(feature = "package-2021-06")]
pub mod package_2021_06;
#[cfg(all(feature = "package-2021-06", not(feature = "without_tag_import")))]
pub use package_2021_06::*;
#[cfg(feature = "package-2018-06-preview")]
pub mod package_2018_06_preview;
#[cfg(all(feature = "package-2018-06-preview", not(feature = "without_tag_import")))]
pub use package_2018_06_preview::*;
#[cfg(feature = "package-2015-03-preview")]
pub mod package_2015_03_preview;
#[cfg(all(feature = "package-2015-03-preview", not(feature = "without_tag_import")))]
pub use package_2015_03_preview::*;

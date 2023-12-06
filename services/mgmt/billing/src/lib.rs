#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-10")]
pub mod package_2021_10;
#[cfg(all(feature = "package-2021-10", not(feature = "without_tag_import")))]
pub use package_2021_10::*;
#[cfg(feature = "package-2020-11-preview")]
pub mod package_2020_11_preview;
#[cfg(all(feature = "package-2020-11-preview", not(feature = "without_tag_import")))]
pub use package_2020_11_preview::*;
#[cfg(feature = "package-2020-09-preview")]
pub mod package_2020_09_preview;
#[cfg(all(feature = "package-2020-09-preview", not(feature = "without_tag_import")))]
pub use package_2020_09_preview::*;
#[cfg(feature = "package-2020-05")]
pub mod package_2020_05;
#[cfg(all(feature = "package-2020-05", not(feature = "without_tag_import")))]
pub use package_2020_05::*;
#[cfg(feature = "package-2019-10-preview")]
pub mod package_2019_10_preview;
#[cfg(all(feature = "package-2019-10-preview", not(feature = "without_tag_import")))]
pub use package_2019_10_preview::*;

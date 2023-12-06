#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2021-01-01-preview")]
pub mod package_2021_01_01_preview;
#[cfg(all(feature = "package-2021-01-01-preview", not(feature = "without_tag_import")))]
pub use package_2021_01_01_preview::*;
#[cfg(feature = "package-2021-01-01")]
pub mod package_2021_01_01;
#[cfg(all(feature = "package-2021-01-01", not(feature = "without_tag_import")))]
pub use package_2021_01_01::*;
#[cfg(feature = "package-2019-02-28-preview")]
pub mod package_2019_02_28_preview;
#[cfg(all(feature = "package-2019-02-28-preview", not(feature = "without_tag_import")))]
pub use package_2019_02_28_preview::*;
#[cfg(feature = "package-0_3-preview_2")]
pub mod package_0_3_preview_2;
#[cfg(all(feature = "package-0_3-preview_2", not(feature = "without_tag_import")))]
pub use package_0_3_preview_2::*;
#[cfg(feature = "package-0_3-preview_1")]
pub mod package_0_3_preview_1;
#[cfg(all(feature = "package-0_3-preview_1", not(feature = "without_tag_import")))]
pub use package_0_3_preview_1::*;

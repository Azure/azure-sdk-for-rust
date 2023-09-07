#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2023-02-27")]
pub mod package_preview_2023_02_27;
#[cfg(all(feature = "package-preview-2023-02-27", not(feature = "no-default-tag")))]
pub use package_preview_2023_02_27::*;
#[cfg(feature = "package-2023-06-30")]
pub mod package_2023_06_30;
#[cfg(all(feature = "package-2023-06-30", not(feature = "no-default-tag")))]
pub use package_2023_06_30::*;
#[cfg(feature = "package-2022-05-31")]
pub mod package_2022_05_31;
#[cfg(all(feature = "package-2022-05-31", not(feature = "no-default-tag")))]
pub use package_2022_05_31::*;
#[cfg(feature = "package-2021-06-30-preview")]
pub mod package_2021_06_30_preview;
#[cfg(all(feature = "package-2021-06-30-preview", not(feature = "no-default-tag")))]
pub use package_2021_06_30_preview::*;
#[cfg(feature = "package-2020-10-31")]
pub mod package_2020_10_31;
#[cfg(all(feature = "package-2020-10-31", not(feature = "no-default-tag")))]
pub use package_2020_10_31::*;

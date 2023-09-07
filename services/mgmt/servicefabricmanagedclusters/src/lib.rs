#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-03-preview")]
pub mod package_2023_03_preview;
#[cfg(all(feature = "package-2023-03-preview", not(feature = "no-default-tag")))]
pub use package_2023_03_preview::*;
#[cfg(feature = "package-2023-02-preview")]
pub mod package_2023_02_preview;
#[cfg(all(feature = "package-2023-02-preview", not(feature = "no-default-tag")))]
pub use package_2023_02_preview::*;
#[cfg(feature = "package-2022-10-preview")]
pub mod package_2022_10_preview;
#[cfg(all(feature = "package-2022-10-preview", not(feature = "no-default-tag")))]
pub use package_2022_10_preview::*;
#[cfg(feature = "package-2022-08-preview")]
pub mod package_2022_08_preview;
#[cfg(all(feature = "package-2022-08-preview", not(feature = "no-default-tag")))]
pub use package_2022_08_preview::*;
#[cfg(feature = "package-2022-06-preview")]
pub mod package_2022_06_preview;
#[cfg(all(feature = "package-2022-06-preview", not(feature = "no-default-tag")))]
pub use package_2022_06_preview::*;

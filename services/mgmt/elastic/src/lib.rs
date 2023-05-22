#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-05-01-preview")]
pub mod package_2023_05_01_preview;
#[cfg(all(feature = "package-2023-05-01-preview", not(feature = "no-default-tag")))]
pub use package_2023_05_01_preview::*;
#[cfg(feature = "package-2023-02-01-preview")]
pub mod package_2023_02_01_preview;
#[cfg(all(feature = "package-2023-02-01-preview", not(feature = "no-default-tag")))]
pub use package_2023_02_01_preview::*;
#[cfg(feature = "package-2022-09-01-preview")]
pub mod package_2022_09_01_preview;
#[cfg(all(feature = "package-2022-09-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_09_01_preview::*;
#[cfg(feature = "package-2022-07-01-preview")]
pub mod package_2022_07_01_preview;
#[cfg(all(feature = "package-2022-07-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_07_01_preview::*;
#[cfg(feature = "package-2022-05-05-preview")]
pub mod package_2022_05_05_preview;
#[cfg(all(feature = "package-2022-05-05-preview", not(feature = "no-default-tag")))]
pub use package_2022_05_05_preview::*;

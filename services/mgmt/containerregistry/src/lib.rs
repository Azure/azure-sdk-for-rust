#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-08-preview")]
pub mod package_2023_08_preview;
#[cfg(all(feature = "package-2023-08-preview", not(feature = "no-default-tag")))]
pub use package_2023_08_preview::*;
#[cfg(feature = "package-2023-07")]
pub mod package_2023_07;
#[cfg(all(feature = "package-2023-07", not(feature = "no-default-tag")))]
pub use package_2023_07::*;
#[cfg(feature = "package-2023-06-preview")]
pub mod package_2023_06_preview;
#[cfg(all(feature = "package-2023-06-preview", not(feature = "no-default-tag")))]
pub use package_2023_06_preview::*;
#[cfg(feature = "package-2023-01-preview")]
pub mod package_2023_01_preview;
#[cfg(all(feature = "package-2023-01-preview", not(feature = "no-default-tag")))]
pub use package_2023_01_preview::*;
#[cfg(feature = "package-2022-12")]
pub mod package_2022_12;
#[cfg(all(feature = "package-2022-12", not(feature = "no-default-tag")))]
pub use package_2022_12::*;

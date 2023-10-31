#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-10-10-preview")]
pub mod package_2023_10_10_preview;
#[cfg(all(feature = "package-2023-10-10-preview", not(feature = "no-default-tag")))]
pub use package_2023_10_10_preview::*;
#[cfg(feature = "package-2023-09-01-preview")]
pub mod package_2023_09_01_preview;
#[cfg(all(feature = "package-2023-09-01-preview", not(feature = "no-default-tag")))]
pub use package_2023_09_01_preview::*;
#[cfg(feature = "package-2023-09-01")]
pub mod package_2023_09_01;
#[cfg(all(feature = "package-2023-09-01", not(feature = "no-default-tag")))]
pub use package_2023_09_01::*;
#[cfg(feature = "package-2022-08-29-preview")]
pub mod package_2022_08_29_preview;
#[cfg(all(feature = "package-2022-08-29-preview", not(feature = "no-default-tag")))]
pub use package_2022_08_29_preview::*;
#[cfg(feature = "package-2022-08-29")]
pub mod package_2022_08_29;
#[cfg(all(feature = "package-2022-08-29", not(feature = "no-default-tag")))]
pub use package_2022_08_29::*;

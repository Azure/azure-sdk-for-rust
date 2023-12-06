#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2023-11-01")]
pub mod package_2023_11_01;
#[cfg(all(feature = "package-2023-11-01", not(feature = "without_tag_import")))]
pub use package_2023_11_01::*;
#[cfg(feature = "package-2023-05-15-preview")]
pub mod package_2023_05_15_preview;
#[cfg(all(feature = "package-2023-05-15-preview", not(feature = "without_tag_import")))]
pub use package_2023_05_15_preview::*;
#[cfg(feature = "package-2022-08-08")]
pub mod package_2022_08_08;
#[cfg(all(feature = "package-2022-08-08", not(feature = "without_tag_import")))]
pub use package_2022_08_08::*;
#[cfg(feature = "package-2022-02-22")]
pub mod package_2022_02_22;
#[cfg(all(feature = "package-2022-02-22", not(feature = "without_tag_import")))]
pub use package_2022_02_22::*;
#[cfg(feature = "package-2022-01-31")]
pub mod package_2022_01_31;
#[cfg(all(feature = "package-2022-01-31", not(feature = "without_tag_import")))]
pub use package_2022_01_31::*;

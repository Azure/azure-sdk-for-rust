#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-encoding-2022-07")]
pub mod package_encoding_2022_07;
#[cfg(all(feature = "package-encoding-2022-07", not(feature = "no-default-tag")))]
pub use package_encoding_2022_07::*;
#[cfg(feature = "package-encoding-2022-05-preview")]
pub mod package_encoding_2022_05_preview;
#[cfg(all(feature = "package-encoding-2022-05-preview", not(feature = "no-default-tag")))]
pub use package_encoding_2022_05_preview::*;
#[cfg(feature = "package-2022-08")]
pub mod package_2022_08;
#[cfg(all(feature = "package-2022-08", not(feature = "no-default-tag")))]
pub use package_2022_08::*;
#[cfg(feature = "package-2021-11")]
pub mod package_2021_11;
#[cfg(all(feature = "package-2021-11", not(feature = "no-default-tag")))]
pub use package_2021_11::*;
#[cfg(feature = "package-2021-06")]
pub mod package_2021_06;
#[cfg(all(feature = "package-2021-06", not(feature = "no-default-tag")))]
pub use package_2021_06::*;

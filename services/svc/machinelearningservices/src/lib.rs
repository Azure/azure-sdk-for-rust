#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2019-09-preview")]
pub mod package_2019_09_preview;
#[cfg(all(feature = "package-2019-09-preview", not(feature = "no-default-tag")))]
pub use package_2019_09_preview::*;
#[cfg(feature = "package-2019-08-preview")]
pub mod package_2019_08_preview;
#[cfg(all(feature = "package-2019-08-preview", not(feature = "no-default-tag")))]
pub use package_2019_08_preview::*;

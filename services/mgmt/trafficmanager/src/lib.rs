#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2018-08")]
pub mod package_2018_08;
#[cfg(all(feature = "package-2018-08", not(feature = "no-default-tag")))]
pub use package_2018_08::*;
#[cfg(feature = "package-2018-04")]
pub mod package_2018_04;
#[cfg(all(feature = "package-2018-04", not(feature = "no-default-tag")))]
pub use package_2018_04::*;
#[cfg(feature = "package-2018-03")]
pub mod package_2018_03;
#[cfg(all(feature = "package-2018-03", not(feature = "no-default-tag")))]
pub use package_2018_03::*;
#[cfg(feature = "package-2018-02")]
pub mod package_2018_02;
#[cfg(all(feature = "package-2018-02", not(feature = "no-default-tag")))]
pub use package_2018_02::*;
#[cfg(feature = "package-2017-09-preview")]
pub mod package_2017_09_preview;
#[cfg(all(feature = "package-2017-09-preview", not(feature = "no-default-tag")))]
pub use package_2017_09_preview::*;

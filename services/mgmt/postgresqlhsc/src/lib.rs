#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-11-08")]
pub mod package_2022_11_08;
#[cfg(all(feature = "package-2022-11-08", not(feature = "without_tag_import")))]
pub use package_2022_11_08::*;
#[cfg(feature = "package-2020-10-05-privatepreview")]
pub mod package_2020_10_05_privatepreview;
#[cfg(all(feature = "package-2020-10-05-privatepreview", not(feature = "without_tag_import")))]
pub use package_2020_10_05_privatepreview::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2019-08")]
pub mod package_2019_08;
#[cfg(all(feature = "package-2019-08", not(feature = "without_tag_import")))]
pub use package_2019_08::*;
#[cfg(feature = "package-2019-07")]
pub mod package_2019_07;
#[cfg(all(feature = "package-2019-07", not(feature = "without_tag_import")))]
pub use package_2019_07::*;
#[cfg(feature = "package-2018-08")]
pub mod package_2018_08;
#[cfg(all(feature = "package-2018-08", not(feature = "without_tag_import")))]
pub use package_2018_08::*;

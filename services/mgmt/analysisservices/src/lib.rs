#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2017-08-beta")]
pub mod package_2017_08_beta;
#[cfg(all(feature = "package-2017-08-beta", not(feature = "without_tag_import")))]
pub use package_2017_08_beta::*;
#[cfg(feature = "package-2017-07")]
pub mod package_2017_07;
#[cfg(all(feature = "package-2017-07", not(feature = "without_tag_import")))]
pub use package_2017_07::*;
#[cfg(feature = "package-2016-05")]
pub mod package_2016_05;
#[cfg(all(feature = "package-2016-05", not(feature = "without_tag_import")))]
pub use package_2016_05::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "profile-hybrid-2020-09-01")]
pub mod profile_hybrid_2020_09_01;
#[cfg(all(feature = "profile-hybrid-2020-09-01", not(feature = "no-default-tag")))]
pub use profile_hybrid_2020_09_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2015-06-preview")]
pub mod package_2015_06_preview;
#[cfg(all(feature = "package-2015-06-preview", not(feature = "no-default-tag")))]
pub use package_2015_06_preview::{models, Client, ClientBuilder};

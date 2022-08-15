#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2019-07")]
pub mod package_2019_07;
#[cfg(all(feature = "package-2019-07", not(feature = "no-default-tag")))]
pub use package_2019_07::{models, Client, ClientBuilder};

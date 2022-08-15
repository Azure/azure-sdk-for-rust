#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-08-09")]
pub mod package_2021_08_09;
#[cfg(all(feature = "package-2021-08-09", not(feature = "no-default-tag")))]
pub use package_2021_08_09::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-08-06-preview")]
pub mod package_2020_08_06_preview;
#[cfg(all(feature = "package-2020-08-06-preview", not(feature = "no-default-tag")))]
pub use package_2020_08_06_preview::{models, Client, ClientBuilder};

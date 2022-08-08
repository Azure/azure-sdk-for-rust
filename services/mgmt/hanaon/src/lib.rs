#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2020-02-07-preview")]
pub mod package_2020_02_07_preview;
#[cfg(all(feature = "package-2020-02-07-preview", not(feature = "no-default-tag")))]
pub use package_2020_02_07_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2017-11")]
pub mod package_2017_11;
#[cfg(all(feature = "package-2017-11", not(feature = "no-default-tag")))]
pub use package_2017_11::{models, Client, ClientBuilder};

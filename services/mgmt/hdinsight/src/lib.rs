#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2021-06")]
pub mod package_2021_06;
#[cfg(all(feature = "package-2021-06", not(feature = "no-default-tag")))]
pub use package_2021_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-06-preview")]
pub mod package_2018_06_preview;
#[cfg(all(feature = "package-2018-06-preview", not(feature = "no-default-tag")))]
pub use package_2018_06_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2015-03-preview")]
pub mod package_2015_03_preview;
#[cfg(all(feature = "package-2015-03-preview", not(feature = "no-default-tag")))]
pub use package_2015_03_preview::{models, Client, ClientBuilder};

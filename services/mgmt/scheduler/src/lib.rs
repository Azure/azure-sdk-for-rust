#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2016-03")]
pub mod package_2016_03;
#[cfg(all(feature = "package-2016-03", not(feature = "no-default-tag")))]
pub use package_2016_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-2016-01")]
pub mod package_2016_01;
#[cfg(all(feature = "package-2016-01", not(feature = "no-default-tag")))]
pub use package_2016_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2014-08-preview")]
pub mod package_2014_08_preview;
#[cfg(all(feature = "package-2014-08-preview", not(feature = "no-default-tag")))]
pub use package_2014_08_preview::{models, Client, ClientBuilder};

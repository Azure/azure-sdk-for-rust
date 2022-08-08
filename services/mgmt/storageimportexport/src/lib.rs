#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-preview-2021-01")]
pub mod package_preview_2021_01;
#[cfg(all(feature = "package-preview-2021-01", not(feature = "no-default-tag")))]
pub use package_preview_2021_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-08")]
pub mod package_2020_08;
#[cfg(all(feature = "package-2020-08", not(feature = "no-default-tag")))]
pub use package_2020_08::{models, Client, ClientBuilder};
#[cfg(feature = "package-2016-11")]
pub mod package_2016_11;
#[cfg(all(feature = "package-2016-11", not(feature = "no-default-tag")))]
pub use package_2016_11::{models, Client, ClientBuilder};

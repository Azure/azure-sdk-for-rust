#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-preview-2019-07")]
pub mod package_preview_2019_07;
#[cfg(all(feature = "package-preview-2019-07", not(feature = "no-default-tag")))]
pub use package_preview_2019_07::{models, Client, ClientBuilder};
#[cfg(feature = "package-2017-03-01-preview")]
pub mod package_2017_03_01_preview;
#[cfg(all(feature = "package-2017-03-01-preview", not(feature = "no-default-tag")))]
pub use package_2017_03_01_preview::{models, Client, ClientBuilder};

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2020-05-12-preview")]
pub mod package_2020_05_12_preview;
#[cfg(all(feature = "package-2020-05-12-preview", not(feature = "no-default-tag")))]
pub use package_2020_05_12_preview::{models, Client, ClientBuilder};

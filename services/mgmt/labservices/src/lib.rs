#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-preview-2021-11")]
pub mod package_preview_2021_11;
#[cfg(all(feature = "package-preview-2021-11", not(feature = "no-default-tag")))]
pub use package_preview_2021_11::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2021-10")]
pub mod package_preview_2021_10;
#[cfg(all(feature = "package-preview-2021-10", not(feature = "no-default-tag")))]
pub use package_preview_2021_10::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-10")]
pub mod package_2018_10;
#[cfg(all(feature = "package-2018-10", not(feature = "no-default-tag")))]
pub use package_2018_10::{models, Client, ClientBuilder};

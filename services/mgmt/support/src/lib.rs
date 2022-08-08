#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-preview-2021-06")]
pub mod package_preview_2021_06;
#[cfg(all(feature = "package-preview-2021-06", not(feature = "no-default-tag")))]
pub use package_preview_2021_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-04")]
pub mod package_2020_04;
#[cfg(all(feature = "package-2020-04", not(feature = "no-default-tag")))]
pub use package_2020_04::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-05-preview")]
pub mod package_2019_05_preview;
#[cfg(all(feature = "package-2019-05-preview", not(feature = "no-default-tag")))]
pub use package_2019_05_preview::{models, Client, ClientBuilder};

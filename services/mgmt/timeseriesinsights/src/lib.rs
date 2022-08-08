#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-preview-2021-06")]
pub mod package_preview_2021_06;
#[cfg(all(feature = "package-preview-2021-06", not(feature = "no-default-tag")))]
pub use package_preview_2021_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2021-03")]
pub mod package_preview_2021_03;
#[cfg(all(feature = "package-preview-2021-03", not(feature = "no-default-tag")))]
pub use package_preview_2021_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-05-15")]
pub mod package_2020_05_15;
#[cfg(all(feature = "package-2020-05-15", not(feature = "no-default-tag")))]
pub use package_2020_05_15::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-08-preview")]
pub mod package_2018_08_preview;
#[cfg(all(feature = "package-2018-08-preview", not(feature = "no-default-tag")))]
pub use package_2018_08_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2017-11-15")]
pub mod package_2017_11_15;
#[cfg(all(feature = "package-2017-11-15", not(feature = "no-default-tag")))]
pub use package_2017_11_15::{models, Client, ClientBuilder};

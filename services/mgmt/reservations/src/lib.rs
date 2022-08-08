#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-preview-2019-07-19")]
pub mod package_preview_2019_07_19;
#[cfg(all(feature = "package-preview-2019-07-19", not(feature = "no-default-tag")))]
pub use package_preview_2019_07_19::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2019-04")]
pub mod package_preview_2019_04;
#[cfg(all(feature = "package-preview-2019-04", not(feature = "no-default-tag")))]
pub use package_preview_2019_04::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-03")]
pub mod package_2022_03;
#[cfg(all(feature = "package-2022-03", not(feature = "no-default-tag")))]
pub use package_2022_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-07-01")]
pub mod package_2021_07_01;
#[cfg(all(feature = "package-2021-07-01", not(feature = "no-default-tag")))]
pub use package_2021_07_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-11-preview")]
pub mod package_2020_11_preview;
#[cfg(all(feature = "package-2020-11-preview", not(feature = "no-default-tag")))]
pub use package_2020_11_preview::{models, Client, ClientBuilder};

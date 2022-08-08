#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2021-06")]
pub mod package_2021_06;
#[cfg(all(feature = "package-2021-06", not(feature = "no-default-tag")))]
pub use package_2021_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-12")]
pub mod package_2020_12;
#[cfg(all(feature = "package-2020-12", not(feature = "no-default-tag")))]
pub use package_2020_12::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-06")]
pub mod package_2020_06;
#[cfg(all(feature = "package-2020-06", not(feature = "no-default-tag")))]
pub use package_2020_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-07-preview")]
pub mod package_2019_07_preview;
#[cfg(all(feature = "package-2019-07-preview", not(feature = "no-default-tag")))]
pub use package_2019_07_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-03")]
pub mod package_2018_03;
#[cfg(all(feature = "package-2018-03", not(feature = "no-default-tag")))]
pub use package_2018_03::{models, Client, ClientBuilder};

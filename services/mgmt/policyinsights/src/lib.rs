#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2022-03")]
pub mod package_2022_03;
#[cfg(all(feature = "package-2022-03", not(feature = "no-default-tag")))]
pub use package_2022_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-10")]
pub mod package_2021_10;
#[cfg(all(feature = "package-2021-10", not(feature = "no-default-tag")))]
pub use package_2021_10::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-01")]
pub mod package_2021_01;
#[cfg(all(feature = "package-2021-01", not(feature = "no-default-tag")))]
pub use package_2021_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-07-preview")]
pub mod package_2020_07_preview;
#[cfg(all(feature = "package-2020-07-preview", not(feature = "no-default-tag")))]
pub use package_2020_07_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-07")]
pub mod package_2020_07;
#[cfg(all(feature = "package-2020-07", not(feature = "no-default-tag")))]
pub use package_2020_07::{models, Client, ClientBuilder};

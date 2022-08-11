#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-01-25")]
pub mod package_2022_01_25;
#[cfg(all(feature = "package-2022-01-25", not(feature = "no-default-tag")))]
pub use package_2022_01_25::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-01-25")]
pub mod package_2021_01_25;
#[cfg(all(feature = "package-2021-01-25", not(feature = "no-default-tag")))]
pub use package_2021_01_25::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-06-25")]
pub mod package_2020_06_25;
#[cfg(all(feature = "package-2020-06-25", not(feature = "no-default-tag")))]
pub use package_2020_06_25::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-11-20")]
pub mod package_2018_11_20;
#[cfg(all(feature = "package-2018-11-20", not(feature = "no-default-tag")))]
pub use package_2018_11_20::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-06-30-preview")]
pub mod package_2018_06_30_preview;
#[cfg(all(feature = "package-2018-06-30-preview", not(feature = "no-default-tag")))]
pub use package_2018_06_30_preview::{models, Client, ClientBuilder};

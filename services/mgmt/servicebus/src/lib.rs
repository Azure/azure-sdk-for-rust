#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2022-01-preview")]
pub mod package_2022_01_preview;
#[cfg(all(feature = "package-2022-01-preview", not(feature = "no-default-tag")))]
pub use package_2022_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-11")]
pub mod package_2021_11;
#[cfg(all(feature = "package-2021-11", not(feature = "no-default-tag")))]
pub use package_2021_11::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-06-preview")]
pub mod package_2021_06_preview;
#[cfg(all(feature = "package-2021-06-preview", not(feature = "no-default-tag")))]
pub use package_2021_06_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-01-preview")]
pub mod package_2021_01_preview;
#[cfg(all(feature = "package-2021-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-01-preview")]
pub mod package_2018_01_preview;
#[cfg(all(feature = "package-2018-01-preview", not(feature = "no-default-tag")))]
pub use package_2018_01_preview::{models, Client, ClientBuilder};

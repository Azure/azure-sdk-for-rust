#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2021-10")]
pub mod package_2021_10;
#[cfg(all(feature = "package-2021-10", not(feature = "no-default-tag")))]
pub use package_2021_10::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-11-preview")]
pub mod package_2020_11_preview;
#[cfg(all(feature = "package-2020-11-preview", not(feature = "no-default-tag")))]
pub use package_2020_11_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-09-preview")]
pub mod package_2020_09_preview;
#[cfg(all(feature = "package-2020-09-preview", not(feature = "no-default-tag")))]
pub use package_2020_09_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-05")]
pub mod package_2020_05;
#[cfg(all(feature = "package-2020-05", not(feature = "no-default-tag")))]
pub use package_2020_05::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-10-preview")]
pub mod package_2019_10_preview;
#[cfg(all(feature = "package-2019-10-preview", not(feature = "no-default-tag")))]
pub use package_2019_10_preview::{models, Client, ClientBuilder};

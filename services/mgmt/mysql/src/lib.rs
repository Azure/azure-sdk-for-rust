#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-flexibleserver-2021-12-01-preview")]
pub mod package_flexibleserver_2021_12_01_preview;
#[cfg(all(feature = "package-flexibleserver-2021-12-01-preview", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2021_12_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-flexibleserver-2021-05-01-preview")]
pub mod package_flexibleserver_2021_05_01_preview;
#[cfg(all(feature = "package-flexibleserver-2021-05-01-preview", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2021_05_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-flexibleserver-2021-05-01")]
pub mod package_flexibleserver_2021_05_01;
#[cfg(all(feature = "package-flexibleserver-2021-05-01", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2021_05_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-07-01-privatepreview")]
pub mod package_2020_07_01_privatepreview;
#[cfg(all(feature = "package-2020-07-01-privatepreview", not(feature = "no-default-tag")))]
pub use package_2020_07_01_privatepreview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-07-01-preview")]
pub mod package_2020_07_01_preview;
#[cfg(all(feature = "package-2020-07-01-preview", not(feature = "no-default-tag")))]
pub use package_2020_07_01_preview::{models, Client, ClientBuilder};

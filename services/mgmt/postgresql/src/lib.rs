#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-flexibleserver-2022-03-privatepreview")]
pub mod package_flexibleserver_2022_03_privatepreview;
#[cfg(all(feature = "package-flexibleserver-2022-03-privatepreview", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2022_03_privatepreview::{models, Client, ClientBuilder};
#[cfg(feature = "package-flexibleserver-2022-01-preview")]
pub mod package_flexibleserver_2022_01_preview;
#[cfg(all(feature = "package-flexibleserver-2022-01-preview", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2022_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-flexibleserver-2021-06-preview")]
pub mod package_flexibleserver_2021_06_preview;
#[cfg(all(feature = "package-flexibleserver-2021-06-preview", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2021_06_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-flexibleserver-2021-06")]
pub mod package_flexibleserver_2021_06;
#[cfg(all(feature = "package-flexibleserver-2021-06", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2021_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-06-15-privatepreview")]
pub mod package_2021_06_15_privatepreview;
#[cfg(all(feature = "package-2021-06-15-privatepreview", not(feature = "no-default-tag")))]
pub use package_2021_06_15_privatepreview::{models, Client, ClientBuilder};

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-netapp-2022-03-01")]
pub mod package_netapp_2022_03_01;
#[cfg(all(feature = "package-netapp-2022-03-01", not(feature = "no-default-tag")))]
pub use package_netapp_2022_03_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-netapp-2022-01-01")]
pub mod package_netapp_2022_01_01;
#[cfg(all(feature = "package-netapp-2022-01-01", not(feature = "no-default-tag")))]
pub use package_netapp_2022_01_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-netapp-2021-10-01")]
pub mod package_netapp_2021_10_01;
#[cfg(all(feature = "package-netapp-2021-10-01", not(feature = "no-default-tag")))]
pub use package_netapp_2021_10_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-netapp-2021-08-01")]
pub mod package_netapp_2021_08_01;
#[cfg(all(feature = "package-netapp-2021-08-01", not(feature = "no-default-tag")))]
pub use package_netapp_2021_08_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-netapp-2021-06-01")]
pub mod package_netapp_2021_06_01;
#[cfg(all(feature = "package-netapp-2021-06-01", not(feature = "no-default-tag")))]
pub use package_netapp_2021_06_01::{models, Client, ClientBuilder};

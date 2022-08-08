#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-vnet-2021-06-01-preview")]
pub mod package_vnet_2021_06_01_preview;
#[cfg(all(feature = "package-vnet-2021-06-01-preview", not(feature = "no-default-tag")))]
pub use package_vnet_2021_06_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-vnet-2020-12-01")]
pub mod package_vnet_2020_12_01;
#[cfg(all(feature = "package-vnet-2020-12-01", not(feature = "no-default-tag")))]
pub use package_vnet_2020_12_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-vnet-2019-06-01-preview")]
pub mod package_vnet_2019_06_01_preview;
#[cfg(all(feature = "package-vnet-2019-06-01-preview", not(feature = "no-default-tag")))]
pub use package_vnet_2019_06_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-spark-2020-12-01")]
pub mod package_spark_2020_12_01;
#[cfg(all(feature = "package-spark-2020-12-01", not(feature = "no-default-tag")))]
pub use package_spark_2020_12_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-spark-2019-11-01-preview")]
pub mod package_spark_2019_11_01_preview;
#[cfg(all(feature = "package-spark-2019-11-01-preview", not(feature = "no-default-tag")))]
pub use package_spark_2019_11_01_preview::{models, Client, ClientBuilder};

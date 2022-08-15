#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-sqlGen3-2020-04-01-preview")]
pub mod package_sqlgen3_2020_04_01_preview;
#[cfg(all(feature = "package-sqlGen3-2020-04-01-preview", not(feature = "no-default-tag")))]
pub use package_sqlgen3_2020_04_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2021-06")]
pub mod package_preview_2021_06;
#[cfg(all(feature = "package-preview-2021-06", not(feature = "no-default-tag")))]
pub use package_preview_2021_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-kusto-pool-2021-04-preview")]
pub mod package_kusto_pool_2021_04_preview;
#[cfg(all(feature = "package-kusto-pool-2021-04-preview", not(feature = "no-default-tag")))]
pub use package_kusto_pool_2021_04_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-composite-v2")]
pub mod package_composite_v2;
#[cfg(all(feature = "package-composite-v2", not(feature = "no-default-tag")))]
pub use package_composite_v2::{models, Client, ClientBuilder};
#[cfg(feature = "package-composite-v1")]
pub mod package_composite_v1;
#[cfg(all(feature = "package-composite-v1", not(feature = "no-default-tag")))]
pub use package_composite_v1::{models, Client, ClientBuilder};

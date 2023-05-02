#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-flexibleserver-2022-09-30-preview-privatelink")]
pub mod package_flexibleserver_2022_09_30_preview_privatelink;
#[cfg(all(feature = "package-flexibleserver-2022-09-30-preview-privatelink", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2022_09_30_preview_privatelink::*;
#[cfg(feature = "package-flexibleserver-2022-09-30-preview")]
pub mod package_flexibleserver_2022_09_30_preview;
#[cfg(all(feature = "package-flexibleserver-2022-09-30-preview", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2022_09_30_preview::*;
#[cfg(feature = "package-flexibleserver-2022-01-01")]
pub mod package_flexibleserver_2022_01_01;
#[cfg(all(feature = "package-flexibleserver-2022-01-01", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2022_01_01::*;
#[cfg(feature = "package-flexibleserver-2021-12-01-preview")]
pub mod package_flexibleserver_2021_12_01_preview;
#[cfg(all(feature = "package-flexibleserver-2021-12-01-preview", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2021_12_01_preview::*;
#[cfg(feature = "package-flexibleserver-2021-05-01-preview")]
pub mod package_flexibleserver_2021_05_01_preview;
#[cfg(all(feature = "package-flexibleserver-2021-05-01-preview", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2021_05_01_preview::*;

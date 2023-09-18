#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-flexibleserver-2023-06-30-privatelink")]
pub mod package_flexibleserver_2023_06_30_privatelink;
#[cfg(all(feature = "package-flexibleserver-2023-06-30-privatelink", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2023_06_30_privatelink::*;
#[cfg(feature = "package-flexibleserver-2023-06-01-preview")]
pub mod package_flexibleserver_2023_06_01_preview;
#[cfg(all(feature = "package-flexibleserver-2023-06-01-preview", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2023_06_01_preview::*;
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

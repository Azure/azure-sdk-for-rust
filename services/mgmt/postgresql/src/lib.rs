#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-flexibleserver-2023-03-01-preview")]
pub mod package_flexibleserver_2023_03_01_preview;
#[cfg(all(feature = "package-flexibleserver-2023-03-01-preview", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2023_03_01_preview::*;
#[cfg(feature = "package-flexibleserver-2022-12-01")]
pub mod package_flexibleserver_2022_12_01;
#[cfg(all(feature = "package-flexibleserver-2022-12-01", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2022_12_01::*;
#[cfg(feature = "package-flexibleserver-2022-03-privatepreview")]
pub mod package_flexibleserver_2022_03_privatepreview;
#[cfg(all(feature = "package-flexibleserver-2022-03-privatepreview", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2022_03_privatepreview::*;
#[cfg(feature = "package-flexibleserver-2022-03-preview")]
pub mod package_flexibleserver_2022_03_preview;
#[cfg(all(feature = "package-flexibleserver-2022-03-preview", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2022_03_preview::*;
#[cfg(feature = "package-flexibleserver-2022-01-preview")]
pub mod package_flexibleserver_2022_01_preview;
#[cfg(all(feature = "package-flexibleserver-2022-01-preview", not(feature = "no-default-tag")))]
pub use package_flexibleserver_2022_01_preview::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-dotnet-sdk")]
pub mod package_dotnet_sdk;
#[cfg(all(feature = "package-dotnet-sdk", not(feature = "no-default-tag")))]
pub use package_dotnet_sdk::*;
#[cfg(feature = "package-composite-v2")]
pub mod package_composite_v2;
#[cfg(all(feature = "package-composite-v2", not(feature = "no-default-tag")))]
pub use package_composite_v2::*;
#[cfg(feature = "package-composite-v1")]
pub mod package_composite_v1;
#[cfg(all(feature = "package-composite-v1", not(feature = "no-default-tag")))]
pub use package_composite_v1::*;
#[cfg(feature = "package-2023-01")]
pub mod package_2023_01;
#[cfg(all(feature = "package-2023-01", not(feature = "no-default-tag")))]
pub use package_2023_01::*;
#[cfg(feature = "package-2022-05")]
pub mod package_2022_05;
#[cfg(all(feature = "package-2022-05", not(feature = "no-default-tag")))]
pub use package_2022_05::*;

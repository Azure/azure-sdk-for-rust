#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-12-01-preview")]
pub mod package_2021_12_01_preview;
#[cfg(all(feature = "package-2021-12-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_12_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-06")]
pub mod package_2021_06;
#[cfg(all(feature = "package-2021-06", not(feature = "no-default-tag")))]
pub use package_2021_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-10")]
pub mod package_2020_10;
#[cfg(all(feature = "package-2020-10", not(feature = "no-default-tag")))]
pub use package_2020_10::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-08")]
pub mod package_2020_08;
#[cfg(all(feature = "package-2020-08", not(feature = "no-default-tag")))]
pub use package_2020_08::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-03-preview")]
pub mod package_2020_03_preview;
#[cfg(all(feature = "package-2020-03-preview", not(feature = "no-default-tag")))]
pub use package_2020_03_preview::{models, Client, ClientBuilder};

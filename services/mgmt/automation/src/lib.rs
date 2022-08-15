#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-02-22")]
pub mod package_2022_02_22;
#[cfg(all(feature = "package-2022-02-22", not(feature = "no-default-tag")))]
pub use package_2022_02_22::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-01-31")]
pub mod package_2022_01_31;
#[cfg(all(feature = "package-2022-01-31", not(feature = "no-default-tag")))]
pub use package_2022_01_31::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-06-22")]
pub mod package_2021_06_22;
#[cfg(all(feature = "package-2021-06-22", not(feature = "no-default-tag")))]
pub use package_2021_06_22::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-01-13-preview")]
pub mod package_2020_01_13_preview;
#[cfg(all(feature = "package-2020-01-13-preview", not(feature = "no-default-tag")))]
pub use package_2020_01_13_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-06")]
pub mod package_2019_06;
#[cfg(all(feature = "package-2019-06", not(feature = "no-default-tag")))]
pub use package_2019_06::{models, Client, ClientBuilder};

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-05-31")]
pub mod package_2022_05_31;
#[cfg(all(feature = "package-2022-05-31", not(feature = "no-default-tag")))]
pub use package_2022_05_31::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-06-30-preview")]
pub mod package_2021_06_30_preview;
#[cfg(all(feature = "package-2021-06-30-preview", not(feature = "no-default-tag")))]
pub use package_2021_06_30_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-10-31")]
pub mod package_2020_10_31;
#[cfg(all(feature = "package-2020-10-31", not(feature = "no-default-tag")))]
pub use package_2020_10_31::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-05-31-preview")]
pub mod package_2020_05_31_preview;
#[cfg(all(feature = "package-2020-05-31-preview", not(feature = "no-default-tag")))]
pub use package_2020_05_31_preview::{models, Client, ClientBuilder};

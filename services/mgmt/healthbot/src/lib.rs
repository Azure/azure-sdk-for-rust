#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-08-24")]
pub mod package_2021_08_24;
#[cfg(all(feature = "package-2021-08-24", not(feature = "no-default-tag")))]
pub use package_2021_08_24::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-06-10")]
pub mod package_2021_06_10;
#[cfg(all(feature = "package-2021-06-10", not(feature = "no-default-tag")))]
pub use package_2021_06_10::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-12-08-preview")]
pub mod package_2020_12_08_preview;
#[cfg(all(feature = "package-2020-12-08-preview", not(feature = "no-default-tag")))]
pub use package_2020_12_08_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-12-08")]
pub mod package_2020_12_08;
#[cfg(all(feature = "package-2020-12-08", not(feature = "no-default-tag")))]
pub use package_2020_12_08::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-10-20-preview")]
pub mod package_2020_10_20_preview;
#[cfg(all(feature = "package-2020-10-20-preview", not(feature = "no-default-tag")))]
pub use package_2020_10_20_preview::{models, Client, ClientBuilder};

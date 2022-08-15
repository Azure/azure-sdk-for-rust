#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2020-05-01-preview")]
pub mod package_2020_05_01_preview;
#[cfg(all(feature = "package-2020-05-01-preview", not(feature = "no-default-tag")))]
pub use package_2020_05_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-05-01")]
pub mod package_2020_05_01;
#[cfg(all(feature = "package-2020-05-01", not(feature = "no-default-tag")))]
pub use package_2020_05_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-08-preview")]
pub mod package_2018_08_preview;
#[cfg(all(feature = "package-2018-08-preview", not(feature = "no-default-tag")))]
pub use package_2018_08_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-07-01")]
pub mod package_2018_07_01;
#[cfg(all(feature = "package-2018-07-01", not(feature = "no-default-tag")))]
pub use package_2018_07_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2017-07")]
pub mod package_2017_07;
#[cfg(all(feature = "package-2017-07", not(feature = "no-default-tag")))]
pub use package_2017_07::{models, Client, ClientBuilder};

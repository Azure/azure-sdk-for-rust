#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-04-preview")]
pub mod package_2021_04_preview;
#[cfg(all(feature = "package-2021-04-preview", not(feature = "no-default-tag")))]
pub use package_2021_04_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-08-preview")]
pub mod package_2020_08_preview;
#[cfg(all(feature = "package-2020-08-preview", not(feature = "no-default-tag")))]
pub use package_2020_08_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-08")]
pub mod package_2020_08;
#[cfg(all(feature = "package-2020-08", not(feature = "no-default-tag")))]
pub use package_2020_08::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-03")]
pub mod package_2020_03;
#[cfg(all(feature = "package-2020-03", not(feature = "no-default-tag")))]
pub use package_2020_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-10-preview")]
pub mod package_2019_10_preview;
#[cfg(all(feature = "package-2019-10-preview", not(feature = "no-default-tag")))]
pub use package_2019_10_preview::{models, Client, ClientBuilder};

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-pure-2020-03-preview")]
pub mod package_pure_2020_03_preview;
#[cfg(all(feature = "package-pure-2020-03-preview", not(feature = "no-default-tag")))]
pub use package_pure_2020_03_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-pure-2020-03")]
pub mod package_pure_2020_03;
#[cfg(all(feature = "package-pure-2020-03", not(feature = "no-default-tag")))]
pub use package_pure_2020_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-pure-2017-04-preview")]
pub mod package_pure_2017_04_preview;
#[cfg(all(feature = "package-pure-2017-04-preview", not(feature = "no-default-tag")))]
pub use package_pure_2017_04_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-pure-2016-03")]
pub mod package_pure_2016_03;
#[cfg(all(feature = "package-pure-2016-03", not(feature = "no-default-tag")))]
pub use package_pure_2016_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-10-preview")]
pub mod package_2021_10_preview;
#[cfg(all(feature = "package-2021-10-preview", not(feature = "no-default-tag")))]
pub use package_2021_10_preview::{models, Client, ClientBuilder};

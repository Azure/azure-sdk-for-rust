#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-preview-2020-07")]
pub mod package_preview_2020_07;
#[cfg(all(feature = "package-preview-2020-07", not(feature = "no-default-tag")))]
pub use package_preview_2020_07::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2020-03")]
pub mod package_preview_2020_03;
#[cfg(all(feature = "package-preview-2020-03", not(feature = "no-default-tag")))]
pub use package_preview_2020_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2017-04")]
pub mod package_preview_2017_04;
#[cfg(all(feature = "package-preview-2017-04", not(feature = "no-default-tag")))]
pub use package_preview_2017_04::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-03")]
pub mod package_2020_03;
#[cfg(all(feature = "package-2020-03", not(feature = "no-default-tag")))]
pub use package_2020_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-2017-04-01")]
pub mod package_2017_04_01;
#[cfg(all(feature = "package-2017-04-01", not(feature = "no-default-tag")))]
pub use package_2017_04_01::{models, Client, ClientBuilder};

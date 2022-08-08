#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-preview-2022-01")]
pub mod package_preview_2022_01;
#[cfg(all(feature = "package-preview-2022-01", not(feature = "no-default-tag")))]
pub use package_preview_2022_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-02-preview")]
pub mod package_2020_02_preview;
#[cfg(all(feature = "package-2020-02-preview", not(feature = "no-default-tag")))]
pub use package_2020_02_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-09")]
pub mod package_2019_09;
#[cfg(all(feature = "package-2019-09", not(feature = "no-default-tag")))]
pub use package_2019_09::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-06")]
pub mod package_2019_06;
#[cfg(all(feature = "package-2019-06", not(feature = "no-default-tag")))]
pub use package_2019_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-04-preview")]
pub mod package_2019_04_preview;
#[cfg(all(feature = "package-2019-04-preview", not(feature = "no-default-tag")))]
pub use package_2019_04_preview::{models, Client, ClientBuilder};

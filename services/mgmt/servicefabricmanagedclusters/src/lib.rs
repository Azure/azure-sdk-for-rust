#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2022-06-preview")]
pub mod package_2022_06_preview;
#[cfg(all(feature = "package-2022-06-preview", not(feature = "no-default-tag")))]
pub use package_2022_06_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-02-preview")]
pub mod package_2022_02_preview;
#[cfg(all(feature = "package-2022-02-preview", not(feature = "no-default-tag")))]
pub use package_2022_02_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-01")]
pub mod package_2022_01;
#[cfg(all(feature = "package-2022-01", not(feature = "no-default-tag")))]
pub use package_2022_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-11-preview")]
pub mod package_2021_11_preview;
#[cfg(all(feature = "package-2021-11-preview", not(feature = "no-default-tag")))]
pub use package_2021_11_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-09-privatepreview")]
pub mod package_2021_09_privatepreview;
#[cfg(all(feature = "package-2021-09-privatepreview", not(feature = "no-default-tag")))]
pub use package_2021_09_privatepreview::{models, Client, ClientBuilder};

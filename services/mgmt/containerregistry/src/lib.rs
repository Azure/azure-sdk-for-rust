#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2022-02-preview")]
pub mod package_2022_02_preview;
#[cfg(all(feature = "package-2022-02-preview", not(feature = "no-default-tag")))]
pub use package_2022_02_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-12-preview")]
pub mod package_2021_12_preview;
#[cfg(all(feature = "package-2021-12-preview", not(feature = "no-default-tag")))]
pub use package_2021_12_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-09")]
pub mod package_2021_09;
#[cfg(all(feature = "package-2021-09", not(feature = "no-default-tag")))]
pub use package_2021_09::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-08-preview")]
pub mod package_2021_08_preview;
#[cfg(all(feature = "package-2021-08-preview", not(feature = "no-default-tag")))]
pub use package_2021_08_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-06-preview")]
pub mod package_2021_06_preview;
#[cfg(all(feature = "package-2021-06-preview", not(feature = "no-default-tag")))]
pub use package_2021_06_preview::{models, Client, ClientBuilder};

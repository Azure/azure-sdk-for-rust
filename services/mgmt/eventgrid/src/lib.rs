#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2022-06")]
pub mod package_2022_06;
#[cfg(all(feature = "package-2022-06", not(feature = "no-default-tag")))]
pub use package_2022_06::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-12")]
pub mod package_2021_12;
#[cfg(all(feature = "package-2021-12", not(feature = "no-default-tag")))]
pub use package_2021_12::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-10-preview")]
pub mod package_2021_10_preview;
#[cfg(all(feature = "package-2021-10-preview", not(feature = "no-default-tag")))]
pub use package_2021_10_preview::{models, Client, ClientBuilder};

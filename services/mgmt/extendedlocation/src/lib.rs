#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-2021-08-31-preview")]
pub mod package_2021_08_31_preview;
#[cfg(all(feature = "package-2021-08-31-preview", not(feature = "no-default-tag")))]
pub use package_2021_08_31_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-08-15")]
pub mod package_2021_08_15;
#[cfg(all(feature = "package-2021-08-15", not(feature = "no-default-tag")))]
pub use package_2021_08_15::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-03-15-preview")]
pub mod package_2021_03_15_preview;
#[cfg(all(feature = "package-2021-03-15-preview", not(feature = "no-default-tag")))]
pub use package_2021_03_15_preview::{models, Client, ClientBuilder};

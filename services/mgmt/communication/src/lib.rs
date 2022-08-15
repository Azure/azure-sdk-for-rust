#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2022-07")]
pub mod package_preview_2022_07;
#[cfg(all(feature = "package-preview-2022-07", not(feature = "no-default-tag")))]
pub use package_preview_2022_07::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-10-01-preview")]
pub mod package_2021_10_01_preview;
#[cfg(all(feature = "package-2021-10-01-preview", not(feature = "no-default-tag")))]
pub use package_2021_10_01_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-08-20-preview")]
pub mod package_2020_08_20_preview;
#[cfg(all(feature = "package-2020-08-20-preview", not(feature = "no-default-tag")))]
pub use package_2020_08_20_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-08-20")]
pub mod package_2020_08_20;
#[cfg(all(feature = "package-2020-08-20", not(feature = "no-default-tag")))]
pub use package_2020_08_20::{models, Client, ClientBuilder};

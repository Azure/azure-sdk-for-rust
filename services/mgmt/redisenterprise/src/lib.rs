#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2021-02")]
pub mod package_preview_2021_02;
#[cfg(all(feature = "package-preview-2021-02", not(feature = "no-default-tag")))]
pub use package_preview_2021_02::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-01")]
pub mod package_2022_01;
#[cfg(all(feature = "package-2022-01", not(feature = "no-default-tag")))]
pub use package_2022_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-08")]
pub mod package_2021_08;
#[cfg(all(feature = "package-2021-08", not(feature = "no-default-tag")))]
pub use package_2021_08::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-03")]
pub mod package_2021_03;
#[cfg(all(feature = "package-2021-03", not(feature = "no-default-tag")))]
pub use package_2021_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-10-01-preview")]
pub mod package_2020_10_01_preview;
#[cfg(all(feature = "package-2020-10-01-preview", not(feature = "no-default-tag")))]
pub use package_2020_10_01_preview::{models, Client, ClientBuilder};

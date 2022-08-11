#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-06-30-preview")]
pub mod package_2022_06_30_preview;
#[cfg(all(feature = "package-2022-06-30-preview", not(feature = "no-default-tag")))]
pub use package_2022_06_30_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-05-31")]
pub mod package_2022_05_31;
#[cfg(all(feature = "package-2022-05-31", not(feature = "no-default-tag")))]
pub use package_2022_05_31::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-04-30-preview")]
pub mod package_2021_04_30_preview;
#[cfg(all(feature = "package-2021-04-30-preview", not(feature = "no-default-tag")))]
pub use package_2021_04_30_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-1_2-preview")]
pub mod package_1_2_preview;
#[cfg(all(feature = "package-1_2-preview", not(feature = "no-default-tag")))]
pub use package_1_2_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-1_1-preview")]
pub mod package_1_1_preview;
#[cfg(all(feature = "package-1_1-preview", not(feature = "no-default-tag")))]
pub use package_1_1_preview::{models, Client, ClientBuilder};

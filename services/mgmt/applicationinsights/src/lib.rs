#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-preview-2020-02")]
pub mod package_preview_2020_02;
#[cfg(all(feature = "package-preview-2020-02", not(feature = "no-default-tag")))]
pub use package_preview_2020_02::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-06-15")]
pub mod package_2022_06_15;
#[cfg(all(feature = "package-2022-06-15", not(feature = "no-default-tag")))]
pub use package_2022_06_15::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-04-01")]
pub mod package_2022_04_01;
#[cfg(all(feature = "package-2022-04-01", not(feature = "no-default-tag")))]
pub use package_2022_04_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-02-01")]
pub mod package_2022_02_01;
#[cfg(all(feature = "package-2022-02-01", not(feature = "no-default-tag")))]
pub use package_2022_02_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-01-11")]
pub mod package_2022_01_11;
#[cfg(all(feature = "package-2022-01-11", not(feature = "no-default-tag")))]
pub use package_2022_01_11::{models, Client, ClientBuilder};

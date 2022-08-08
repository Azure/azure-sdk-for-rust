#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-preview-2022-01")]
pub mod package_preview_2022_01;
#[cfg(all(feature = "package-preview-2022-01", not(feature = "no-default-tag")))]
pub use package_preview_2022_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2021-11")]
pub mod package_preview_2021_11;
#[cfg(all(feature = "package-preview-2021-11", not(feature = "no-default-tag")))]
pub use package_preview_2021_11::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-04")]
pub mod package_2022_04;
#[cfg(all(feature = "package-2022-04", not(feature = "no-default-tag")))]
pub use package_2022_04::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-03")]
pub mod package_2022_03;
#[cfg(all(feature = "package-2022-03", not(feature = "no-default-tag")))]
pub use package_2022_03::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-02")]
pub mod package_2022_02;
#[cfg(all(feature = "package-2022-02", not(feature = "no-default-tag")))]
pub use package_2022_02::{models, Client, ClientBuilder};

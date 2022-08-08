#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-preview-2019-05")]
pub mod package_preview_2019_05;
#[cfg(all(feature = "package-preview-2019-05", not(feature = "no-default-tag")))]
pub use package_preview_2019_05::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2019-04")]
pub mod package_preview_2019_04;
#[cfg(all(feature = "package-preview-2019-04", not(feature = "no-default-tag")))]
pub use package_preview_2019_04::{models, Client, ClientBuilder};
#[cfg(feature = "package-preview-2018-11")]
pub mod package_preview_2018_11;
#[cfg(all(feature = "package-preview-2018-11", not(feature = "no-default-tag")))]
pub use package_preview_2018_11::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-10")]
pub mod package_2021_10;
#[cfg(all(feature = "package-2021-10", not(feature = "no-default-tag")))]
pub use package_2021_10::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-10")]
pub mod package_2019_10;
#[cfg(all(feature = "package-2019-10", not(feature = "no-default-tag")))]
pub use package_2019_10::{models, Client, ClientBuilder};

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-2019-05")]
pub mod package_preview_2019_05;
#[cfg(all(feature = "package-preview-2019-05", not(feature = "no-default-tag")))]
pub use package_preview_2019_05::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-02")]
pub mod package_2022_02;
#[cfg(all(feature = "package-2022-02", not(feature = "no-default-tag")))]
pub use package_2022_02::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-10")]
pub mod package_2021_10;
#[cfg(all(feature = "package-2021-10", not(feature = "no-default-tag")))]
pub use package_2021_10::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-02")]
pub mod package_2020_02;
#[cfg(all(feature = "package-2020-02", not(feature = "no-default-tag")))]
pub use package_2020_02::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-02")]
pub mod package_2019_02;
#[cfg(all(feature = "package-2019-02", not(feature = "no-default-tag")))]
pub use package_2019_02::{models, Client, ClientBuilder};

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2020-07")]
pub mod package_2020_07;
#[cfg(all(feature = "package-2020-07", not(feature = "no-default-tag")))]
pub use package_2020_07::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-05")]
pub mod package_2020_05;
#[cfg(all(feature = "package-2020-05", not(feature = "no-default-tag")))]
pub use package_2020_05::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-01")]
pub mod package_2020_01;
#[cfg(all(feature = "package-2020-01", not(feature = "no-default-tag")))]
pub use package_2020_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2019-10")]
pub mod package_2019_10;
#[cfg(all(feature = "package-2019-10", not(feature = "no-default-tag")))]
pub use package_2019_10::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-02")]
pub mod package_2018_02;
#[cfg(all(feature = "package-2018-02", not(feature = "no-default-tag")))]
pub use package_2018_02::{models, Client, ClientBuilder};

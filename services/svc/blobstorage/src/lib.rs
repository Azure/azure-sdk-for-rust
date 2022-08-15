#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2021-08")]
pub mod package_2021_08;
#[cfg(all(feature = "package-2021-08", not(feature = "no-default-tag")))]
pub use package_2021_08::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-04")]
pub mod package_2021_04;
#[cfg(all(feature = "package-2021-04", not(feature = "no-default-tag")))]
pub use package_2021_04::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-02")]
pub mod package_2021_02;
#[cfg(all(feature = "package-2021-02", not(feature = "no-default-tag")))]
pub use package_2021_02::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-12")]
pub mod package_2020_12;
#[cfg(all(feature = "package-2020-12", not(feature = "no-default-tag")))]
pub use package_2020_12::{models, Client, ClientBuilder};
#[cfg(feature = "package-2020-10")]
pub mod package_2020_10;
#[cfg(all(feature = "package-2020-10", not(feature = "no-default-tag")))]
pub use package_2020_10::{models, Client, ClientBuilder};

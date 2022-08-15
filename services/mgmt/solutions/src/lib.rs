#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-managedapplications-2021-07")]
pub mod package_managedapplications_2021_07;
#[cfg(all(feature = "package-managedapplications-2021-07", not(feature = "no-default-tag")))]
pub use package_managedapplications_2021_07::{models, Client, ClientBuilder};
#[cfg(feature = "package-managedapplications-2021-02")]
pub mod package_managedapplications_2021_02;
#[cfg(all(feature = "package-managedapplications-2021-02", not(feature = "no-default-tag")))]
pub use package_managedapplications_2021_02::{models, Client, ClientBuilder};
#[cfg(feature = "package-managedapplications-2020-08")]
pub mod package_managedapplications_2020_08;
#[cfg(all(feature = "package-managedapplications-2020-08", not(feature = "no-default-tag")))]
pub use package_managedapplications_2020_08::{models, Client, ClientBuilder};
#[cfg(feature = "package-managedapplications-2019-07")]
pub mod package_managedapplications_2019_07;
#[cfg(all(feature = "package-managedapplications-2019-07", not(feature = "no-default-tag")))]
pub use package_managedapplications_2019_07::{models, Client, ClientBuilder};
#[cfg(feature = "package-managedapplications-2018-09")]
pub mod package_managedapplications_2018_09;
#[cfg(all(feature = "package-managedapplications-2018-09", not(feature = "no-default-tag")))]
pub use package_managedapplications_2018_09::{models, Client, ClientBuilder};

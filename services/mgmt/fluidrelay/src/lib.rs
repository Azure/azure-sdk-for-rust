#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-2022-06-01")]
pub mod package_2022_06_01;
#[cfg(all(feature = "package-2022-06-01", not(feature = "no-default-tag")))]
pub use package_2022_06_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-05-26")]
pub mod package_2022_05_26;
#[cfg(all(feature = "package-2022-05-26", not(feature = "no-default-tag")))]
pub use package_2022_05_26::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-05-11")]
pub mod package_2022_05_11;
#[cfg(all(feature = "package-2022-05-11", not(feature = "no-default-tag")))]
pub use package_2022_05_11::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-04-21")]
pub mod package_2022_04_21;
#[cfg(all(feature = "package-2022-04-21", not(feature = "no-default-tag")))]
pub use package_2022_04_21::{models, Client, ClientBuilder};
#[cfg(feature = "package-2022-02-15")]
pub mod package_2022_02_15;
#[cfg(all(feature = "package-2022-02-15", not(feature = "no-default-tag")))]
pub use package_2022_02_15::{models, Client, ClientBuilder};

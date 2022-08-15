#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "profile-hybrid-2020-09-01")]
pub mod profile_hybrid_2020_09_01;
#[cfg(all(feature = "profile-hybrid-2020-09-01", not(feature = "no-default-tag")))]
pub use profile_hybrid_2020_09_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-05")]
pub mod package_2018_05;
#[cfg(all(feature = "package-2018-05", not(feature = "no-default-tag")))]
pub use package_2018_05::{models, Client, ClientBuilder};
#[cfg(feature = "package-2018-03-preview")]
pub mod package_2018_03_preview;
#[cfg(all(feature = "package-2018-03-preview", not(feature = "no-default-tag")))]
pub use package_2018_03_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-2017-10")]
pub mod package_2017_10;
#[cfg(all(feature = "package-2017-10", not(feature = "no-default-tag")))]
pub use package_2017_10::{models, Client, ClientBuilder};
#[cfg(feature = "package-2017-09")]
pub mod package_2017_09;
#[cfg(all(feature = "package-2017-09", not(feature = "no-default-tag")))]
pub use package_2017_09::{models, Client, ClientBuilder};

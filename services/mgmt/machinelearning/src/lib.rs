#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-workspaces-2019-10")]
pub mod package_workspaces_2019_10;
#[cfg(all(feature = "package-workspaces-2019-10", not(feature = "no-default-tag")))]
pub use package_workspaces_2019_10::{models, Client, ClientBuilder};
#[cfg(feature = "package-workspaces-2016-04")]
pub mod package_workspaces_2016_04;
#[cfg(all(feature = "package-workspaces-2016-04", not(feature = "no-default-tag")))]
pub use package_workspaces_2016_04::{models, Client, ClientBuilder};
#[cfg(feature = "package-webservices-2017-01")]
pub mod package_webservices_2017_01;
#[cfg(all(feature = "package-webservices-2017-01", not(feature = "no-default-tag")))]
pub use package_webservices_2017_01::{models, Client, ClientBuilder};
#[cfg(feature = "package-webservices-2016-05-preview")]
pub mod package_webservices_2016_05_preview;
#[cfg(all(feature = "package-webservices-2016-05-preview", not(feature = "no-default-tag")))]
pub use package_webservices_2016_05_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-commitmentPlans-2016-05-preview")]
pub mod package_commitmentplans_2016_05_preview;
#[cfg(all(feature = "package-commitmentPlans-2016-05-preview", not(feature = "no-default-tag")))]
pub use package_commitmentplans_2016_05_preview::{models, Client, ClientBuilder};

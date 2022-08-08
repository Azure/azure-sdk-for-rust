#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-job-2017-09-preview")]
pub mod package_job_2017_09_preview;
#[cfg(all(feature = "package-job-2017-09-preview", not(feature = "no-default-tag")))]
pub use package_job_2017_09_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-job-2016-11")]
pub mod package_job_2016_11;
#[cfg(all(feature = "package-job-2016-11", not(feature = "no-default-tag")))]
pub use package_job_2016_11::{models, Client, ClientBuilder};
#[cfg(feature = "package-job-2016-03-preview")]
pub mod package_job_2016_03_preview;
#[cfg(all(feature = "package-job-2016-03-preview", not(feature = "no-default-tag")))]
pub use package_job_2016_03_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-job-2015-11-preview")]
pub mod package_job_2015_11_preview;
#[cfg(all(feature = "package-job-2015-11-preview", not(feature = "no-default-tag")))]
pub use package_job_2015_11_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-catalog-2016-11")]
pub mod package_catalog_2016_11;
#[cfg(all(feature = "package-catalog-2016-11", not(feature = "no-default-tag")))]
pub use package_catalog_2016_11::{models, Client, ClientBuilder};

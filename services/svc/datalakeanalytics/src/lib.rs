#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-job-2017-09-preview")]
pub mod package_job_2017_09_preview;
#[cfg(all(feature = "package-job-2017-09-preview", not(feature = "without_tag_import")))]
pub use package_job_2017_09_preview::*;
#[cfg(feature = "package-job-2016-11")]
pub mod package_job_2016_11;
#[cfg(all(feature = "package-job-2016-11", not(feature = "without_tag_import")))]
pub use package_job_2016_11::*;
#[cfg(feature = "package-job-2016-03-preview")]
pub mod package_job_2016_03_preview;
#[cfg(all(feature = "package-job-2016-03-preview", not(feature = "without_tag_import")))]
pub use package_job_2016_03_preview::*;
#[cfg(feature = "package-job-2015-11-preview")]
pub mod package_job_2015_11_preview;
#[cfg(all(feature = "package-job-2015-11-preview", not(feature = "without_tag_import")))]
pub use package_job_2015_11_preview::*;
#[cfg(feature = "package-catalog-2016-11")]
pub mod package_catalog_2016_11;
#[cfg(all(feature = "package-catalog-2016-11", not(feature = "without_tag_import")))]
pub use package_catalog_2016_11::*;

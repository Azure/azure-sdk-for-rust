#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-commitmentPlans-2016-05-preview")]
pub mod package_commitmentplans_2016_05_preview;
#[cfg(feature = "package-webservices-2016-05-preview")]
pub mod package_webservices_2016_05_preview;
#[cfg(feature = "package-webservices-2017-01")]
pub mod package_webservices_2017_01;
#[cfg(feature = "package-workspaces-2016-04")]
pub mod package_workspaces_2016_04;
#[cfg(feature = "package-workspaces-2019-10")]
pub mod package_workspaces_2019_10;
#[cfg(all(feature = "default_tag", feature = "package-workspaces-2019-10"))]
pub use package_workspaces_2019_10::*;

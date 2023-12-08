#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2021-10-preview")]
pub mod package_2021_10_preview;
#[cfg(feature = "package-pure-2016-03")]
pub mod package_pure_2016_03;
#[cfg(feature = "package-pure-2017-04-preview")]
pub mod package_pure_2017_04_preview;
#[cfg(feature = "package-pure-2020-03")]
pub mod package_pure_2020_03;
#[cfg(feature = "package-pure-2020-03-preview")]
pub mod package_pure_2020_03_preview;
#[cfg(all(feature = "default_tag", feature = "package-pure-2020-03"))]
pub use package_pure_2020_03::*;

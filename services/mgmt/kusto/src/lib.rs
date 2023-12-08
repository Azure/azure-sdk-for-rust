#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "schema-2017-09-07-privatepreview")]
pub mod schema_2017_09_07_privatepreview;
#[cfg(feature = "schema-2018-09-07-preview")]
pub mod schema_2018_09_07_preview;
#[cfg(feature = "schema-2019-01-21")]
pub mod schema_2019_01_21;
#[cfg(feature = "schema-2019-05-15")]
pub mod schema_2019_05_15;
#[cfg(feature = "schema-2019-09-07")]
pub mod schema_2019_09_07;
#[cfg(all(feature = "default_tag", feature = "schema-2019-09-07"))]
pub use schema_2019_09_07::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "v1")]
pub mod v1;
#[cfg(feature = "20171001")]
pub mod v20171001;
#[cfg(feature = "20210519")]
pub mod v20210519;
#[cfg(feature = "20221027")]
pub mod v20221027;
#[cfg(feature = "20221027Preview")]
pub mod v20221027preview;
#[cfg(all(feature = "default_tag", feature = "v1"))]
pub use v1::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "7_1")]
pub mod v7_1;
#[cfg(feature = "7_2")]
pub mod v7_2;
#[cfg(feature = "8_0")]
pub mod v8_0;
#[cfg(feature = "8_1")]
pub mod v8_1;
#[cfg(feature = "8_2")]
pub mod v8_2;
#[cfg(all(feature = "default_tag", feature = "8_1"))]
pub use v8_1::*;

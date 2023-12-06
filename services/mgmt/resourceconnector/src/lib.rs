#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2022-10-27")]
pub mod package_2022_10_27;
#[cfg(all(feature = "package-2022-10-27", not(feature = "without_tag_import")))]
pub use package_2022_10_27::*;
#[cfg(feature = "package-2022-04-15-preview")]
pub mod package_2022_04_15_preview;
#[cfg(all(feature = "package-2022-04-15-preview", not(feature = "without_tag_import")))]
pub use package_2022_04_15_preview::*;
#[cfg(feature = "package-2021-10-31-preview")]
pub mod package_2021_10_31_preview;
#[cfg(all(feature = "package-2021-10-31-preview", not(feature = "without_tag_import")))]
pub use package_2021_10_31_preview::*;

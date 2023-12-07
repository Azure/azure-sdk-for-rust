#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-preview-2023-01")]
pub mod package_preview_2023_01;
#[cfg(all(feature = "package-preview-2023-01", not(feature = "without_tag_import")))]
pub use package_preview_2023_01::*;
#[cfg(feature = "package-2023-09")]
pub mod package_2023_09;
#[cfg(all(feature = "package-2023-09", not(feature = "without_tag_import")))]
pub use package_2023_09::*;
#[cfg(feature = "package-2017-04")]
pub mod package_2017_04;
#[cfg(all(feature = "package-2017-04", not(feature = "without_tag_import")))]
pub use package_2017_04::*;
#[cfg(feature = "package-2016-03")]
pub mod package_2016_03;
#[cfg(all(feature = "package-2016-03", not(feature = "without_tag_import")))]
pub use package_2016_03::*;
#[cfg(feature = "package-2014-09")]
pub mod package_2014_09;
#[cfg(all(feature = "package-2014-09", not(feature = "without_tag_import")))]
pub use package_2014_09::*;

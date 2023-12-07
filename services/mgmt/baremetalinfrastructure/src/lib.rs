#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-preview-2023-11")]
pub mod package_preview_2023_11;
#[cfg(all(feature = "package-preview-2023-11", not(feature = "without_tag_import")))]
pub use package_preview_2023_11::*;
#[cfg(feature = "package-preview-2023-08")]
pub mod package_preview_2023_08;
#[cfg(all(feature = "package-preview-2023-08", not(feature = "without_tag_import")))]
pub use package_preview_2023_08::*;
#[cfg(feature = "package-2023-04-06")]
pub mod package_2023_04_06;
#[cfg(all(feature = "package-2023-04-06", not(feature = "without_tag_import")))]
pub use package_2023_04_06::*;
#[cfg(feature = "package-2021-08-09")]
pub mod package_2021_08_09;
#[cfg(all(feature = "package-2021-08-09", not(feature = "without_tag_import")))]
pub use package_2021_08_09::*;
#[cfg(feature = "package-2020-08-06-preview")]
pub mod package_2020_08_06_preview;
#[cfg(all(feature = "package-2020-08-06-preview", not(feature = "without_tag_import")))]
pub use package_2020_08_06_preview::*;

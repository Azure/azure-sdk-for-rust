#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-preview-2022-09")]
pub mod package_preview_2022_09;
#[cfg(all(feature = "package-preview-2022-09", not(feature = "without_tag_import")))]
pub use package_preview_2022_09::*;
#[cfg(feature = "package-preview-2021-06")]
pub mod package_preview_2021_06;
#[cfg(all(feature = "package-preview-2021-06", not(feature = "without_tag_import")))]
pub use package_preview_2021_06::*;
#[cfg(feature = "package-2022-09-preview")]
pub mod package_2022_09_preview;
#[cfg(all(feature = "package-2022-09-preview", not(feature = "without_tag_import")))]
pub use package_2022_09_preview::*;
#[cfg(feature = "package-2020-04")]
pub mod package_2020_04;
#[cfg(all(feature = "package-2020-04", not(feature = "without_tag_import")))]
pub use package_2020_04::*;
#[cfg(feature = "package-2019-05-preview")]
pub mod package_2019_05_preview;
#[cfg(all(feature = "package-2019-05-preview", not(feature = "without_tag_import")))]
pub use package_2019_05_preview::*;

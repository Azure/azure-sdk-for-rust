#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "profile-hybrid-2020-09-01")]
pub mod profile_hybrid_2020_09_01;
#[cfg(all(feature = "profile-hybrid-2020-09-01", not(feature = "without_tag_import")))]
pub use profile_hybrid_2020_09_01::*;
#[cfg(feature = "package-2023-07-preview")]
pub mod package_2023_07_preview;
#[cfg(all(feature = "package-2023-07-preview", not(feature = "without_tag_import")))]
pub use package_2023_07_preview::*;
#[cfg(feature = "package-2018-05")]
pub mod package_2018_05;
#[cfg(all(feature = "package-2018-05", not(feature = "without_tag_import")))]
pub use package_2018_05::*;
#[cfg(feature = "package-2018-03-preview")]
pub mod package_2018_03_preview;
#[cfg(all(feature = "package-2018-03-preview", not(feature = "without_tag_import")))]
pub use package_2018_03_preview::*;
#[cfg(feature = "package-2017-10")]
pub mod package_2017_10;
#[cfg(all(feature = "package-2017-10", not(feature = "without_tag_import")))]
pub use package_2017_10::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2023-01")]
pub mod package_2023_01;
#[cfg(all(feature = "package-2023-01", not(feature = "without_tag_import")))]
pub use package_2023_01::*;
#[cfg(feature = "package-2022-08")]
pub mod package_2022_08;
#[cfg(all(feature = "package-2022-08", not(feature = "without_tag_import")))]
pub use package_2022_08::*;
#[cfg(feature = "package-2022-06")]
pub mod package_2022_06;
#[cfg(all(feature = "package-2022-06", not(feature = "without_tag_import")))]
pub use package_2022_06::*;
#[cfg(feature = "package-2021-03")]
pub mod package_2021_03;
#[cfg(all(feature = "package-2021-03", not(feature = "without_tag_import")))]
pub use package_2021_03::*;
#[cfg(feature = "package-2020-02-preview")]
pub mod package_2020_02_preview;
#[cfg(all(feature = "package-2020-02-preview", not(feature = "without_tag_import")))]
pub use package_2020_02_preview::*;

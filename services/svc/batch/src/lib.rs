#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2020-09_12_0")]
pub mod package_2020_09_12_0;
#[cfg(feature = "package-2021-06_14_0")]
pub mod package_2021_06_14_0;
#[cfg(feature = "package-2022-01_15_0")]
pub mod package_2022_01_15_0;
#[cfg(feature = "package-2022-10_16_0")]
pub mod package_2022_10_16_0;
#[cfg(feature = "package-2023-05_17_0")]
pub mod package_2023_05_17_0;
#[cfg(all(feature = "default_tag", feature = "package-2023-05_17_0"))]
pub use package_2023_05_17_0::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-migrate-2018-02")]
pub mod package_migrate_2018_02;
#[cfg(feature = "package-migrate-2019-10")]
pub mod package_migrate_2019_10;
#[cfg(feature = "package-migrate-2020-01")]
pub mod package_migrate_2020_01;
#[cfg(feature = "package-migrate-2020-07")]
pub mod package_migrate_2020_07;
#[cfg(feature = "package-migrate-2023-03")]
pub mod package_migrate_2023_03;
#[cfg(all(feature = "default_tag", feature = "package-migrate-2023-03"))]
pub use package_migrate_2023_03::*;

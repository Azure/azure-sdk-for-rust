#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-managedapplications-2018-09")]
pub mod package_managedapplications_2018_09;
#[cfg(feature = "package-managedapplications-2019-07")]
pub mod package_managedapplications_2019_07;
#[cfg(feature = "package-managedapplications-2020-08")]
pub mod package_managedapplications_2020_08;
#[cfg(feature = "package-managedapplications-2021-02")]
pub mod package_managedapplications_2021_02;
#[cfg(feature = "package-managedapplications-2021-07")]
pub mod package_managedapplications_2021_07;
#[cfg(all(feature = "default_tag", feature = "package-managedapplications-2021-07"))]
pub use package_managedapplications_2021_07::*;

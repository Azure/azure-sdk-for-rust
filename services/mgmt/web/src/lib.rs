#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2022-09")]
pub mod package_2022_09;
#[cfg(all(feature = "package-2022-09", not(feature = "without_tag_import")))]
pub use package_2022_09::*;
#[cfg(feature = "package-2022-03")]
pub mod package_2022_03;
#[cfg(all(feature = "package-2022-03", not(feature = "without_tag_import")))]
pub use package_2022_03::*;
#[cfg(feature = "package-2021-03")]
pub mod package_2021_03;
#[cfg(all(feature = "package-2021-03", not(feature = "without_tag_import")))]
pub use package_2021_03::*;
#[cfg(feature = "package-2021-02")]
pub mod package_2021_02;
#[cfg(all(feature = "package-2021-02", not(feature = "without_tag_import")))]
pub use package_2021_02::*;
#[cfg(feature = "package-2021-01-15")]
pub mod package_2021_01_15;
#[cfg(all(feature = "package-2021-01-15", not(feature = "without_tag_import")))]
pub use package_2021_01_15::*;

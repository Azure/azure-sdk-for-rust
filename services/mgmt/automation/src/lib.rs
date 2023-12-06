#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2022-08-08")]
pub mod package_2022_08_08;
#[cfg(all(feature = "package-2022-08-08", not(feature = "without_tag_import")))]
pub use package_2022_08_08::*;
#[cfg(feature = "package-2022-02-22")]
pub mod package_2022_02_22;
#[cfg(all(feature = "package-2022-02-22", not(feature = "without_tag_import")))]
pub use package_2022_02_22::*;
#[cfg(feature = "package-2022-01-31")]
pub mod package_2022_01_31;
#[cfg(all(feature = "package-2022-01-31", not(feature = "without_tag_import")))]
pub use package_2022_01_31::*;
#[cfg(feature = "package-2021-06-22")]
pub mod package_2021_06_22;
#[cfg(all(feature = "package-2021-06-22", not(feature = "without_tag_import")))]
pub use package_2021_06_22::*;
#[cfg(feature = "package-2020-01-13-preview")]
pub mod package_2020_01_13_preview;
#[cfg(all(feature = "package-2020-01-13-preview", not(feature = "without_tag_import")))]
pub use package_2020_01_13_preview::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-preview-2019-05")]
pub mod package_preview_2019_05;
#[cfg(all(feature = "package-preview-2019-05", not(feature = "without_tag_import")))]
pub use package_preview_2019_05::*;
#[cfg(feature = "package-2023-07")]
pub mod package_2023_07;
#[cfg(all(feature = "package-2023-07", not(feature = "without_tag_import")))]
pub use package_2023_07::*;
#[cfg(feature = "package-2022-07")]
pub mod package_2022_07;
#[cfg(all(feature = "package-2022-07", not(feature = "without_tag_import")))]
pub use package_2022_07::*;
#[cfg(feature = "package-2022-02")]
pub mod package_2022_02;
#[cfg(all(feature = "package-2022-02", not(feature = "without_tag_import")))]
pub use package_2022_02::*;
#[cfg(feature = "package-2021-10")]
pub mod package_2021_10;
#[cfg(all(feature = "package-2021-10", not(feature = "without_tag_import")))]
pub use package_2021_10::*;

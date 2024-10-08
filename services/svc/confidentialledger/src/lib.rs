#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2023-06-01-preview-mccf")]
pub mod package_2023_06_01_preview_mccf;
#[cfg(feature = "package-2024-01-26-preview-identity")]
pub mod package_2024_01_26_preview_identity;
#[cfg(feature = "package-2024-01-26-preview-ledger")]
pub mod package_2024_01_26_preview_ledger;
#[cfg(feature = "package-2024-08-22-preview-identity")]
pub mod package_2024_08_22_preview_identity;
#[cfg(feature = "package-2024-08-22-preview-ledger")]
pub mod package_2024_08_22_preview_ledger;
#[cfg(all(feature = "default_tag", feature = "package-2024-08-22-preview-ledger"))]
pub use package_2024_08_22_preview_ledger::*;

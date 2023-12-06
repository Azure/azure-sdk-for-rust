#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2023-06-01-preview-mccf")]
pub mod package_2023_06_01_preview_mccf;
#[cfg(all(feature = "package-2023-06-01-preview-mccf", not(feature = "without_tag_import")))]
pub use package_2023_06_01_preview_mccf::*;
#[cfg(feature = "package-2023-01-18-preview-ledger")]
pub mod package_2023_01_18_preview_ledger;
#[cfg(all(feature = "package-2023-01-18-preview-ledger", not(feature = "without_tag_import")))]
pub use package_2023_01_18_preview_ledger::*;
#[cfg(feature = "package-2023-01-18-preview-identity")]
pub mod package_2023_01_18_preview_identity;
#[cfg(all(feature = "package-2023-01-18-preview-identity", not(feature = "without_tag_import")))]
pub use package_2023_01_18_preview_identity::*;
#[cfg(feature = "package-2022-05-13-ledger")]
pub mod package_2022_05_13_ledger;
#[cfg(all(feature = "package-2022-05-13-ledger", not(feature = "without_tag_import")))]
pub use package_2022_05_13_ledger::*;
#[cfg(feature = "package-2022-05-13-identity")]
pub mod package_2022_05_13_identity;
#[cfg(all(feature = "package-2022-05-13-identity", not(feature = "without_tag_import")))]
pub use package_2022_05_13_identity::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "profile-hybrid-2020-09-01")]
pub mod profile_hybrid_2020_09_01;
#[cfg(all(feature = "profile-hybrid-2020-09-01", not(feature = "without_tag_import")))]
pub use profile_hybrid_2020_09_01::*;
#[cfg(feature = "package-2023-08-preview")]
pub mod package_2023_08_preview;
#[cfg(all(feature = "package-2023-08-preview", not(feature = "without_tag_import")))]
pub use package_2023_08_preview::*;
#[cfg(feature = "package-2023-07-preview")]
pub mod package_2023_07_preview;
#[cfg(all(feature = "package-2023-07-preview", not(feature = "without_tag_import")))]
pub use package_2023_07_preview::*;
#[cfg(feature = "package-2023-06")]
pub mod package_2023_06;
#[cfg(all(feature = "package-2023-06", not(feature = "without_tag_import")))]
pub use package_2023_06::*;
#[cfg(feature = "package-2023-05")]
pub mod package_2023_05;
#[cfg(all(feature = "package-2023-05", not(feature = "without_tag_import")))]
pub use package_2023_05::*;

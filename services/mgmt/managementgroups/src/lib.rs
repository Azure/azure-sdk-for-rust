#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2023-04")]
pub mod package_2023_04;
#[cfg(all(feature = "package-2023-04", not(feature = "without_tag_import")))]
pub use package_2023_04::*;
#[cfg(feature = "package-2021-04")]
pub mod package_2021_04;
#[cfg(all(feature = "package-2021-04", not(feature = "without_tag_import")))]
pub use package_2021_04::*;
#[cfg(feature = "package-2020-10")]
pub mod package_2020_10;
#[cfg(all(feature = "package-2020-10", not(feature = "without_tag_import")))]
pub use package_2020_10::*;
#[cfg(feature = "package-2020-05")]
pub mod package_2020_05;
#[cfg(all(feature = "package-2020-05", not(feature = "without_tag_import")))]
pub use package_2020_05::*;
#[cfg(feature = "package-2020-02")]
pub mod package_2020_02;
#[cfg(all(feature = "package-2020-02", not(feature = "without_tag_import")))]
pub use package_2020_02::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#[cfg(feature = "package-2023-11")]
pub mod package_2023_11;
#[cfg(all(feature = "package-2023-11", not(feature = "without_tag_import")))]
pub use package_2023_11::*;
#[cfg(feature = "package-2022-09")]
pub mod package_2022_09;
#[cfg(all(feature = "package-2022-09", not(feature = "without_tag_import")))]
pub use package_2022_09::*;
#[cfg(feature = "package-2021-04-preview")]
pub mod package_2021_04_preview;
#[cfg(all(feature = "package-2021-04-preview", not(feature = "without_tag_import")))]
pub use package_2021_04_preview::*;
#[cfg(feature = "package-2020-08-preview")]
pub mod package_2020_08_preview;
#[cfg(all(feature = "package-2020-08-preview", not(feature = "without_tag_import")))]
pub use package_2020_08_preview::*;
#[cfg(feature = "package-2020-08")]
pub mod package_2020_08;
#[cfg(all(feature = "package-2020-08", not(feature = "without_tag_import")))]
pub use package_2020_08::*;

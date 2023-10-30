#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-migrate-2023-03")]
pub mod package_migrate_2023_03;
#[cfg(all(feature = "package-migrate-2023-03", not(feature = "no-default-tag")))]
pub use package_migrate_2023_03::*;
#[cfg(feature = "package-migrate-2020-07")]
pub mod package_migrate_2020_07;
#[cfg(all(feature = "package-migrate-2020-07", not(feature = "no-default-tag")))]
pub use package_migrate_2020_07::*;
#[cfg(feature = "package-migrate-2020-01")]
pub mod package_migrate_2020_01;
#[cfg(all(feature = "package-migrate-2020-01", not(feature = "no-default-tag")))]
pub use package_migrate_2020_01::*;
#[cfg(feature = "package-migrate-2019-10")]
pub mod package_migrate_2019_10;
#[cfg(all(feature = "package-migrate-2019-10", not(feature = "no-default-tag")))]
pub use package_migrate_2019_10::*;
#[cfg(feature = "package-migrate-2018-02")]
pub mod package_migrate_2018_02;
#[cfg(all(feature = "package-migrate-2018-02", not(feature = "no-default-tag")))]
pub use package_migrate_2018_02::*;

#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-composite-v1")]
pub mod package_composite_v1;
#[cfg(all(feature = "package-composite-v1", not(feature = "no-default-tag")))]
pub use package_composite_v1::{models, Client, ClientBuilder};
#[cfg(feature = "package-2021-03-03")]
pub mod package_2021_03_03;
#[cfg(all(feature = "package-2021-03-03", not(feature = "no-default-tag")))]
pub use package_2021_03_03::{models, Client, ClientBuilder};

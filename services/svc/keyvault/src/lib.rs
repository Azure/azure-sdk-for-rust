#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "package-preview-7_3-preview")]
pub mod package_preview_7_3_preview;
#[cfg(all(feature = "package-preview-7_3-preview", not(feature = "no-default-tag")))]
pub use package_preview_7_3_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-7_3")]
pub mod package_7_3;
#[cfg(all(feature = "package-7_3", not(feature = "no-default-tag")))]
pub use package_7_3::{models, Client, ClientBuilder};
#[cfg(feature = "package-7_2-preview")]
pub mod package_7_2_preview;
#[cfg(all(feature = "package-7_2-preview", not(feature = "no-default-tag")))]
pub use package_7_2_preview::{models, Client, ClientBuilder};
#[cfg(feature = "package-7_2")]
pub mod package_7_2;
#[cfg(all(feature = "package-7_2", not(feature = "no-default-tag")))]
pub use package_7_2::{models, Client, ClientBuilder};
#[cfg(feature = "package-7_1-preview")]
pub mod package_7_1_preview;
#[cfg(all(feature = "package-7_1-preview", not(feature = "no-default-tag")))]
pub use package_7_1_preview::{models, Client, ClientBuilder};

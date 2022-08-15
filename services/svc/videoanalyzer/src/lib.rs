#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "package-preview-1_1_0")]
pub mod package_preview_1_1_0;
#[cfg(all(feature = "package-preview-1_1_0", not(feature = "no-default-tag")))]
pub use package_preview_1_1_0::{models, Client, ClientBuilder};
#[cfg(feature = "package-ava-edge-1-0-0-preview")]
pub mod package_ava_edge_1_0_0_preview;
#[cfg(all(feature = "package-ava-edge-1-0-0-preview", not(feature = "no-default-tag")))]
pub use package_ava_edge_1_0_0_preview::{models, Client, ClientBuilder};

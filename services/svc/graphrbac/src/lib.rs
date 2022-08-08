#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "1_6")]
pub mod v1_6;
#[cfg(all(feature = "1_6", not(feature = "no-default-tag")))]
pub use v1_6::{models, Client, ClientBuilder};

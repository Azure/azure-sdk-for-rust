#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#[cfg(feature = "8_2")]
pub mod v8_2;
#[cfg(all(feature = "8_2", not(feature = "no-default-tag")))]
pub use v8_2::{models, Client, ClientBuilder};
#[cfg(feature = "8_1")]
pub mod v8_1;
#[cfg(all(feature = "8_1", not(feature = "no-default-tag")))]
pub use v8_1::{models, Client, ClientBuilder};
#[cfg(feature = "8_0")]
pub mod v8_0;
#[cfg(all(feature = "8_0", not(feature = "no-default-tag")))]
pub use v8_0::{models, Client, ClientBuilder};
#[cfg(feature = "7_2")]
pub mod v7_2;
#[cfg(all(feature = "7_2", not(feature = "no-default-tag")))]
pub use v7_2::{models, Client, ClientBuilder};
#[cfg(feature = "7_1")]
pub mod v7_1;
#[cfg(all(feature = "7_1", not(feature = "no-default-tag")))]
pub use v7_1::{models, Client, ClientBuilder};

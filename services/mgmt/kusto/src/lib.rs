#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#[cfg(feature = "schema-2019-09-07")]
pub mod schema_2019_09_07;
#[cfg(all(feature = "schema-2019-09-07", not(feature = "no-default-tag")))]
pub use schema_2019_09_07::{models, Client, ClientBuilder};
#[cfg(feature = "schema-2019-05-15")]
pub mod schema_2019_05_15;
#[cfg(all(feature = "schema-2019-05-15", not(feature = "no-default-tag")))]
pub use schema_2019_05_15::{models, Client, ClientBuilder};
#[cfg(feature = "schema-2019-01-21")]
pub mod schema_2019_01_21;
#[cfg(all(feature = "schema-2019-01-21", not(feature = "no-default-tag")))]
pub use schema_2019_01_21::{models, Client, ClientBuilder};
#[cfg(feature = "schema-2018-09-07-preview")]
pub mod schema_2018_09_07_preview;
#[cfg(all(feature = "schema-2018-09-07-preview", not(feature = "no-default-tag")))]
pub use schema_2018_09_07_preview::{models, Client, ClientBuilder};
#[cfg(feature = "schema-2017-09-07-privatepreview")]
pub mod schema_2017_09_07_privatepreview;
#[cfg(all(feature = "schema-2017-09-07-privatepreview", not(feature = "no-default-tag")))]
pub use schema_2017_09_07_privatepreview::{models, Client, ClientBuilder};

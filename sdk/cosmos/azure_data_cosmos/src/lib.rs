#![doc = include_str!("../README.md")]
// Docs.rs build is done with the nightly compiler, so we can enable nightly features in that build.
// In this case we enable two features:
// - `doc_auto_cfg`: Automatically scans `cfg` attributes and uses them to show those required configurations in the generated documentation.
// - `doc_cfg_hide`: Ignore the `doc` configuration for `doc_auto_cfg`.
// See https://doc.rust-lang.org/rustdoc/unstable-features.html#doc_auto_cfg-automatically-generate-doccfg for more details.
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(docsrs, feature(doc_cfg_hide))]

pub mod clients;
pub(crate) mod constants;
mod options;
mod partition_key;
pub(crate) mod pipeline;
mod query;
pub(crate) mod utils;

/// Model types sent to and received from the Cosmos DB API.
pub mod models;

pub use clients::{CosmosClient, CosmosClientMethods};
pub use options::*;
pub use partition_key::*;
pub use query::*;

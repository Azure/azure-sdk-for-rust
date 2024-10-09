#![doc = include_str!("../README.md")]
#![allow(clippy::enum_variant_names)]
#![allow(clippy::new_without_default)]
#![allow(clippy::module_inception)]
#![warn(unused_extern_crates)]
#![deny(missing_docs)]
#![recursion_limit = "256"]
#[macro_use]
extern crate serde;
#[macro_use]
extern crate azure_core;

pub mod clients;
mod operations;
pub mod prelude;
pub mod resources;

mod authorization_policy;
mod consistency_level;
mod cosmos_entity;
mod headers;
mod resource_quota;

pub(crate) use authorization_policy::AuthorizationPolicy;

pub use consistency_level::ConsistencyLevel;
pub use cosmos_entity::CosmosEntity;
pub use resource_quota::ResourceQuota;

type ReadonlyString = std::borrow::Cow<'static, str>;

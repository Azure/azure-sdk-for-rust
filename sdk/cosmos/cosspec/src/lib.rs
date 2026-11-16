// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Backend-agnostic specification tests for the Azure Cosmos DB SDK for Rust.
//!
//! A spec test is a short `async fn` that states one SDK behavior. Resources are
//! injected through parameters, capabilities are declared up front, and the runner
//! composes every spec with every compatible service profile on each configured
//! backend (see [`config::SpecConfig`]).
//!
//! ```rust,ignore
//! use cosspec::{assert_items_unordered, spec_test, testdata, ExclusiveContainer, SpecResult};
//!
//! /// A full-container query returns every item exactly once.
//! #[spec_test]
//! pub async fn query_returns_every_item(
//!     #[profile(basic_items)] container: ExclusiveContainer,
//! ) -> SpecResult {
//!     let items = container.query_all("SELECT * FROM c").await?;
//!     assert_items_unordered!(items, testdata::items::basic_items().all());
//!     Ok(())
//! }
//! ```
//!
//! See `sdk/cosmos/docs/specs/0030-cosmos-spec-tests.md` for the design.

mod args;
mod assertions;
pub mod backend;
mod capability;
pub mod config;
mod console;
mod context;
pub mod faults;
pub mod http_tap;
mod logging;
pub mod manifest;
mod pool;
pub mod profiles;
pub mod provision;
mod regions;
mod resources;
pub mod results;
mod runner;
pub mod testdata;
pub mod trace_capture;

pub use capability::Capability;
pub use context::SpecContext;
pub use cosspec_macros::spec_test;
pub use http_tap::HttpTap;
pub use profiles::{ContainerProfile, ServiceProfile};
pub use regions::{RegionAlias, RegionMap};
pub use resources::{ExclusiveAccount, ExclusiveContainer, ExclusiveDatabase, SharedContainer};
pub use runner::main;

/// The error type returned by spec tests.
pub type SpecError = Box<dyn std::error::Error + Send + Sync>;

/// The result type returned by spec tests.
pub type SpecResult<T = ()> = Result<T, SpecError>;

/// Raised when a spec uses the framework in a way its declaration does not allow.
///
/// Examples: calling [`ExclusiveContainer::split`] without
/// `#[requires(TopologyControl)]`, or writing to a [`SharedContainer`].
#[derive(Debug)]
pub struct InvalidConfiguration(pub String);

impl std::fmt::Display for InvalidConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid spec configuration: {}", self.0)
    }
}

impl std::error::Error for InvalidConfiguration {}

#[doc(hidden)]
pub mod __private {
    pub use crate::context::{Inject, ParamKind};
    pub use crate::runner::{KnownIssue, ParamDescriptor, SpecDescriptor, SPECS};
    pub use linkme;
}

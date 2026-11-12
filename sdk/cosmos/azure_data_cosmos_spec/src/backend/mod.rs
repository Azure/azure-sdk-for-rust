// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Backends: where specs run.
//!
//! A [`Backend`] opens one pooled [`BackendAccount`] per service profile. The
//! account builds SDK clients, provisions resources, and realizes demands such as
//! partition splits. Specs never see the backend directly.

mod in_memory;

pub use in_memory::InMemoryBackend;

use crate::{provision::Provisioner, Capability, ServiceProfile, SpecResult};
use async_trait::async_trait;
use azure_data_cosmos::{fault_injection::FaultInjectionRule, options::Region, CosmosClient};
use azure_data_cosmos_driver::transport_observer::TransportObserver;
use std::{fmt::Debug, sync::Arc};

/// The environment variable that selects the backend. Defaults to `inmemory`.
pub const BACKEND_ENV: &str = "AZURE_COSMOS_SPEC_BACKEND";

/// Options for a client a spec will use.
#[derive(Clone, Debug, Default)]
pub struct ClientOptions {
    /// Observes every HTTP exchange the client makes.
    pub observer: Option<Arc<dyn TransportObserver>>,
    /// Client-side fault injection rules.
    pub faults: Vec<Arc<FaultInjectionRule>>,
}

/// A place specs can run: the in-memory emulator, a hosted emulator, or a live account.
#[async_trait]
pub trait Backend: Send + Sync + Debug {
    /// The name used in test names and `AZURE_COSMOS_SPEC_BACKEND`.
    fn name(&self) -> &'static str;

    /// Whether this backend can provide `profile` at all.
    fn supports(&self, profile: ServiceProfile) -> bool;

    /// The capabilities available for `profile`, including the profile's own.
    fn capabilities(&self, profile: ServiceProfile) -> Vec<Capability>;

    /// Opens an account shaped like `profile`. Called once per profile per run
    /// for the pooled account, and once per spec for an `ExclusiveAccount`.
    async fn open_account(&self, profile: ServiceProfile) -> SpecResult<Arc<dyn BackendAccount>>;
}

/// An account opened by a [`Backend`].
#[async_trait]
pub trait BackendAccount: Send + Sync + Debug {
    /// The account's service profile.
    fn profile(&self) -> ServiceProfile;

    /// Builds a new client for this account.
    async fn client(&self, options: ClientOptions) -> SpecResult<CosmosClient>;

    /// The account's provisioner.
    fn provisioner(&self) -> &dyn Provisioner;

    /// Partition topology control, if the backend supports it.
    fn topology(&self) -> Option<&dyn TopologyControl> {
        None
    }

    /// Region control, if the backend supports it.
    fn regions(&self) -> Option<&dyn RegionControl> {
        None
    }
}

/// Changes a container's physical partition layout.
#[async_trait]
pub trait TopologyControl: Send + Sync {
    /// Splits every physical partition of a container in two and waits for the
    /// split to finish.
    async fn split_all(&self, database: &str, container: &str) -> SpecResult;
}

/// Changes which of an account's regions are serving.
#[async_trait]
pub trait RegionControl: Send + Sync {
    /// Takes a region offline. Requests to it fail as they would during an outage.
    async fn set_offline(&self, region: &Region) -> SpecResult;

    /// Brings an offline region back.
    async fn set_online(&self, region: &Region) -> SpecResult;
}

/// The backend selected by `AZURE_COSMOS_SPEC_BACKEND`.
pub fn selected() -> SpecResult<Arc<dyn Backend>> {
    match std::env::var(BACKEND_ENV).as_deref() {
        Err(_) | Ok("") | Ok("inmemory") => Ok(Arc::new(InMemoryBackend)),
        Ok(other) => Err(format!("unknown {BACKEND_ENV} '{other}'; supported: inmemory").into()),
    }
}

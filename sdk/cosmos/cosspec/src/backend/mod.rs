// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Backends: where specs run.
//!
//! A [`Backend`] opens one pooled [`BackendAccount`] per service profile. The
//! account builds SDK clients, provisions resources, and realizes demands such as
//! partition splits. Specs never see the backend directly.
//!
//! The run configuration ([`SpecConfig`](crate::config::SpecConfig)) picks the
//! backends and the service profiles each one runs.

mod emulator;
mod in_memory;
mod live;
mod remote;

pub use emulator::EmulatorBackend;
pub use in_memory::InMemoryBackend;
pub use live::LiveBackend;

use crate::{
    provision::Provisioner, Capability, InvalidConfiguration, RegionMap, ServiceProfile, SpecResult,
};
use async_trait::async_trait;
use azure_data_cosmos::{
    fault_injection::FaultInjectionRule, feed::FeedRange, options::Region, CosmosClient,
};
use azure_data_cosmos_driver::{
    models::EffectivePartitionKey, transport_observer::TransportObserver,
};
use std::{fmt::Debug, sync::Arc};

/// Options for a client a spec will use.
#[derive(Clone, Debug, Default)]
pub struct ClientOptions {
    /// Observes every HTTP exchange the client makes.
    pub observer: Option<Arc<dyn TransportObserver>>,
    /// Client-side fault injection rules.
    pub faults: Vec<Arc<FaultInjectionRule>>,
}

/// A place specs can run: the in-memory emulator, the Azure Cosmos DB Emulator, or live accounts.
#[async_trait]
pub trait Backend: Send + Sync + Debug {
    /// The backend's key under `[backends]` in the run configuration, such as `inmemory`.
    fn kind(&self) -> &'static str;

    /// The name used in test names and result files. Defaults to [`kind`](Backend::kind).
    fn name(&self) -> &str {
        self.kind()
    }

    /// The service profiles this backend runs, in order.
    fn profiles(&self) -> Vec<ServiceProfile>;

    /// Whether this backend runs `profile`.
    fn supports(&self, profile: ServiceProfile) -> bool {
        self.profiles().contains(&profile)
    }

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

    /// The account endpoint, recorded in spec results.
    fn endpoint(&self) -> String;

    /// Builds a new client for this account.
    async fn client(&self, options: ClientOptions) -> SpecResult<CosmosClient>;

    /// The account's provisioner.
    fn provisioner(&self) -> &dyn Provisioner;

    /// Partition topology control, if the backend supports it.
    fn topology(&self) -> Option<&dyn TopologyControl> {
        None
    }

    /// Simple one-to-two splits and two-to-one merges, if the backend supports them.
    ///
    /// Defaults to [`ViaTopology`] when the account has [`TopologyControl`].
    fn simple_split_merge(&self) -> Option<Box<dyn SimpleSplitMerge + '_>> {
        self.topology()
            .map(|topology| Box::new(ViaTopology(topology)) as Box<dyn SimpleSplitMerge + '_>)
    }

    /// The real region behind each of the account's region aliases.
    fn regions(&self) -> &RegionMap;

    /// Region control, if the backend supports it.
    fn region_control(&self) -> Option<&dyn RegionControl> {
        None
    }
}

/// Changes a container's physical partition layout arbitrarily.
///
/// Physical partitions are identified by their EPK ranges, as [`FeedRange`]s.
#[async_trait]
pub trait TopologyControl: Send + Sync {
    /// The container's physical partitions, in EPK order.
    async fn partitions(&self, database: &str, container: &str) -> SpecResult<Vec<FeedRange>>;

    /// Splits the physical partition whose range is exactly `partition` at `at`, or
    /// at its midpoint when `at` is `None`, and waits for the split to finish.
    async fn split(
        &self,
        database: &str,
        container: &str,
        partition: &FeedRange,
        at: Option<EffectivePartitionKey>,
    ) -> SpecResult;

    /// Merges two adjacent physical partitions into one and waits for the merge
    /// to finish.
    async fn merge(
        &self,
        database: &str,
        container: &str,
        left: &FeedRange,
        right: &FeedRange,
    ) -> SpecResult;

    /// Splits every physical partition at its midpoint.
    async fn split_all(&self, database: &str, container: &str) -> SpecResult {
        for partition in self.partitions(database, container).await? {
            self.split(database, container, &partition, None).await?;
        }
        Ok(())
    }
}

/// The simplest topology changes: one partition into two, and two back into one.
///
/// This is deliberately narrow so backends without arbitrary [`TopologyControl`],
/// such as live accounts, can provide it too (for example, by scaling throughput).
#[async_trait]
pub trait SimpleSplitMerge: Send + Sync {
    /// Splits a container with one physical partition into two and waits for the
    /// split to finish.
    async fn simple_partition_split(&self, database: &str, container: &str) -> SpecResult;

    /// Merges a container with two physical partitions into one and waits for the
    /// merge to finish.
    async fn simple_partition_merge(&self, database: &str, container: &str) -> SpecResult;
}

/// [`SimpleSplitMerge`] built on [`TopologyControl`].
pub struct ViaTopology<'a>(pub &'a dyn TopologyControl);

#[async_trait]
impl SimpleSplitMerge for ViaTopology<'_> {
    async fn simple_partition_split(&self, database: &str, container: &str) -> SpecResult {
        let partitions = self.0.partitions(database, container).await?;
        let [partition] = partitions.as_slice() else {
            return Err(InvalidConfiguration(format!(
                "a simple split needs one physical partition, but the container has {}",
                partitions.len()
            ))
            .into());
        };
        self.0.split(database, container, partition, None).await
    }

    async fn simple_partition_merge(&self, database: &str, container: &str) -> SpecResult {
        let partitions = self.0.partitions(database, container).await?;
        let [left, right] = partitions.as_slice() else {
            return Err(InvalidConfiguration(format!(
                "a simple merge needs two physical partitions, but the container has {}",
                partitions.len()
            ))
            .into());
        };
        self.0.merge(database, container, left, right).await
    }
}

/// Changes which of an account's regions are serving.
#[async_trait]
pub trait RegionControl: Send + Sync {
    /// Takes a region offline. Requests to it fail as they would during an outage.
    async fn set_offline(&self, region: &Region) -> SpecResult;

    /// Brings an offline region back.
    async fn set_online(&self, region: &Region) -> SpecResult;
}

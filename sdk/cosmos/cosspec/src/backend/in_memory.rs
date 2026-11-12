// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The in-process in-memory emulator backend.

use super::{Backend, BackendAccount, ClientOptions, RegionControl, TopologyControl};
use crate::{
    pool,
    profiles::ContainerProfile,
    provision::{ControlPlaneProvisioner, Provisioner},
    Capability, ServiceProfile, SpecResult,
};
use async_trait::async_trait;
use azure_core::{credentials::Secret, http::Url};
use azure_data_cosmos::{
    feed::FeedRange, options::Region, AccountEndpoint, AccountReference, CosmosClient,
    CosmosClientBuilder, CosmosRuntimeBuilder, RoutingStrategy,
};
use azure_data_cosmos_driver::{
    in_memory_emulator::{
        ContainerConfig, InMemoryEmulatorHttpClient, ReplicationConfig, VirtualAccountConfig,
        VirtualRegion, WriteMode,
    },
    models::EffectivePartitionKey,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};

const KEY: &str = "dGVzdGtleQ==";

/// Runs specs against an in-process in-memory emulator.
///
/// Every account is a fresh emulator with immediate replication, so specs are
/// deterministic and need no network. It provides every service profile and
/// every capability.
#[derive(Debug)]
pub struct InMemoryBackend {
    name: String,
    profiles: Vec<ServiceProfile>,
}

impl InMemoryBackend {
    /// Creates a backend named `name` that runs `profiles`.
    pub fn new(name: impl Into<String>, profiles: Vec<ServiceProfile>) -> Self {
        Self {
            name: name.into(),
            profiles,
        }
    }
}

impl Default for InMemoryBackend {
    /// The `inmemory` backend with every service profile.
    fn default() -> Self {
        Self::new("inmemory", ServiceProfile::ALL.to_vec())
    }
}

#[async_trait]
impl Backend for InMemoryBackend {
    fn name(&self) -> &str {
        &self.name
    }

    fn kind(&self) -> &'static str {
        "inmemory"
    }

    fn profiles(&self) -> Vec<ServiceProfile> {
        self.profiles.clone()
    }

    fn capabilities(&self, profile: ServiceProfile) -> Vec<Capability> {
        let mut capabilities = vec![
            Capability::TopologyControl,
            Capability::SimpleSplitMerge,
            Capability::RegionControl,
            Capability::AccountIsolation,
        ];
        capabilities.extend_from_slice(profile.capabilities());
        capabilities
    }

    async fn open_account(&self, profile: ServiceProfile) -> SpecResult<Arc<dyn BackendAccount>> {
        let regions = profile.regions();
        let virtual_regions = regions
            .iter()
            .map(|region| {
                Ok(VirtualRegion::new(
                    region.display_name(),
                    Url::parse(&format!("https://{}.spec.local", region.as_str()))?,
                ))
            })
            .collect::<SpecResult<Vec<_>>>()?;
        let endpoint = virtual_regions[0].gateway_url().to_string();
        let write_mode = match profile {
            ServiceProfile::MultiWrite => WriteMode::Multi,
            _ => WriteMode::Single,
        };
        let config = VirtualAccountConfig::new(virtual_regions)?
            .with_write_mode(write_mode)
            .with_replication_config(ReplicationConfig::immediate());
        let emulator = Arc::new(InMemoryEmulatorHttpClient::new(config));
        Ok(Arc::new(InMemoryAccount {
            profile,
            regions,
            endpoint,
            emulator,
            partitions: Mutex::default(),
        }))
    }
}

#[derive(Debug)]
struct InMemoryAccount {
    profile: ServiceProfile,
    regions: Vec<Region>,
    endpoint: String,
    emulator: Arc<InMemoryEmulatorHttpClient>,
    /// Current physical partitions per `(database, container)`, in EPK order.
    partitions: Mutex<HashMap<(String, String), Vec<Partition>>>,
}

/// An emulator physical partition: its store ID and EPK range.
#[derive(Clone, Debug)]
struct Partition {
    id: u32,
    range: FeedRange,
}

impl InMemoryAccount {
    fn key(database: &str, container: &str) -> (String, String) {
        (database.to_owned(), container.to_owned())
    }

    fn tracked(&self, database: &str, container: &str) -> SpecResult<Vec<Partition>> {
        self.partitions
            .lock()
            .unwrap()
            .get(&Self::key(database, container))
            .cloned()
            .ok_or_else(|| {
                format!("container '{container}' was not created by this backend").into()
            })
    }

    fn find(partitions: &[Partition], range: &FeedRange) -> SpecResult<usize> {
        partitions
            .iter()
            .position(|p| p.range == *range)
            .ok_or_else(|| format!("no physical partition has the range {range}").into())
    }
}

#[async_trait]
impl BackendAccount for InMemoryAccount {
    fn profile(&self) -> ServiceProfile {
        self.profile
    }

    fn endpoint(&self) -> String {
        self.endpoint.clone()
    }

    async fn client(&self, options: ClientOptions) -> SpecResult<CosmosClient> {
        let runtime = CosmosRuntimeBuilder::from(self.emulator.runtime_builder())
            .build()
            .await?;
        let mut builder = CosmosClientBuilder::new().with_runtime(runtime);
        if let Some(observer) = options.observer {
            builder = builder.with_transport_observer(observer);
        }
        if !options.faults.is_empty() {
            builder = builder.with_fault_injection_rules(options.faults)?;
        }
        let account = AccountReference::with_authentication_key(
            self.endpoint.parse::<AccountEndpoint>()?,
            Secret::new(KEY),
        );
        let routing = RoutingStrategy::PreferredRegions(self.regions.clone());
        Ok(Box::pin(builder.build(account, routing)).await?)
    }

    fn provisioner(&self) -> &dyn Provisioner {
        self
    }

    fn topology(&self) -> Option<&dyn TopologyControl> {
        Some(self)
    }

    fn regions(&self) -> Option<&dyn RegionControl> {
        Some(self)
    }
}

#[async_trait]
impl Provisioner for InMemoryAccount {
    async fn create_database(&self, client: &CosmosClient, database: &str) -> SpecResult {
        ControlPlaneProvisioner
            .create_database(client, database)
            .await
    }

    async fn delete_database(&self, client: &CosmosClient, database: &str) -> SpecResult {
        ControlPlaneProvisioner
            .delete_database(client, database)
            .await
    }

    /// Creates the container directly in the emulator store so the physical
    /// partition count is exact.
    async fn create_container(
        &self,
        client: &CosmosClient,
        database: &str,
        container: &str,
        profile: &ContainerProfile,
    ) -> SpecResult {
        let partition_key = serde_json::from_value(serde_json::json!({
            "paths": [profile.partition_key_path],
            "kind": "Hash",
            "version": 2,
        }))?;
        let config = ContainerConfig::new()
            .with_partition_count(profile.physical_partitions)
            .build()?;
        self.emulator.store().create_container_with_config(
            database,
            container,
            partition_key,
            config,
        );
        // The store numbers a new container's partitions from 0 in EPK order.
        let mut ranges = Box::pin(
            pool::container_client(client, database, container)
                .await?
                .read_feed_ranges(None),
        )
        .await?;
        ranges.sort_by(|a, b| a.min_inclusive().cmp(b.min_inclusive()));
        let partitions = (0..).zip(ranges).map(|(id, range)| Partition { id, range });
        self.partitions
            .lock()
            .unwrap()
            .insert(Self::key(database, container), partitions.collect());
        Ok(())
    }

    async fn delete_container(
        &self,
        client: &CosmosClient,
        database: &str,
        container: &str,
    ) -> SpecResult {
        self.partitions
            .lock()
            .unwrap()
            .remove(&Self::key(database, container));
        ControlPlaneProvisioner
            .delete_container(client, database, container)
            .await
    }

    async fn list_databases(&self, client: &CosmosClient) -> SpecResult<Vec<String>> {
        ControlPlaneProvisioner.list_databases(client).await
    }
}

#[async_trait]
impl TopologyControl for InMemoryAccount {
    async fn partitions(&self, database: &str, container: &str) -> SpecResult<Vec<FeedRange>> {
        Ok(self
            .tracked(database, container)?
            .into_iter()
            .map(|p| p.range)
            .collect())
    }

    async fn split(
        &self,
        database: &str,
        container: &str,
        partition: &FeedRange,
        at: Option<EffectivePartitionKey>,
    ) -> SpecResult {
        let mut partitions = self.tracked(database, container)?;
        let index = Self::find(&partitions, partition)?;
        let parent = partitions[index].id;
        let store = self.emulator.store();
        let boundary = match at {
            Some(at) => {
                store.validate_split_epk(database, container, parent, &at)?;
                at
            }
            None => store.midpoint_split_epk(database, container, parent)?,
        };
        let children = store
            .split_partition_and_wait(
                database,
                container,
                parent,
                boundary.clone(),
                Duration::ZERO,
            )
            .await?;
        // The first child takes the lower half of the parent's range.
        let [lower, upper] = children.as_slice() else {
            return Err(format!("split produced {} partitions, not 2", children.len()).into());
        };
        let lower = Partition {
            id: *lower,
            range: FeedRange::new(partition.min_inclusive().clone(), boundary.clone())?,
        };
        let upper = Partition {
            id: *upper,
            range: FeedRange::new(boundary, partition.max_exclusive().clone())?,
        };
        partitions.splice(index..=index, [lower, upper]);
        self.partitions
            .lock()
            .unwrap()
            .insert(Self::key(database, container), partitions);
        Ok(())
    }

    async fn merge(
        &self,
        database: &str,
        container: &str,
        left: &FeedRange,
        right: &FeedRange,
    ) -> SpecResult {
        let mut partitions = self.tracked(database, container)?;
        let index = Self::find(&partitions, left)?;
        if partitions.get(index + 1).map(|p| &p.range) != Some(right) {
            return Err(format!("{left} and {right} are not adjacent physical partitions").into());
        }
        let child = self
            .emulator
            .store()
            .merge_partitions_and_wait(
                database,
                container,
                partitions[index].id,
                partitions[index + 1].id,
                Duration::ZERO,
            )
            .await?;
        let merged = Partition {
            id: child,
            range: FeedRange::new(left.min_inclusive().clone(), right.max_exclusive().clone())?,
        };
        partitions.splice(index..=index + 1, [merged]);
        self.partitions
            .lock()
            .unwrap()
            .insert(Self::key(database, container), partitions);
        Ok(())
    }
}

#[async_trait]
impl RegionControl for InMemoryAccount {
    async fn set_offline(&self, region: &Region) -> SpecResult {
        Ok(self
            .emulator
            .store()
            .set_region_offline(region.display_name())?)
    }

    async fn set_online(&self, region: &Region) -> SpecResult {
        Ok(self
            .emulator
            .store()
            .set_region_online(region.display_name())?)
    }
}

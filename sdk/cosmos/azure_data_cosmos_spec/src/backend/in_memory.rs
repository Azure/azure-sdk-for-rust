// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! The in-process in-memory emulator backend.

use super::{Backend, BackendAccount, ClientOptions, RegionControl, TopologyControl};
use crate::{
    profiles::ContainerProfile,
    provision::{ControlPlaneProvisioner, Provisioner},
    Capability, ServiceProfile, SpecResult,
};
use async_trait::async_trait;
use azure_core::{credentials::Secret, http::Url};
use azure_data_cosmos::{
    options::Region, AccountEndpoint, AccountReference, CosmosClient, CosmosClientBuilder,
    CosmosRuntimeBuilder, RoutingStrategy,
};
use azure_data_cosmos_driver::in_memory_emulator::{
    ContainerConfig, InMemoryEmulatorHttpClient, ReplicationConfig, VirtualAccountConfig,
    VirtualRegion, WriteMode,
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
/// deterministic and need no network.
#[derive(Debug)]
pub struct InMemoryBackend;

#[async_trait]
impl Backend for InMemoryBackend {
    fn name(&self) -> &'static str {
        "inmemory"
    }

    fn supports(&self, _profile: ServiceProfile) -> bool {
        true
    }

    fn capabilities(&self, profile: ServiceProfile) -> Vec<Capability> {
        let mut capabilities = vec![
            Capability::TopologyControl,
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
        let mut account = InMemoryAccount {
            profile,
            regions,
            endpoint,
            emulator,
            control_plane: None,
            partitions: Mutex::default(),
        };
        let admin = account.client(ClientOptions::default()).await?;
        account.control_plane = Some(ControlPlaneProvisioner::new(admin));
        Ok(Arc::new(account))
    }
}

#[derive(Debug)]
struct InMemoryAccount {
    profile: ServiceProfile,
    regions: Vec<Region>,
    endpoint: String,
    emulator: Arc<InMemoryEmulatorHttpClient>,
    control_plane: Option<ControlPlaneProvisioner>,
    /// Current physical partition IDs per `(database, container)`.
    partitions: Mutex<HashMap<(String, String), Vec<u32>>>,
}

impl InMemoryAccount {
    fn control_plane(&self) -> &ControlPlaneProvisioner {
        self.control_plane
            .as_ref()
            .expect("control plane is set when the account opens")
    }
}

#[async_trait]
impl BackendAccount for InMemoryAccount {
    fn profile(&self) -> ServiceProfile {
        self.profile
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
    async fn create_database(&self, database: &str) -> SpecResult {
        self.control_plane().create_database(database).await
    }

    async fn delete_database(&self, database: &str) -> SpecResult {
        self.control_plane().delete_database(database).await
    }

    /// Creates the container directly in the emulator store so the physical
    /// partition count is exact.
    async fn create_container(
        &self,
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
        self.partitions.lock().unwrap().insert(
            (database.to_owned(), container.to_owned()),
            (0..profile.physical_partitions).collect(),
        );
        Ok(())
    }

    async fn delete_container(&self, database: &str, container: &str) -> SpecResult {
        self.partitions
            .lock()
            .unwrap()
            .remove(&(database.to_owned(), container.to_owned()));
        self.control_plane()
            .delete_container(database, container)
            .await
    }

    async fn list_databases(&self) -> SpecResult<Vec<String>> {
        self.control_plane().list_databases().await
    }
}

#[async_trait]
impl TopologyControl for InMemoryAccount {
    async fn split_all(&self, database: &str, container: &str) -> SpecResult {
        let key = (database.to_owned(), container.to_owned());
        let parents = self
            .partitions
            .lock()
            .unwrap()
            .get(&key)
            .cloned()
            .ok_or_else(|| format!("container '{container}' was not created by this backend"))?;
        let store = self.emulator.store();
        let mut children = Vec::with_capacity(parents.len() * 2);
        for parent in parents {
            let boundary = store.midpoint_split_epk(database, container, parent)?;
            children.extend(
                store
                    .split_partition_and_wait(database, container, parent, boundary, Duration::ZERO)
                    .await?,
            );
        }
        self.partitions.lock().unwrap().insert(key, children);
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

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{net::SocketAddr, path::Path, sync::Arc, time::Duration};

use azure_core::http::{headers::HeaderValue, Method, Request};
use azure_data_cosmos_driver::{
    in_memory_emulator::{
        ConsistencyLevel, ContainerConfig, InMemoryEmulatorHttpClient, ReplicationConfig,
        VirtualAccountConfig, VirtualRegion, WriteMode,
    },
    models::PartitionKeyDefinition,
};
use serde::Deserialize;
use tokio::net::TcpListener;
use url::Url;

use crate::Result;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct EmulatorConfig {
    pub(crate) account: AccountConfig,
    #[serde(default)]
    pub(crate) management: ManagementConfig,
    #[serde(default)]
    databases: Vec<DatabaseConfig>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct AccountConfig {
    pub(crate) id: String,
    write_mode: WriteModeConfig,
    consistency: ConsistencyConfig,
    #[serde(default)]
    per_partition_failover: bool,
    #[serde(default)]
    throttling: bool,
    regions: Vec<RegionConfig>,
    #[serde(default)]
    replication: ReplicationSettings,
    #[serde(default)]
    replication_overrides: Vec<ReplicationOverride>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct RegionConfig {
    pub(crate) name: String,
    #[serde(default)]
    pub(crate) gateway_port: u16,
    pub(crate) gateway20_port: Option<u16>,
    pub(crate) region_id: Option<u64>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(crate) struct ManagementConfig {
    #[serde(default)]
    pub(crate) port: u16,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct DatabaseConfig {
    id: String,
    #[serde(default)]
    containers: Vec<ContainerSettings>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ContainerSettings {
    id: String,
    partition_key: PartitionKeyDefinition,
    #[serde(default = "default_partition_count")]
    partition_count: u32,
    throughput: Option<u32>,
    #[serde(default)]
    seed_items: Vec<SeedItem>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct SeedItem {
    partition_key: Vec<serde_json::Value>,
    document: serde_json::Value,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ReplicationSettings {
    #[serde(default = "default_min_delay_ms")]
    min_delay_ms: u64,
    #[serde(default = "default_max_delay_ms")]
    max_delay_ms: u64,
    #[serde(default = "default_max_buffered_replications")]
    max_buffered_replications: usize,
}

impl Default for ReplicationSettings {
    fn default() -> Self {
        Self {
            min_delay_ms: default_min_delay_ms(),
            max_delay_ms: default_max_delay_ms(),
            max_buffered_replications: default_max_buffered_replications(),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct ReplicationOverride {
    source: String,
    target: String,
    min_delay_ms: u64,
    max_delay_ms: u64,
    #[serde(default = "default_max_buffered_replications")]
    max_buffered_replications: usize,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum WriteModeConfig {
    Single,
    Multi,
}

#[derive(Clone, Copy, Debug, Deserialize)]
enum ConsistencyConfig {
    Strong,
    BoundedStaleness,
    Session,
    ConsistentPrefix,
    Eventual,
}

#[derive(Clone, Debug)]
pub(crate) struct GatewayBinding {
    pub(crate) region_name: String,
    pub(crate) gateway_url: Url,
    pub(crate) gateway20_url: Option<Url>,
}

pub(crate) struct BoundGateway {
    pub(crate) binding: GatewayBinding,
    pub(crate) gateway_listener: TcpListener,
    pub(crate) gateway20_listener: Option<TcpListener>,
}

impl EmulatorConfig {
    pub(crate) async fn load(path: &Path) -> Result<Self> {
        let contents = tokio::fs::read(path).await?;
        let config: Self = serde_json::from_slice(&contents)?;
        config.validate()?;
        Ok(config)
    }

    pub(crate) async fn bind_gateways(&self) -> Result<Vec<BoundGateway>> {
        let mut gateways = Vec::with_capacity(self.account.regions.len());
        for region in &self.account.regions {
            let gateway_listener =
                TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], region.gateway_port))).await?;
            let gateway_url = loopback_url(gateway_listener.local_addr()?.port())?;
            let gateway20_listener = match region.gateway20_port {
                Some(port) => {
                    Some(TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port))).await?)
                }
                None => None,
            };
            let gateway20_url = gateway20_listener
                .as_ref()
                .map(|listener| loopback_url(listener.local_addr()?.port()))
                .transpose()?;
            gateways.push(BoundGateway {
                binding: GatewayBinding {
                    region_name: region.name.clone(),
                    gateway_url,
                    gateway20_url,
                },
                gateway_listener,
                gateway20_listener,
            });
        }
        Ok(gateways)
    }

    pub(crate) fn create_emulator(
        &self,
        bindings: &[GatewayBinding],
    ) -> Result<Arc<InMemoryEmulatorHttpClient>> {
        if bindings.len() != self.account.regions.len() {
            return Err("every configured region must have a bound gateway".into());
        }
        let regions = self
            .account
            .regions
            .iter()
            .zip(bindings)
            .map(|(region, binding)| {
                let mut virtual_region =
                    VirtualRegion::new(&region.name, binding.gateway_url.clone());
                if let Some(gateway20_url) = &binding.gateway20_url {
                    virtual_region = virtual_region.with_thin_client_url(gateway20_url.clone());
                }
                match region.region_id {
                    Some(region_id) => virtual_region.with_region_id(region_id),
                    None => virtual_region,
                }
            })
            .collect();

        let replication = replication_config(
            self.account.replication.min_delay_ms,
            self.account.replication.max_delay_ms,
            self.account.replication.max_buffered_replications,
        )?;
        let mut account = VirtualAccountConfig::new(regions)?
            .with_account_id(self.account.id.clone())
            .with_write_mode(self.account.write_mode.into())
            .with_consistency(self.account.consistency.into())
            .with_replication_config(replication)
            .with_throttling_enabled(self.account.throttling)
            .with_per_partition_failover(self.account.per_partition_failover);

        for replication_override in &self.account.replication_overrides {
            let override_config = replication_config(
                replication_override.min_delay_ms,
                replication_override.max_delay_ms,
                replication_override.max_buffered_replications,
            )?;
            account = account.with_replication_override(
                &replication_override.source,
                &replication_override.target,
                override_config,
            )?;
        }

        Ok(Arc::new(InMemoryEmulatorHttpClient::new(account)))
    }

    pub(crate) async fn provision(
        &self,
        emulator: &Arc<InMemoryEmulatorHttpClient>,
        gateway_url: &Url,
    ) -> Result<()> {
        let store = emulator.store();

        for database in &self.databases {
            store.create_database(&database.id);
            for container in &database.containers {
                let mut container_config =
                    ContainerConfig::new().with_partition_count(container.partition_count);
                if let Some(throughput) = container.throughput {
                    container_config = container_config.with_throughput(throughput);
                }
                store.create_container_with_config(
                    &database.id,
                    &container.id,
                    container.partition_key.clone(),
                    container_config.build()?,
                );

                for seed_item in &container.seed_items {
                    seed_document(
                        emulator,
                        gateway_url,
                        &database.id,
                        &container.id,
                        seed_item,
                    )
                    .await?;
                }
            }
        }
        Ok(())
    }

    fn validate(&self) -> Result<()> {
        if self.account.id.trim().is_empty() {
            return Err("account.id must not be empty".into());
        }
        if self.account.regions.is_empty() {
            return Err("account.regions must contain at least one region".into());
        }
        let mut ports = std::collections::HashSet::new();
        if self.management.port != 0 {
            ports.insert(self.management.port);
        }
        let mut region_names = std::collections::HashSet::new();
        let mut region_ids = std::collections::HashSet::new();
        for (index, region) in self.account.regions.iter().enumerate() {
            if region.name.trim().is_empty() {
                return Err("region names must not be empty".into());
            }
            if !region_names.insert(region.name.to_ascii_lowercase()) {
                return Err(
                    format!("region name '{}' is configured more than once", region.name).into(),
                );
            }
            let region_id = region.region_id.unwrap_or(index as u64);
            if !region_ids.insert(region_id) {
                return Err(format!("region ID {region_id} is configured more than once").into());
            }
            if region.gateway_port != 0 && !ports.insert(region.gateway_port) {
                return Err(
                    format!("port {} is configured more than once", region.gateway_port).into(),
                );
            }
            if let Some(gateway20_port) = region.gateway20_port {
                if gateway20_port != 0 && !ports.insert(gateway20_port) {
                    return Err(
                        format!("port {gateway20_port} is configured more than once").into(),
                    );
                }
            }
        }
        Ok(())
    }
}

async fn seed_document(
    emulator: &InMemoryEmulatorHttpClient,
    gateway_url: &Url,
    database_id: &str,
    container_id: &str,
    seed_item: &SeedItem,
) -> Result<()> {
    let url = gateway_url.join(&format!("dbs/{database_id}/colls/{container_id}/docs"))?;
    let mut request = Request::new(url, Method::Post);
    request.headers_mut().insert(
        "x-ms-documentdb-partitionkey",
        HeaderValue::from(serde_json::to_string(&seed_item.partition_key)?),
    );
    request.set_body(serde_json::to_vec(&seed_item.document)?);

    let response = emulator.execute_request(&request).await?;
    let status = response.status();
    if status.is_success() {
        return Ok(());
    }

    let raw = response.try_into_raw_response().await?;
    Err(format!(
        "failed to seed {database_id}/{container_id}: HTTP {}: {}",
        u16::from(status),
        String::from_utf8_lossy(raw.body().as_ref())
    )
    .into())
}

fn replication_config(
    min_delay_ms: u64,
    max_delay_ms: u64,
    max_buffered: usize,
) -> Result<ReplicationConfig> {
    Ok(ReplicationConfig::range(
        Duration::from_millis(min_delay_ms),
        Duration::from_millis(max_delay_ms),
    )?
    .with_max_buffered_replications(max_buffered))
}

fn loopback_url(port: u16) -> Result<Url> {
    Ok(Url::parse(&format!("http://127.0.0.1:{port}/"))?)
}

fn default_partition_count() -> u32 {
    4
}

fn default_min_delay_ms() -> u64 {
    20
}

fn default_max_delay_ms() -> u64 {
    50
}

fn default_max_buffered_replications() -> usize {
    10_000
}

impl From<WriteModeConfig> for WriteMode {
    fn from(value: WriteModeConfig) -> Self {
        match value {
            WriteModeConfig::Single => Self::Single,
            WriteModeConfig::Multi => Self::Multi,
        }
    }
}

impl From<ConsistencyConfig> for ConsistencyLevel {
    fn from(value: ConsistencyConfig) -> Self {
        match value {
            ConsistencyConfig::Strong => Self::Strong,
            ConsistencyConfig::BoundedStaleness => Self::BoundedStaleness,
            ConsistencyConfig::Session => Self::Session,
            ConsistencyConfig::ConsistentPrefix => Self::ConsistentPrefix,
            ConsistencyConfig::Eventual => Self::Eventual,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn parses_and_provisions_seed_items() {
        let config: EmulatorConfig = serde_json::from_value(serde_json::json!({
            "account": {
                "id": "test-account",
                "writeMode": "single",
                "consistency": "Session",
                "regions": [{ "name": "East US", "gatewayPort": 0 }]
            },
            "management": { "port": 0 },
            "databases": [{
                "id": "testdb",
                "containers": [{
                    "id": "testcoll",
                    "partitionKey": { "paths": ["/pk"], "kind": "Hash", "version": 2 },
                    "partitionCount": 1,
                    "throughput": 400,
                    "seedItems": [{
                        "partitionKey": ["pk1"],
                        "document": { "id": "item1", "pk": "pk1", "value": 42 }
                    }]
                }]
            }]
        }))
        .unwrap();
        config.validate().unwrap();
        let bound_gateways = config.bind_gateways().await.unwrap();
        let bindings: Vec<_> = bound_gateways
            .iter()
            .map(|gateway| gateway.binding.clone())
            .collect();
        let gateway_url = bindings[0].gateway_url.clone();
        let emulator = config.create_emulator(&bindings).unwrap();
        config.provision(&emulator, &gateway_url).await.unwrap();

        let mut request = Request::new(
            gateway_url
                .join("dbs/testdb/colls/testcoll/docs/item1")
                .unwrap(),
            Method::Get,
        );
        request.headers_mut().insert(
            "x-ms-documentdb-partitionkey",
            HeaderValue::from_static(r#"["pk1"]"#),
        );
        let response = emulator.execute_request(&request).await.unwrap();
        assert_eq!(response.status(), azure_core::http::StatusCode::Ok);
    }

    #[test]
    fn rejects_duplicate_ports() {
        let config: EmulatorConfig = serde_json::from_value(serde_json::json!({
            "account": {
                "id": "test-account",
                "writeMode": "single",
                "consistency": "Session",
                "regions": [{ "name": "East US", "gatewayPort": 9090 }]
            },
            "management": { "port": 9090 }
        }))
        .unwrap();

        assert!(config.validate().is_err());
    }

    #[tokio::test]
    async fn zero_ports_use_distinct_os_assigned_endpoints() {
        let zero_port: EmulatorConfig = serde_json::from_value(serde_json::json!({
            "account": {
                "id": "test-account",
                "writeMode": "single",
                "consistency": "Session",
                "regions": [{ "name": "East US", "gateway20Port": 0 }]
            }
        }))
        .unwrap();
        zero_port.validate().unwrap();
        assert_eq!(zero_port.management.port, 0);

        let gateways = zero_port.bind_gateways().await.unwrap();
        let gateway = &gateways[0];
        let gateway_port = gateway.binding.gateway_url.port().unwrap();
        let gateway20_port = gateway
            .binding
            .gateway20_url
            .as_ref()
            .unwrap()
            .port()
            .unwrap();
        assert_ne!(gateway_port, 0);
        assert_ne!(gateway20_port, 0);
        assert_ne!(gateway_port, gateway20_port);
    }

    #[test]
    fn rejects_duplicate_region_identity() {
        let duplicate: EmulatorConfig = serde_json::from_value(serde_json::json!({
            "account": {
                "id": "test-account",
                "writeMode": "single",
                "consistency": "Session",
                "regions": [
                    { "name": "East US", "gatewayPort": 18081, "regionId": 1 },
                    { "name": "east us", "gatewayPort": 18082, "regionId": 1 }
                ]
            }
        }))
        .unwrap();
        assert!(duplicate.validate().is_err());
    }
}

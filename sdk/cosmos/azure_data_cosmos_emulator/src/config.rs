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
#[serde(rename_all = "camelCase")]
enum WriteModeConfig {
    Single,
    Multi,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
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

pub(crate) struct BoundEndpoint {
    pub(crate) url: Url,
    pub(crate) listener: TcpListener,
}

pub(crate) struct BoundGateway {
    pub(crate) region_name: String,
    pub(crate) gateway: BoundEndpoint,
    pub(crate) gateway20: Option<BoundEndpoint>,
}

impl BoundGateway {
    pub(crate) fn binding(&self) -> GatewayBinding {
        GatewayBinding {
            region_name: self.region_name.clone(),
            gateway_url: self.gateway.url.clone(),
            gateway20_url: self.gateway20.as_ref().map(|endpoint| endpoint.url.clone()),
        }
    }
}

pub(crate) struct BoundHost {
    pub(crate) gateways: Vec<BoundGateway>,
    pub(crate) management: BoundEndpoint,
}

#[derive(Clone, Copy)]
enum ListenerSlot {
    Gateway(usize),
    Gateway20(usize),
    Management,
}

impl EmulatorConfig {
    pub(crate) async fn load(path: &Path) -> Result<Self> {
        let contents = tokio::fs::read(path)
            .await
            .map_err(|error| format!("failed to read config file '{}': {error}", path.display()))?;
        let config: Self = serde_json::from_slice(&contents).map_err(|error| {
            format!(
                "failed to parse config file '{}' as JSON: {error}",
                path.display()
            )
        })?;
        config.validate()?;
        Ok(config)
    }

    pub(crate) async fn bind(&self) -> Result<BoundHost> {
        let mut specifications = Vec::new();
        for (index, region) in self.account.regions.iter().enumerate() {
            specifications.push((ListenerSlot::Gateway(index), region.gateway_port));
            if let Some(port) = region.gateway20_port {
                specifications.push((ListenerSlot::Gateway20(index), port));
            }
        }
        specifications.push((ListenerSlot::Management, self.management.port));
        specifications.sort_by_key(|(_, port)| *port == 0);

        let region_count = self.account.regions.len();
        let mut gateways: Vec<Option<BoundEndpoint>> =
            std::iter::repeat_with(|| None).take(region_count).collect();
        let mut gateways20: Vec<Option<BoundEndpoint>> =
            std::iter::repeat_with(|| None).take(region_count).collect();
        let mut management = None;
        for (slot, port) in specifications {
            let listener = TcpListener::bind(SocketAddr::from(([127, 0, 0, 1], port))).await?;
            let local_address = listener.local_addr()?;
            if !local_address.ip().is_loopback() {
                return Err(
                    format!("emulator listener must bind to loopback: {local_address}").into(),
                );
            }
            let endpoint = BoundEndpoint {
                url: loopback_url(local_address.port())?,
                listener,
            };
            match slot {
                ListenerSlot::Gateway(index) => gateways[index] = Some(endpoint),
                ListenerSlot::Gateway20(index) => gateways20[index] = Some(endpoint),
                ListenerSlot::Management => management = Some(endpoint),
            }
        }

        let gateways = self
            .account
            .regions
            .iter()
            .enumerate()
            .map(|(index, region)| {
                Ok(BoundGateway {
                    region_name: region.name.clone(),
                    gateway: gateways[index]
                        .take()
                        .ok_or("configured gateway listener was not bound")?,
                    gateway20: gateways20[index].take(),
                })
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(BoundHost {
            gateways,
            management: management.ok_or("management listener was not bound")?,
        })
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
                    virtual_region = virtual_region.with_gateway_v2_url(gateway20_url.clone());
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
        self.validate()?;
        let store = emulator.store();

        for database in &self.databases {
            store.create_database(&database.id);
            for container in &database.containers {
                store.create_container_with_config(
                    &database.id,
                    &container.id,
                    container.partition_key.clone(),
                    build_container_config(container)?,
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
        store.drain_pending_replications().await;
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
        let mut database_ids = std::collections::HashSet::new();
        for database in &self.databases {
            validate_resource_id("database", &database.id)?;
            if !database_ids.insert(database.id.as_str()) {
                return Err(
                    format!("database '{}' is configured more than once", database.id).into(),
                );
            }
            let mut container_ids = std::collections::HashSet::new();
            for container in &database.containers {
                validate_resource_id("container", &container.id)?;
                if !container_ids.insert(container.id.as_str()) {
                    return Err(format!(
                        "container '{}/{}' is configured more than once",
                        database.id, container.id
                    )
                    .into());
                }
                build_container_config(container)?;
            }
        }
        Ok(())
    }
}

/// Builds the driver-level [`ContainerConfig`] for a configured container.
///
/// Shared by [`EmulatorConfig::validate`] (which discards the built config,
/// using it only to surface a validation error before anything is
/// provisioned) and [`EmulatorConfig::provision`] (which uses it to actually
/// create the container), so a future container option added to one path
/// cannot silently be forgotten on the other.
fn build_container_config(container: &ContainerSettings) -> Result<ContainerConfig> {
    let mut container_config =
        ContainerConfig::new().with_partition_count(container.partition_count);
    if let Some(throughput) = container.throughput {
        container_config = container_config.with_throughput(throughput);
    }
    Ok(container_config.build()?)
}

fn validate_resource_id(resource_type: &str, id: &str) -> Result<()> {
    if id.trim().is_empty() {
        return Err(format!("{resource_type} IDs must not be empty").into());
    }
    if id.contains('/') || id.contains('\\') || id.contains('?') || id.contains('#') {
        return Err(format!(
            "{resource_type} ID '{id}' contains a character that cannot be used in a Cosmos resource path"
        )
        .into());
    }
    Ok(())
}

async fn seed_document(
    emulator: &InMemoryEmulatorHttpClient,
    gateway_url: &Url,
    database_id: &str,
    container_id: &str,
    seed_item: &SeedItem,
) -> Result<()> {
    let mut url = gateway_url.clone();
    url.path_segments_mut()
        .map_err(|_| "gateway URL cannot be used as a base URL")?
        .extend(["dbs", database_id, "colls", container_id, "docs"]);
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
                "consistency": "session",
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
        let bound_host = config.bind().await.unwrap();
        let bindings: Vec<_> = bound_host
            .gateways
            .iter()
            .map(|gateway| gateway.binding())
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
                "consistency": "session",
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
                "consistency": "session",
                "regions": [{ "name": "East US", "gateway20Port": 0 }]
            }
        }))
        .unwrap();
        zero_port.validate().unwrap();
        assert_eq!(zero_port.management.port, 0);

        let host = zero_port.bind().await.unwrap();
        let gateway = &host.gateways[0];
        let gateway_port = gateway.gateway.url.port().unwrap();
        let gateway20_port = gateway.gateway20.as_ref().unwrap().url.port().unwrap();
        assert_ne!(gateway_port, 0);
        assert_ne!(gateway20_port, 0);
        assert_ne!(gateway_port, gateway20_port);
        assert!(gateway
            .gateway
            .listener
            .local_addr()
            .unwrap()
            .ip()
            .is_loopback());
        assert!(gateway
            .gateway20
            .as_ref()
            .unwrap()
            .listener
            .local_addr()
            .unwrap()
            .ip()
            .is_loopback());
        assert!(host
            .management
            .listener
            .local_addr()
            .unwrap()
            .ip()
            .is_loopback());
    }

    #[test]
    fn rejects_duplicate_region_identity() {
        let duplicate: EmulatorConfig = serde_json::from_value(serde_json::json!({
            "account": {
                "id": "test-account",
                "writeMode": "single",
                "consistency": "session",
                "regions": [
                    { "name": "East US", "gatewayPort": 18081, "regionId": 1 },
                    { "name": "east us", "gatewayPort": 18082, "regionId": 1 }
                ]
            }
        }))
        .unwrap();
        assert!(duplicate.validate().is_err());
    }

    #[test]
    fn rejects_duplicate_startup_resources() {
        let duplicate_database: EmulatorConfig = serde_json::from_value(serde_json::json!({
            "account": {
                "id": "test-account",
                "writeMode": "single",
                "consistency": "session",
                "regions": [{ "name": "East US" }]
            },
            "databases": [{ "id": "db" }, { "id": "db" }]
        }))
        .unwrap();
        assert!(duplicate_database.validate().is_err());

        let duplicate_container: EmulatorConfig = serde_json::from_value(serde_json::json!({
            "account": {
                "id": "test-account",
                "writeMode": "single",
                "consistency": "session",
                "regions": [{ "name": "East US" }]
            },
            "databases": [{
                "id": "db",
                "containers": [
                    { "id": "coll", "partitionKey": { "paths": ["/pk"], "kind": "Hash", "version": 2 } },
                    { "id": "coll", "partitionKey": { "paths": ["/pk"], "kind": "Hash", "version": 2 } }
                ]
            }]
        }))
        .unwrap();
        assert!(duplicate_container.validate().is_err());
    }

    #[tokio::test]
    async fn invalid_startup_config_does_not_partially_provision() {
        let config: EmulatorConfig = serde_json::from_value(serde_json::json!({
            "account": {
                "id": "test-account",
                "writeMode": "single",
                "consistency": "session",
                "regions": [{ "name": "East US" }]
            },
            "databases": [
                { "id": "valid" },
                {
                    "id": "invalid",
                    "containers": [{
                        "id": "coll",
                        "partitionKey": { "paths": ["/pk"], "kind": "Hash", "version": 2 },
                        "partitionCount": 0
                    }]
                }
            ]
        }))
        .unwrap();
        let bindings = vec![GatewayBinding {
            region_name: "East US".to_owned(),
            gateway_url: Url::parse("http://127.0.0.1:18081/").unwrap(),
            gateway20_url: None,
        }];
        let emulator = config.create_emulator(&bindings).unwrap();
        let result = config.provision(&emulator, &bindings[0].gateway_url).await;
        assert!(result.is_err());
        let request = Request::new(
            bindings[0].gateway_url.join("dbs/valid").unwrap(),
            Method::Get,
        );
        let response = emulator.execute_request(&request).await.unwrap();
        assert_eq!(response.status(), azure_core::http::StatusCode::NotFound);
    }

    #[tokio::test]
    async fn applies_replication_overrides() {
        let config: EmulatorConfig = serde_json::from_value(serde_json::json!({
            "account": {
                "id": "test-account",
                "writeMode": "single",
                "consistency": "session",
                "regions": [
                    { "name": "East US", "gatewayPort": 0 },
                    { "name": "West US", "gatewayPort": 0 }
                ],
                "replication": { "minDelayMs": 0, "maxDelayMs": 0 },
                "replicationOverrides": [
                    { "source": "East US", "target": "West US", "minDelayMs": 5000, "maxDelayMs": 5000 }
                ]
            }
        }))
        .unwrap();
        config.validate().unwrap();
        let bound_host = config.bind().await.unwrap();
        let bindings: Vec<_> = bound_host
            .gateways
            .iter()
            .map(|gateway| gateway.binding())
            .collect();
        let east_url = bindings[0].gateway_url.clone();
        let west_url = bindings[1].gateway_url.clone();
        let emulator = config.create_emulator(&bindings).unwrap();
        emulator.store().create_database("testdb");
        let partition_key: PartitionKeyDefinition = serde_json::from_value(serde_json::json!({
            "paths": ["/pk"], "kind": "Hash", "version": 2
        }))
        .unwrap();
        emulator.store().create_container_with_config(
            "testdb",
            "testcoll",
            partition_key,
            ContainerConfig::new()
                .with_partition_count(1)
                .build()
                .unwrap(),
        );

        let mut request = Request::new(
            east_url.join("dbs/testdb/colls/testcoll/docs").unwrap(),
            Method::Post,
        );
        request.headers_mut().insert(
            "x-ms-documentdb-partitionkey",
            HeaderValue::from_static(r#"["pk1"]"#),
        );
        request.set_body(
            serde_json::to_vec(&serde_json::json!({ "id": "item1", "pk": "pk1" })).unwrap(),
        );
        let response = emulator.execute_request(&request).await.unwrap();
        assert_eq!(response.status(), azure_core::http::StatusCode::Created);

        // The account-wide default (0ms) would have replicated almost
        // instantly; the override for East US -> West US specifically is
        // 5s, so after a much shorter wait West US must not see the write
        // yet. This is only a meaningful assertion if the per-pair override
        // — not the account default — is actually the one being applied.
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;
        let mut read = Request::new(
            west_url
                .join("dbs/testdb/colls/testcoll/docs/item1")
                .unwrap(),
            Method::Get,
        );
        read.headers_mut().insert(
            "x-ms-documentdb-partitionkey",
            HeaderValue::from_static(r#"["pk1"]"#),
        );
        let response = emulator.execute_request(&read).await.unwrap();
        assert_eq!(
            response.status(),
            azure_core::http::StatusCode::NotFound,
            "replicationOverrides' longer delay must be applied instead of the account default"
        );
    }

    #[tokio::test]
    async fn load_reports_a_clean_error_naming_a_missing_config_file() {
        let missing_path =
            Path::new("azure-data-cosmos-emulator-this-config-file-does-not-exist.json");
        let error = EmulatorConfig::load(missing_path)
            .await
            .expect_err("loading a nonexistent path must return an error, not panic");
        assert!(
            error
                .to_string()
                .contains("azure-data-cosmos-emulator-this-config-file-does-not-exist.json"),
            "error must name the offending path, got: {error}"
        );
    }

    #[tokio::test]
    async fn load_reports_a_clean_error_for_malformed_json() {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path =
            std::env::temp_dir().join(format!("azure-data-cosmos-emulator-test-{unique}.json"));
        tokio::fs::write(&path, b"{ not valid json").await.unwrap();

        let error = EmulatorConfig::load(&path).await;
        tokio::fs::remove_file(&path).await.ok();

        let error = error.expect_err("malformed JSON must return an error, not panic");
        assert!(
            error.to_string().contains(&path.display().to_string()),
            "error must name the offending path, got: {error}"
        );
    }
}

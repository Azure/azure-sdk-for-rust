// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{panic::AssertUnwindSafe, sync::OnceLock};

use azure_core::Uuid;
use azure_data_cosmos::{
    clients::ContainerClient,
    models::{ContainerProperties, PartitionKeyDefinition},
    options::{
        BinaryEncodingOptions, ConnectionPoolOptions, OperationOptions, PartitionFailoverOptions,
        ReadConsistencyStrategy, Region,
    },
    AccountEndpoint, AccountReference, CosmosClient, CosmosClientBuilder, CosmosRuntime,
    RoutingStrategy,
};
use futures::FutureExt;

use crate::e2e_test_cases::catalog::{ClientDefinition, RuntimeDefinition};

pub type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

/// Serializes tests that share hosted-emulator account state, including
/// replication controls, wire counters, and dynamic topology transitions.
pub(super) fn fixture_test_lock() -> &'static tokio::sync::Mutex<()> {
    static LOCK: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| tokio::sync::Mutex::new(()))
}

pub struct E2eTestFixture {
    cleanup: DatabaseCleanup,
    pub database_id: String,
    pub container_id: String,
    pub container: ContainerClient,
}

pub struct E2eTest;

pub struct E2eTestBuilder {
    client: Option<CosmosClient>,
    partition_key: PartitionKeyDefinition,
}

pub struct ClientSetup {
    pub routing_strategy: RoutingStrategy,
    pub runtime_read_consistency: Option<ReadConsistencyStrategy>,
    pub client_read_consistency: Option<ReadConsistencyStrategy>,
    pub gateway_v2_enabled: Option<bool>,
    pub ppcb_enabled: Option<bool>,
    pub binary_encoding_enabled: Option<bool>,
}

impl ClientSetup {
    pub fn from_profile(
        runtime: &RuntimeDefinition,
        client: &ClientDefinition,
        routing_strategy: RoutingStrategy,
    ) -> TestResult<Self> {
        let matches_profile = match (client.routing.as_str(), &routing_strategy) {
            ("proximity", RoutingStrategy::ProximityTo(_)) => true,
            ("preferredRegions", RoutingStrategy::PreferredRegions(regions)) => !regions.is_empty(),
            ("accountOrder", RoutingStrategy::PreferredRegions(regions)) => regions.is_empty(),
            _ => false,
        };
        if !matches_profile {
            return Err(format!(
                "client profile '{}' declares routing '{}' but the test constructed {routing_strategy:?}",
                client.id, client.routing
            )
            .into());
        }
        Self::from_profile_with_routing_override(runtime, client, routing_strategy)
    }

    /// Applies profile defaults while allowing a scenario that explicitly
    /// exercises multiple routing strategies to override `client.routing`.
    pub fn from_profile_with_routing_override(
        runtime: &RuntimeDefinition,
        client: &ClientDefinition,
        routing_strategy: RoutingStrategy,
    ) -> TestResult<Self> {
        Ok(Self {
            routing_strategy,
            runtime_read_consistency: parse_optional_read_consistency(
                runtime.default_read_consistency_strategy.as_deref(),
            )?,
            client_read_consistency: parse_optional_read_consistency(
                client.default_read_consistency_strategy.as_deref(),
            )?,
            gateway_v2_enabled: parse_setup_switch(&runtime.gateway_v2, "backendDefault")?,
            ppcb_enabled: parse_setup_switch(&runtime.ppcb, "sdkDefault")?,
            binary_encoding_enabled: parse_setup_switch(&client.binary_encoding, "sdkDefault")?,
        })
    }
}

fn parse_optional_read_consistency(
    value: Option<&str>,
) -> TestResult<Option<ReadConsistencyStrategy>> {
    value
        .map(|value| value.parse::<ReadConsistencyStrategy>().map_err(Into::into))
        .transpose()
}

fn parse_setup_switch(value: &str, default: &str) -> TestResult<Option<bool>> {
    match value {
        "enabled" => Ok(Some(true)),
        "disabled" => Ok(Some(false)),
        value if value == default => Ok(None),
        value => Err(format!("unsupported setup switch '{value}'").into()),
    }
}

struct DatabaseCleanup {
    client: CosmosClient,
    database_id: String,
}

impl E2eTest {
    pub fn builder() -> E2eTestBuilder {
        E2eTestBuilder {
            client: None,
            partition_key: "/pk".into(),
        }
    }
}

impl E2eTestBuilder {
    pub fn with_client(mut self, client: CosmosClient) -> Self {
        self.client = Some(client);
        self
    }

    pub fn with_partition_key_definition(mut self, partition_key: PartitionKeyDefinition) -> Self {
        self.partition_key = partition_key;
        self
    }

    pub async fn run<F>(self, test: F) -> TestResult
    where
        F: AsyncFnOnce(&E2eTestFixture) -> TestResult,
    {
        let _guard = fixture_test_lock().lock().await;
        let client = match self.client {
            Some(client) => client,
            None => build_client().await?,
        };
        E2eTestFixture::run_with_client_unlocked(client, self.partition_key, test).await
    }
}

impl E2eTestFixture {
    pub async fn run<F>(test: F) -> TestResult
    where
        F: AsyncFnOnce(&E2eTestFixture) -> TestResult,
    {
        let _guard = fixture_test_lock().lock().await;
        let client = build_client().await?;
        Self::run_with_client_unlocked(client, "/pk".into(), test).await
    }

    pub async fn run_with_partition_key<F>(
        partition_key: PartitionKeyDefinition,
        test: F,
    ) -> TestResult
    where
        F: AsyncFnOnce(&E2eTestFixture) -> TestResult,
    {
        let _guard = fixture_test_lock().lock().await;
        let client = build_client().await?;
        Self::run_with_client_unlocked(client, partition_key, test).await
    }

    pub async fn run_with_container_properties<F>(
        properties: ContainerProperties,
        test: F,
    ) -> TestResult
    where
        F: AsyncFnOnce(&E2eTestFixture) -> TestResult,
    {
        let _guard = fixture_test_lock().lock().await;
        let client = build_client().await?;
        Self::run_with_client_and_properties(client, properties, test).await
    }

    async fn run_with_client_unlocked<F>(
        client: CosmosClient,
        partition_key: PartitionKeyDefinition,
        test: F,
    ) -> TestResult
    where
        F: AsyncFnOnce(&E2eTestFixture) -> TestResult,
    {
        let container_id = format!("items-{}", Uuid::now_v7());
        let properties = ContainerProperties::new(container_id, partition_key);
        Self::run_with_client_and_properties(client, properties, test).await
    }

    async fn run_with_client_and_properties<F>(
        client: CosmosClient,
        properties: ContainerProperties,
        test: F,
    ) -> TestResult
    where
        F: AsyncFnOnce(&E2eTestFixture) -> TestResult,
    {
        let fixture = Self::new(client, properties).await?;
        let outcome = AssertUnwindSafe(test(&fixture)).catch_unwind().await;
        let cleanup = fixture.cleanup().await;
        match outcome {
            Ok(Ok(())) => cleanup,
            Ok(Err(test_error)) => match cleanup {
                Ok(()) => Err(test_error),
                Err(cleanup_error) => Err(format!(
                    "E2E test failed: {test_error}; database cleanup also failed: {cleanup_error}"
                )
                .into()),
            },
            Err(panic) => {
                if let Err(error) = cleanup {
                    eprintln!("E2E database cleanup after panic failed: {error}");
                }
                std::panic::resume_unwind(panic)
            }
        }
    }

    async fn new(client: CosmosClient, properties: ContainerProperties) -> TestResult<Self> {
        // Preserve creation time in leaked resource IDs so cleanup tooling can age them out.
        let database_id = format!("e2e-{}", Uuid::now_v7());
        let container_id = properties.id.to_string();
        client.create_database(&database_id, None).await?;
        let database = client.database_client(&database_id);
        let cleanup = DatabaseCleanup::new(client.clone(), database_id.clone());
        let setup = async {
            database.create_container(properties, None).await?;
            database.container_client(&container_id, None).await
        }
        .await;
        match setup {
            Ok(container) => Ok(Self {
                cleanup,
                database_id,
                container_id,
                container,
            }),
            Err(setup_error) => match cleanup.cleanup().await {
                Ok(()) => Err(setup_error.into()),
                Err(cleanup_error) => Err(format!(
                    "E2E fixture setup failed: {setup_error}; database cleanup also failed: {cleanup_error}"
                )
                .into()),
            },
        }
    }

    async fn cleanup(self) -> TestResult {
        self.cleanup.cleanup().await
    }
}

impl DatabaseCleanup {
    fn new(client: CosmosClient, database_id: String) -> Self {
        Self {
            client,
            database_id,
        }
    }

    async fn cleanup(self) -> TestResult {
        self.client
            .database_client(&self.database_id)
            .delete(None)
            .await?;
        Ok(())
    }
}

pub async fn build_client() -> TestResult<CosmosClient> {
    build_client_with_routing(RoutingStrategy::ProximityTo(Region::EAST_US)).await
}

pub async fn build_client_with_routing(
    routing_strategy: RoutingStrategy,
) -> TestResult<CosmosClient> {
    build_client_with_defaults(ClientSetup {
        routing_strategy,
        runtime_read_consistency: None,
        client_read_consistency: None,
        gateway_v2_enabled: None,
        ppcb_enabled: None,
        binary_encoding_enabled: None,
    })
    .await
}

pub async fn build_client_with_defaults(setup: ClientSetup) -> TestResult<CosmosClient> {
    build_client_with_customizer(setup, Ok).await
}

pub async fn build_client_with_customizer<F>(
    setup: ClientSetup,
    customize: F,
) -> TestResult<CosmosClient>
where
    F: FnOnce(CosmosClientBuilder) -> TestResult<CosmosClientBuilder>,
{
    let connection_string = std::env::var("AZURE_COSMOS_CONNECTION_STRING")?;
    let endpoint = connection_string_value(&connection_string, "AccountEndpoint")?;
    let key = connection_string_value(&connection_string, "AccountKey")?;
    let endpoint: AccountEndpoint = endpoint.parse()?;
    let mut runtime_builder = CosmosRuntime::builder();
    if let Some(enabled) = setup.gateway_v2_enabled {
        let options = ConnectionPoolOptions::builder()
            .with_gateway_v2_disabled(!enabled)
            .build()?;
        runtime_builder = runtime_builder.with_connection_pool(options);
    }
    if let Some(strategy) = setup.runtime_read_consistency {
        let mut options = OperationOptions::default();
        options.read_consistency_strategy = Some(strategy);
        runtime_builder = runtime_builder.with_default_operation_options(options);
    }
    let runtime = runtime_builder.build().await?;

    let mut client_builder = CosmosClient::builder().with_runtime(runtime);
    if let Some(strategy) = setup.client_read_consistency {
        let mut options = OperationOptions::default();
        options.read_consistency_strategy = Some(strategy);
        client_builder = client_builder.with_default_operation_options(options);
    }
    if let Some(enabled) = setup.ppcb_enabled {
        let options = PartitionFailoverOptions::builder()
            .with_circuit_breaker_enabled(enabled)
            .build()?;
        client_builder = client_builder.with_partition_failover_options(options);
    }
    if let Some(enabled) = setup.binary_encoding_enabled {
        client_builder = client_builder
            .with_binary_encoding_options(BinaryEncodingOptions::new().with_enabled(enabled));
    }
    Ok(customize(client_builder)?
        .build(
            AccountReference::with_authentication_key(endpoint, key),
            setup.routing_strategy,
        )
        .await?)
}

pub(super) fn connection_string_value(connection_string: &str, key: &str) -> TestResult<String> {
    connection_string
        .split(';')
        .filter_map(|part| part.split_once('='))
        .find_map(|(name, value)| name.eq_ignore_ascii_case(key).then_some(value.to_owned()))
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("connection string is missing {key}").into())
}

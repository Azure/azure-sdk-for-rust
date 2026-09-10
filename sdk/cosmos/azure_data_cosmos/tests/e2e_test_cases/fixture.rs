// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::Uuid;
use azure_data_cosmos::{
    clients::ContainerClient,
    models::{ContainerProperties, PartitionKeyDefinition},
    options::{
        BinaryEncodingOptions, ConnectionPoolOptions, OperationOptions, PartitionFailoverOptions,
        ReadConsistencyStrategy, Region,
    },
    AccountEndpoint, AccountReference, CosmosClient, CosmosRuntime, RoutingStrategy,
};

pub type TestResult<T = ()> = Result<T, Box<dyn std::error::Error>>;

pub struct E2eTestFixture {
    client: CosmosClient,
    database_id: String,
    pub container: ContainerClient,
}

impl E2eTestFixture {
    pub async fn run<F>(test: F) -> TestResult
    where
        F: AsyncFnOnce(&E2eTestFixture) -> TestResult,
    {
        Self::run_with_partition_key("/pk".into(), test).await
    }

    pub async fn run_with_partition_key<F>(
        partition_key: PartitionKeyDefinition,
        test: F,
    ) -> TestResult
    where
        F: AsyncFnOnce(&E2eTestFixture) -> TestResult,
    {
        let client = build_client().await?;
        Self::run_with_client(client, partition_key, test).await
    }

    pub async fn run_with_client<F>(
        client: CosmosClient,
        partition_key: PartitionKeyDefinition,
        test: F,
    ) -> TestResult
    where
        F: AsyncFnOnce(&E2eTestFixture) -> TestResult,
    {
        let fixture = Self::new(client, partition_key).await?;
        let outcome = test(&fixture).await;
        let cleanup = fixture.cleanup().await;
        outcome?;
        cleanup
    }

    async fn new(client: CosmosClient, partition_key: PartitionKeyDefinition) -> TestResult<Self> {
        let database_id = format!("e2e-{}", Uuid::new_v4());
        let container_id = format!("items-{}", Uuid::new_v4());
        client.create_database(&database_id, None).await?;
        let database = client.database_client(&database_id);
        database
            .create_container(
                ContainerProperties::new(container_id.clone(), partition_key),
                None,
            )
            .await?;
        let container = database.container_client(&container_id, None).await?;
        Ok(Self {
            client,
            database_id,
            container,
        })
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
    build_client_with_defaults(routing_strategy, None, None, None, None, None).await
}

pub async fn build_client_with_defaults(
    routing_strategy: RoutingStrategy,
    runtime_strategy: Option<ReadConsistencyStrategy>,
    client_strategy: Option<ReadConsistencyStrategy>,
    gateway_v2_enabled: Option<bool>,
    ppcb_enabled: Option<bool>,
    binary_encoding_enabled: Option<bool>,
) -> TestResult<CosmosClient> {
    let connection_string = std::env::var("AZURE_COSMOS_CONNECTION_STRING")?;
    let endpoint = connection_string_value(&connection_string, "AccountEndpoint")?;
    let key = connection_string_value(&connection_string, "AccountKey")?;
    let endpoint: AccountEndpoint = endpoint.parse()?;
    let mut runtime_builder = CosmosRuntime::builder();
    if let Some(enabled) = gateway_v2_enabled {
        let options = ConnectionPoolOptions::builder()
            .with_gateway_v2_disabled(!enabled)
            .build()?;
        runtime_builder = runtime_builder.with_connection_pool(options);
    }
    if let Some(strategy) = runtime_strategy {
        let mut options = OperationOptions::default();
        options.read_consistency_strategy = Some(strategy);
        runtime_builder = runtime_builder.with_default_operation_options(options);
    }
    let runtime = runtime_builder.build().await?;

    let mut client_builder = CosmosClient::builder().with_runtime(runtime);
    if let Some(strategy) = client_strategy {
        let mut options = OperationOptions::default();
        options.read_consistency_strategy = Some(strategy);
        client_builder = client_builder.with_default_operation_options(options);
    }
    if let Some(enabled) = ppcb_enabled {
        let options = PartitionFailoverOptions::builder()
            .with_circuit_breaker_enabled(enabled)
            .build()?;
        client_builder = client_builder.with_partition_failover_options(options);
    }
    if let Some(enabled) = binary_encoding_enabled {
        client_builder = client_builder
            .with_binary_encoding_options(BinaryEncodingOptions::new().with_enabled(enabled));
    }
    Ok(client_builder
        .build(
            AccountReference::with_authentication_key(endpoint, key),
            routing_strategy,
        )
        .await?)
}

fn connection_string_value(connection_string: &str, key: &str) -> TestResult<String> {
    connection_string
        .split(';')
        .filter_map(|part| part.split_once('='))
        .find_map(|(name, value)| name.eq_ignore_ascii_case(key).then_some(value.to_owned()))
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("connection string is missing {key}").into())
}

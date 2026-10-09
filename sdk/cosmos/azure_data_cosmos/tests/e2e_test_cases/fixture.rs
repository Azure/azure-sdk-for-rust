// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{
    env::VarError,
    panic::AssertUnwindSafe,
    sync::{Arc, OnceLock},
};

use azure_core::{credentials::TokenCredential, Uuid};
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AuthMode {
    Key,
    Aad,
}

fn parse_auth_mode(value: Result<String, VarError>) -> TestResult<AuthMode> {
    match value {
        Err(VarError::NotPresent) => Ok(AuthMode::Key),
        Ok(value) if value.eq_ignore_ascii_case("key") => Ok(AuthMode::Key),
        Ok(value) if value.eq_ignore_ascii_case("aad") => Ok(AuthMode::Aad),
        Ok(_) => Err("AZURE_COSMOS_AUTH_MODE must be 'key' or 'aad'".into()),
        Err(_) => Err("AZURE_COSMOS_AUTH_MODE could not be read as Unicode".into()),
    }
}

fn account_reference(
    connection_string: &str,
    auth_mode: AuthMode,
    credential: impl FnOnce() -> azure_core::Result<Arc<dyn TokenCredential>>,
) -> TestResult<AccountReference> {
    let endpoint: AccountEndpoint =
        connection_string_value(connection_string, "AccountEndpoint")?.parse()?;
    Ok(match auth_mode {
        AuthMode::Key => AccountReference::with_authentication_key(
            endpoint,
            connection_string_value(connection_string, "AccountKey")?,
        ),
        AuthMode::Aad => AccountReference::with_credential(endpoint, credential()?),
    })
}

async fn select_management_client(
    auth_mode: AuthMode,
    data_client: &CosmosClient,
    build_key_client: impl AsyncFnOnce() -> TestResult<CosmosClient>,
) -> TestResult<CosmosClient> {
    match auth_mode {
        // Preserve custom clients (including their transport and routing) in key mode.
        AuthMode::Key => Ok(data_client.clone()),
        AuthMode::Aad => build_key_client().await,
    }
}

pub(super) async fn build_management_client(
    data_client: &CosmosClient,
) -> TestResult<CosmosClient> {
    let auth_mode = parse_auth_mode(std::env::var("AZURE_COSMOS_AUTH_MODE"))?;
    select_management_client(auth_mode, data_client, async || {
        build_client_with_auth(
            default_client_setup(RoutingStrategy::ProximityTo(Region::EAST_US)),
            Ok,
            AuthMode::Key,
        )
        .await
    })
    .await
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
        let management_client = build_management_client(&client).await?;
        Self::new_with_clients(client, management_client, properties).await
    }

    async fn new_with_clients(
        client: CosmosClient,
        management_client: CosmosClient,
        properties: ContainerProperties,
    ) -> TestResult<Self> {
        // Preserve creation time in leaked resource IDs so cleanup tooling can age them out.
        let database_id = format!("e2e-{}", Uuid::now_v7());
        let container_id = properties.id.to_string();
        management_client
            .create_database(&database_id, None)
            .await?;
        let database = management_client.database_client(&database_id);
        let cleanup = DatabaseCleanup::new(management_client, database_id.clone());
        let setup = async {
            database.create_container(properties, None).await?;
            client
                .database_client(&database_id)
                .container_client(&container_id, None)
                .await
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
    build_client_with_defaults(default_client_setup(routing_strategy)).await
}

fn default_client_setup(routing_strategy: RoutingStrategy) -> ClientSetup {
    ClientSetup {
        routing_strategy,
        runtime_read_consistency: None,
        client_read_consistency: None,
        gateway_v2_enabled: None,
        ppcb_enabled: None,
        binary_encoding_enabled: None,
    }
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
    let auth_mode = parse_auth_mode(std::env::var("AZURE_COSMOS_AUTH_MODE"))?;
    build_client_with_auth(setup, customize, auth_mode).await
}

async fn build_client_with_auth<F>(
    setup: ClientSetup,
    customize: F,
    auth_mode: AuthMode,
) -> TestResult<CosmosClient>
where
    F: FnOnce(CosmosClientBuilder) -> TestResult<CosmosClientBuilder>,
{
    let connection_string = std::env::var("AZURE_COSMOS_CONNECTION_STRING")?;
    let account = account_reference(&connection_string, auth_mode, || {
        azure_core_test::credentials::from_env(None)
    })?;
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
        .build(account, setup.routing_strategy)
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

#[cfg(test)]
mod tests {
    use super::{account_reference, parse_auth_mode, AuthMode, TestResult, VarError};
    use azure_core_test::credentials::MockCredential;

    #[test]
    fn auth_mode_defaults_only_when_absent() -> TestResult {
        assert_eq!(parse_auth_mode(Err(VarError::NotPresent))?, AuthMode::Key);
        for (value, expected) in [
            ("key", AuthMode::Key),
            ("KEY", AuthMode::Key),
            ("aad", AuthMode::Aad),
            ("AAD", AuthMode::Aad),
        ] {
            assert_eq!(parse_auth_mode(Ok(value.into()))?, expected);
        }
        Ok(())
    }

    #[test]
    fn auth_mode_rejects_invalid_and_unreadable_values() {
        for value in ["", "unknown", " aad", "key "] {
            assert!(parse_auth_mode(Ok(value.into())).is_err());
        }
        // Inject the environment read error without mutating process-wide state.
        assert!(parse_auth_mode(Err(VarError::NotUnicode("unreadable".into()))).is_err());
    }

    #[test]
    fn account_auth_does_not_fall_back_between_key_and_aad() -> TestResult {
        let endpoint = "AccountEndpoint=https://eastus.emulator.local";
        let connection_string = format!("{endpoint};AccountKey=dGVzdGtleQ==");
        account_reference(&connection_string, AuthMode::Key, || {
            panic!("key mode must not initialize a token credential")
        })?;
        assert!(account_reference(endpoint, AuthMode::Key, || {
            panic!("a missing key must not fall back to AAD")
        })
        .is_err());

        let mut credential_created = false;
        account_reference(endpoint, AuthMode::Aad, || {
            credential_created = true;
            Ok(MockCredential::new()?)
        })?;
        assert!(credential_created);
        assert!(account_reference(&connection_string, AuthMode::Aad, || {
            Err(azure_core::Error::new(
                azure_core::error::ErrorKind::Credential,
                "credential unavailable",
            ))
        })
        .is_err());
        Ok(())
    }

    #[cfg(feature = "__internal_in_memory_emulator")]
    mod in_memory {
        use super::{
            super::{
                select_management_client, ContainerProperties, CosmosClient, E2eTestFixture,
                Region, RoutingStrategy,
            },
            account_reference, AuthMode, MockCredential, TestResult,
        };
        use azure_core::http::{StatusCode, Url};
        use azure_data_cosmos::CosmosRuntimeBuilder;
        use azure_data_cosmos_driver::in_memory_emulator::{
            InMemoryEmulatorHttpClient, VirtualAccountConfig, VirtualRegion,
        };
        use futures::TryStreamExt;
        use std::sync::Arc;

        fn emulator() -> TestResult<Arc<InMemoryEmulatorHttpClient>> {
            let config = VirtualAccountConfig::new(vec![VirtualRegion::new(
                "East US",
                Url::parse("https://eastus.emulator.local")?,
            )])?;
            Ok(Arc::new(InMemoryEmulatorHttpClient::new(config)))
        }

        async fn client(
            emulator: &Arc<InMemoryEmulatorHttpClient>,
            auth_mode: AuthMode,
        ) -> TestResult<CosmosClient> {
            // No key is available to the AAD branch, so a silent fallback cannot succeed.
            let connection_string = match auth_mode {
                AuthMode::Key => {
                    "AccountEndpoint=https://eastus.emulator.local;AccountKey=dGVzdGtleQ=="
                }
                AuthMode::Aad => "AccountEndpoint=https://eastus.emulator.local",
            };
            let account =
                account_reference(connection_string, auth_mode, || Ok(MockCredential::new()?))?;
            let runtime = CosmosRuntimeBuilder::from(emulator.runtime_builder())
                .build()
                .await?;
            Ok(CosmosClient::builder()
                .with_runtime(runtime)
                .build(account, RoutingStrategy::ProximityTo(Region::EAST_US))
                .await?)
        }

        async fn assert_no_databases(client: &CosmosClient) -> TestResult {
            assert!(client
                .query_databases("SELECT * FROM root r", None)
                .await?
                .try_next()
                .await?
                .is_none());
            Ok(())
        }

        #[tokio::test]
        async fn fixture_uses_selected_auth_and_preserves_custom_key_client() -> TestResult {
            for auth_mode in [AuthMode::Key, AuthMode::Aad] {
                let emulator = emulator()?;
                let data = client(&emulator, auth_mode).await?;
                let mut management_built = false;
                let management = select_management_client(auth_mode, &data, async || {
                    management_built = true;
                    client(&emulator, AuthMode::Key).await
                })
                .await?;
                assert_eq!(management_built, auth_mode == AuthMode::Aad);
                let fixture = E2eTestFixture::new_with_clients(
                    data,
                    management.clone(),
                    ContainerProperties::new("items", "/pk".into()),
                )
                .await?;
                let response = fixture
                    .container
                    .create_item(
                        "A",
                        "item",
                        serde_json::json!({"id": "item", "pk": "A"}),
                        None,
                    )
                    .await?;
                assert_eq!(response.status().status_code(), StatusCode::Created);
                assert_eq!(
                    management
                        .database_client(&fixture.database_id)
                        .read(None)
                        .await?
                        .status()
                        .status_code(),
                    StatusCode::Ok
                );
                fixture.cleanup().await?;
                assert_no_databases(&management).await?;
            }
            Ok(())
        }

        #[tokio::test]
        async fn fixture_does_not_use_management_container_and_cleans_up_on_data_failure(
        ) -> TestResult {
            // Distinct stores make using the wrong client observable without inspecting source.
            let data_emulator = emulator()?;
            let management_emulator = emulator()?;
            let data = client(&data_emulator, AuthMode::Aad).await?;
            let management = select_management_client(AuthMode::Aad, &data, async || {
                client(&management_emulator, AuthMode::Key).await
            })
            .await?;
            let result = E2eTestFixture::new_with_clients(
                data.clone(),
                management.clone(),
                ContainerProperties::new("items", "/pk".into()),
            )
            .await;
            assert!(result.is_err(), "the data account has no such container");
            assert_no_databases(&management).await?;
            assert_no_databases(&data).await?;
            Ok(())
        }

        #[tokio::test]
        async fn management_failure_does_not_fall_back_to_data_client() -> TestResult {
            let emulator = emulator()?;
            let data = client(&emulator, AuthMode::Aad).await?;
            let result = select_management_client(AuthMode::Aad, &data, async || {
                Err("management key unavailable".into())
            })
            .await;
            assert!(result.is_err());
            assert_no_databases(&data).await?;
            Ok(())
        }
    }
}

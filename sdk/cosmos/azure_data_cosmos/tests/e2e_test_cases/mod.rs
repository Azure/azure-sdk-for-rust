// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod catalog;
mod fixture;

use azure_core::http::{Etag, StatusCode};
use azure_data_cosmos::{
    feed::FeedScope,
    options::{
        AvailabilityStrategy, ItemReadOptions, ItemWriteOptions, OperationOptions, Precondition,
        ReadConsistencyStrategy, Region,
    },
    Query, RoutingStrategy,
};
use futures::{StreamExt, TryStreamExt};
use serde::{Deserialize, Serialize};

use catalog::{assert_diagnostics, assert_status, profile as load_profile, scenario};
use fixture::{build_client, build_client_with_defaults, E2eTestFixture, TestResult};

const IMPLEMENTED_TESTS: &[&str] = &[
    "capability_document_is_versioned",
    "bootstrap_primary_endpoint",
    "item_lifecycle",
    "upsert_creates_then_updates",
    "duplicate_create_preserves_original",
    "not_found_does_not_cross_partition_keys",
    "stale_etag_preserves_successful_update",
    "parameterized_query_filters_and_orders",
    "invalid_query_is_not_an_empty_feed",
    "diagnostics_cover_success_and_error",
];

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Item {
    id: String,
    pk: String,
    value: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    score: Option<i64>,
}

fn item(id: &str, pk: &str, value: i64) -> Item {
    Item {
        id: id.to_owned(),
        pk: pk.to_owned(),
        value,
        score: None,
    }
}

fn write_options_with_content() -> ItemWriteOptions {
    let mut operation = OperationOptions::default();
    operation.content_response_on_write =
        Some(azure_data_cosmos::options::ContentResponseOnWrite::Enabled);
    ItemWriteOptions::default().with_operation_options(operation)
}

fn hosted_only() -> bool {
    cfg!(any(
        test_category = "emulator_inmemory",
        test_category = "e2e"
    ))
}

#[test]
fn e2e_scenario_catalog_is_valid() {
    catalog::validate_catalog(IMPLEMENTED_TESTS).expect("E2E scenario catalog must be valid");
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CapabilityDocument {
    api_version: u32,
    emulator_version: String,
    protocols: ProtocolCapabilities,
    data_plane: Vec<String>,
    management_actions: Vec<String>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProtocolCapabilities {
    gateway_v1: bool,
    gateway_v2: bool,
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn capability_document_is_versioned() -> TestResult {
    let test_scenario = scenario("management.capabilities");
    let step = test_scenario.step("readCapabilities");
    let management_endpoint = std::env::var("AZURE_COSMOS_INMEMORY_MANAGEMENT_ENDPOINT")?;
    let response = reqwest::Client::new()
        .get(url::Url::parse(&management_endpoint)?.join("capabilities")?)
        .send()
        .await?;
    assert_status(step, response.status().as_u16(), None);
    let capabilities: CapabilityDocument = serde_json::from_slice(&response.bytes().await?)?;
    assert_eq!(capabilities.api_version, 1);
    assert!(!capabilities.emulator_version.is_empty());
    assert!(capabilities.protocols.gateway_v1);
    assert!(capabilities.data_plane.iter().any(|value| value == "item"));
    assert!(capabilities
        .management_actions
        .iter()
        .any(|value| value == "partitionSplit"));
    let expects_v2 = std::env::var("AZURE_COSMOS_EMULATOR_FLAVOR").as_deref() == Ok("inmemory-v2");
    assert_eq!(capabilities.protocols.gateway_v2, expects_v2);
    Ok(())
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn bootstrap_primary_endpoint() -> TestResult {
    let test_scenario = scenario("bootstrap.primary-success");
    let step = test_scenario.step("buildClient");
    let client = build_client().await?;
    assert!(hosted_only());
    let database_id = format!("e2e-bootstrap-{}", azure_core::Uuid::new_v4());
    let response = client.create_database(&database_id, None).await?;
    assert_status(
        step,
        u16::from(response.status().status_code()),
        response.status().sub_status().map(|value| value.value()),
    );
    assert!(response.status().status_code().is_success());
    response.into_model()?;
    client.database_client(&database_id).delete(None).await?;
    Ok(())
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn item_lifecycle() -> TestResult {
    let profile = std::env::var("AZURE_COSMOS_E2E_PROFILE")
        .unwrap_or_else(|_| "hostedEmulatorSmoke".to_owned());
    run_lifecycle_consistency_matrix(&profile).await
}

async fn run_lifecycle_consistency_matrix(profile: &str) -> TestResult {
    let test_scenario = scenario("item.lifecycle");
    let profile_definition = load_profile(profile);
    let account_ids: Vec<_> = profile_definition
        .accounts
        .iter()
        .map(|definition| definition.id.as_str())
        .collect();
    let runtime_ids: Vec<_> = profile_definition
        .runtimes
        .iter()
        .map(|definition| definition.id.as_str())
        .collect();
    let client_ids: Vec<_> = profile_definition
        .clients
        .iter()
        .map(|definition| definition.id.as_str())
        .collect();
    let account_id = selected_axis("AZURE_COSMOS_E2E_ACCOUNT", &account_ids)?;
    let runtime_id = selected_axis("AZURE_COSMOS_E2E_RUNTIME", &runtime_ids)?;
    let client_id = selected_axis("AZURE_COSMOS_E2E_CLIENT", &client_ids)?;
    let executions: Vec<_> = test_scenario
        .executions_for_configuration(profile, account_id, runtime_id, client_id)
        .collect();
    if executions.is_empty() {
        return Err(format!(
            "item.lifecycle has no execution for profile='{profile}', account='{account_id}', runtime='{runtime_id}', client='{client_id}'"
        )
        .into());
    }

    let runtime_strategy = parse_optional_strategy(
        profile_definition
            .runtime(runtime_id)
            .default_read_consistency_strategy
            .as_deref(),
    )?;
    let client_strategy = parse_optional_strategy(
        profile_definition
            .client(client_id)
            .default_read_consistency_strategy
            .as_deref(),
    )?;

    for execution in executions {
        let read_region = match execution.read_region.as_str() {
            "East US" => Region::EAST_US,
            "West US" => Region::WEST_US,
            region => return Err(format!("unsupported lifecycle read region '{region}'").into()),
        };
        let client = build_client_with_defaults(
            RoutingStrategy::PreferredRegions(vec![read_region.clone(), Region::EAST_US]),
            runtime_strategy,
            client_strategy,
        )
        .await?;
        let definition = test_scenario.fixture("hashV2").partition_key_definition()?;

        E2eTestFixture::run_with_client(client, definition, async |fixture| {
            let item_id = format!("lifecycle-{}", execution.id);
            let created = fixture
                .container
                .create_item("A", &item_id, item(&item_id, "A", 1), None)
                .await?;
            assert_eq!(created.status().status_code(), StatusCode::Created);
            let create_token = created.headers().session_token().cloned();

            let mut operation = OperationOptions::default();
            operation.read_consistency_strategy = parse_optional_strategy(
                (execution.read_consistency_strategy != "Inherit")
                    .then_some(execution.read_consistency_strategy.as_str()),
            )?;
            operation.availability_strategy = Some(AvailabilityStrategy::Disabled);
            let mut read_options = ItemReadOptions::default().with_operation_options(operation);
            if execution.session_token == "createResponse" {
                read_options = read_options.with_session_token(
                    create_token
                        .clone()
                        .ok_or("create response must carry a session token")?,
                );
            }

            let deadline = tokio::time::Instant::now()
                + std::time::Duration::from_millis(
                    execution.expected_read.max_wait_ms.unwrap_or_default(),
                );
            let mut observed_statuses = Vec::new();
            loop {
                let (status_code, sub_status_code, terminal) = match fixture
                    .container
                    .read_item("A", &item_id, Some(read_options.clone()))
                    .await
                {
                    Ok(read) => {
                        let status_code = u16::from(read.status().status_code());
                        let sub_status_code =
                            read.status().sub_status().map(|value| value.value());
                        for request in read.diagnostics().requests().iter() {
                            let request_status = u16::from(request.status().status_code());
                            let request_sub_status =
                                request.status().sub_status().map(|value| value.value());
                            if execution
                                .expected_read
                                .is_acceptable_initial(request_status, request_sub_status)
                            {
                                observed_statuses
                                    .push((request_status, request_sub_status.unwrap_or(0)));
                            }
                        }
                        (
                            status_code,
                            sub_status_code,
                            execution
                                .expected_read
                                .is_terminal(status_code, sub_status_code),
                        )
                    }
                    Err(error) => {
                        let status_code = u16::from(error.status().status_code());
                        let sub_status_code =
                            error.status().sub_status().map(|value| value.value());
                        if let Some(diagnostics) = error.diagnostics() {
                            for request in diagnostics.requests().iter() {
                                let request_status = u16::from(request.status().status_code());
                                let request_sub_status =
                                    request.status().sub_status().map(|value| value.value());
                                if execution
                                    .expected_read
                                    .is_acceptable_initial(request_status, request_sub_status)
                                {
                                    observed_statuses
                                        .push((request_status, request_sub_status.unwrap_or(0)));
                                }
                            }
                        }
                        (
                            status_code,
                            sub_status_code,
                            execution
                                .expected_read
                                .is_terminal(status_code, sub_status_code),
                        )
                    }
                };
                observed_statuses.push((status_code, sub_status_code.unwrap_or(0)));
                if terminal {
                    break;
                }
                if !execution
                    .expected_read
                    .is_acceptable_initial(status_code, sub_status_code)
                {
                    return Err(format!(
                        "execution '{}' observed unexpected read status {status_code}/{}; expected transient {:?} or terminal {:?}; observed {observed_statuses:?}",
                        execution.id,
                        sub_status_code.unwrap_or(0),
                        execution.expected_read.acceptable_initial_statuses,
                        execution.expected_read.terminal_status,
                    )
                    .into());
                }
                if tokio::time::Instant::now() >= deadline {
                    return Err(format!(
                        "execution '{}' did not reach terminal status {:?} within {} ms; observed {observed_statuses:?}",
                        execution.id,
                        execution.expected_read.terminal_status,
                        execution.expected_read.max_wait_ms.unwrap_or_default(),
                    )
                    .into());
                }
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }

            let replaced = fixture
                .container
                .replace_item("A", &item_id, item(&item_id, "A", 2), None)
                .await?;
            assert_eq!(replaced.status().status_code(), StatusCode::Ok);
            let deleted = fixture.container.delete_item("A", &item_id, None).await?;
            assert_eq!(deleted.status().status_code(), StatusCode::NoContent);
            Ok(())
        })
        .await?;
    }
    Ok(())
}

fn selected_axis<'a>(environment_variable: &str, available: &'a [&str]) -> TestResult<&'a str> {
    match std::env::var(environment_variable) {
        Ok(selected) => available
            .iter()
            .copied()
            .find(|candidate| *candidate == selected)
            .ok_or_else(|| {
                format!("{environment_variable}='{selected}' is not one of {available:?}").into()
            }),
        Err(_) if available.len() == 1 => Ok(available[0]),
        Err(_) => Err(format!(
            "{environment_variable} is required because this profile defines {available:?}"
        )
        .into()),
    }
}

fn parse_optional_strategy(value: Option<&str>) -> TestResult<Option<ReadConsistencyStrategy>> {
    value
        .map(str::parse::<ReadConsistencyStrategy>)
        .transpose()
        .map_err(Into::into)
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn upsert_creates_then_updates() -> TestResult {
    E2eTestFixture::run(async |fixture| {
        let test_scenario = scenario("item.upsert-create-update");
        let created = fixture
            .container
            .upsert_item(
                "A",
                "upsert-1",
                item("upsert-1", "A", 1),
                Some(write_options_with_content()),
            )
            .await?;
        assert_status(
            test_scenario.step("upsertCreate"),
            u16::from(created.status().status_code()),
            None,
        );
        assert_diagnostics(test_scenario.step("upsertCreate"), &created.diagnostics());
        let updated = fixture
            .container
            .upsert_item(
                "A",
                "upsert-1",
                item("upsert-1", "A", 2),
                Some(write_options_with_content()),
            )
            .await?;
        assert_status(
            test_scenario.step("upsertUpdate"),
            u16::from(updated.status().status_code()),
            None,
        );
        assert_diagnostics(test_scenario.step("upsertUpdate"), &updated.diagnostics());
        assert_eq!(updated.into_model::<Item>()?.value, 2);
        Ok(())
    })
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn duplicate_create_preserves_original() -> TestResult {
    let test_scenario = scenario("item.create-conflict");
    assert_eq!(
        test_scenario
            .fixtures
            .iter()
            .map(|fixture| fixture.id.as_str())
            .collect::<Vec<_>>(),
        ["hashV1", "hashV2", "hierarchicalV2"]
    );

    for scenario_fixture in &test_scenario.fixtures {
        let partition_key_definition = scenario_fixture.partition_key_definition()?;
        let original = scenario_fixture.item("original");
        let duplicate = scenario_fixture.item("duplicate");
        assert!(original.seed);
        assert!(!duplicate.seed);
        assert_eq!(original.document_id()?, duplicate.document_id()?);

        E2eTestFixture::run_with_partition_key(partition_key_definition, async |fixture| {
            fixture
                .container
                .create_item(
                    original.partition_key()?,
                    original.document_id()?,
                    &original.document,
                    None,
                )
                .await?;
            let error = fixture
                .container
                .create_item(
                    duplicate.partition_key()?,
                    duplicate.document_id()?,
                    &duplicate.document,
                    None,
                )
                .await
                .expect_err("duplicate create must fail");
            let step = test_scenario.step("duplicateCreate");
            assert_status(
                step,
                u16::from(error.status().status_code()),
                error.status().sub_status().map(|value| value.value()),
            );
            assert_diagnostics(
                step,
                &error
                    .diagnostics()
                    .expect("service error must carry diagnostics"),
            );
            let stored: serde_json::Value = fixture
                .container
                .read_item(original.partition_key()?, original.document_id()?, None)
                .await?
                .into_model()?;
            for (name, expected) in original
                .document
                .as_object()
                .expect("fixture document must be an object")
            {
                assert_eq!(
                    stored.get(name),
                    Some(expected),
                    "fixture '{}' field '{name}' changed after duplicate create",
                    scenario_fixture.id
                );
            }
            Ok(())
        })
        .await?;
    }
    Ok(())
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn not_found_does_not_cross_partition_keys() -> TestResult {
    E2eTestFixture::run(async |fixture| {
        let test_scenario = scenario("item.not-found-wrong-partition-key");
        fixture
            .container
            .create_item("A", "item-1", item("item-1", "A", 1), None)
            .await?;
        for (step_id, id, pk) in [
            ("missing", "missing", "A"),
            ("wrongPartitionKey", "item-1", "B"),
        ] {
            let error = fixture
                .container
                .read_item(pk, id, None)
                .await
                .expect_err("read must return not found");
            let step = test_scenario.step(step_id);
            assert_status(
                step,
                u16::from(error.status().status_code()),
                error.status().sub_status().map(|value| value.value()),
            );
            assert_diagnostics(
                step,
                &error
                    .diagnostics()
                    .expect("service error must carry diagnostics"),
            );
        }
        let read = fixture.container.read_item("A", "item-1", None).await?;
        assert_eq!(read.into_model::<Item>()?.value, 1);
        Ok(())
    })
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn stale_etag_preserves_successful_update() -> TestResult {
    E2eTestFixture::run(async |fixture| {
        let test_scenario = scenario("item.optimistic-concurrency");
        let created = fixture
            .container
            .create_item("A", "etag-1", item("etag-1", "A", 1), None)
            .await?;
        let initial_etag = created
            .headers()
            .etag()
            .expect("create must return an ETag")
            .clone();
        let current_options = ItemWriteOptions::default()
            .with_precondition(Precondition::IfMatch(initial_etag.clone()));
        let replaced = fixture
            .container
            .replace_item("A", "etag-1", item("etag-1", "A", 2), Some(current_options))
            .await?;
        assert_status(
            test_scenario.step("replaceCurrent"),
            u16::from(replaced.status().status_code()),
            None,
        );
        let stale_options = ItemWriteOptions::default()
            .with_precondition(Precondition::IfMatch(Etag::from(initial_etag.to_string())));
        let error = fixture
            .container
            .replace_item("A", "etag-1", item("etag-1", "A", 3), Some(stale_options))
            .await
            .expect_err("stale ETag must fail");
        let step = test_scenario.step("replaceStale");
        assert_status(
            step,
            u16::from(error.status().status_code()),
            error.status().sub_status().map(|value| value.value()),
        );
        assert_diagnostics(
            step,
            &error
                .diagnostics()
                .expect("service error must carry diagnostics"),
        );
        assert_eq!(
            fixture
                .container
                .read_item("A", "etag-1", None)
                .await?
                .into_model::<Item>()?
                .value,
            2
        );
        Ok(())
    })
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn parameterized_query_filters_and_orders() -> TestResult {
    E2eTestFixture::run(async |fixture| {
        let test_scenario = scenario("query.parameterized-filter");
        for score in 1..=3 {
            let id = format!("item-{score}");
            let mut value = item(&id, "A", score);
            value.score = Some(score);
            fixture.container.create_item("A", &id, value, None).await?;
        }
        let query = Query::from(
            "SELECT * FROM c WHERE c.pk = @pk AND c.score >= @min ORDER BY c.score ASC",
        )
        .with_parameter("@pk", "A")?
        .with_parameter("@min", 2)?;
        let mut results = fixture
            .container
            .query_items::<Item>(query, FeedScope::partition("A"), None)
            .await?;
        let items: Vec<Item> = results.by_ref().try_collect().await?;
        assert_eq!(
            items
                .iter()
                .map(|item| item.id.as_str())
                .collect::<Vec<_>>(),
            ["item-2", "item-3"]
        );
        assert_eq!(test_scenario.step("query").expected.status, 200);
        Ok(())
    })
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn invalid_query_is_not_an_empty_feed() -> TestResult {
    E2eTestFixture::run(async |fixture| {
        let test_scenario = scenario("query.invalid-syntax");
        let result = fixture
            .container
            .query_items::<Item>("SELECT FROM", FeedScope::partition("A"), None)
            .await;
        let error = match result {
            Err(error) => error,
            Ok(mut stream) => stream
                .next()
                .await
                .expect("invalid query must produce an error")
                .expect_err("invalid query must not produce a page"),
        };
        assert_status(
            test_scenario.step("invalidQuery"),
            u16::from(error.status().status_code()),
            error.status().sub_status().map(|value| value.value()),
        );
        Ok(())
    })
    .await
}

#[tokio::test]
#[cfg_attr(
    not(any(test_category = "emulator_inmemory", test_category = "e2e")),
    ignore = "requires the externally hosted in-memory emulator"
)]
async fn diagnostics_cover_success_and_error() -> TestResult {
    E2eTestFixture::run(async |fixture| {
        let test_scenario = scenario("diagnostics.success-and-error");
        fixture
            .container
            .create_item("A", "item-1", item("item-1", "A", 1), None)
            .await?;
        let success = fixture.container.read_item("A", "item-1", None).await?;
        let success_step = test_scenario.step("successfulRead");
        assert_status(
            success_step,
            u16::from(success.status().status_code()),
            None,
        );
        assert_diagnostics(success_step, &success.diagnostics());
        let error = fixture
            .container
            .read_item("A", "missing", None)
            .await
            .expect_err("missing read must fail");
        let error_step = test_scenario.step("missingRead");
        assert_status(
            error_step,
            u16::from(error.status().status_code()),
            error.status().sub_status().map(|value| value.value()),
        );
        assert_diagnostics(
            error_step,
            &error
                .diagnostics()
                .expect("service error must carry diagnostics"),
        );
        Ok(())
    })
    .await
}

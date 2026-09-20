// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::{future::Future, panic::AssertUnwindSafe};

use azure_core::http::StatusCode;
use azure_data_cosmos::{
    clients::ContainerClient,
    diagnostics::{DiagnosticsContext, TransportKind},
    options::{
        AvailabilityStrategy, ContentResponseOnWrite, ItemReadOptions, ItemWriteOptions,
        OperationOptions, ReadConsistencyStrategy, Region,
    },
};
use futures::FutureExt;
use serde::{Deserialize, Serialize};

use crate::e2e_test_cases::{
    catalog::{
        required_capabilities_for, scenario_applies_to_backend, selected_profile_for, Capability,
        Profile,
    },
    fixture::TestResult,
};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub(super) struct Item {
    pub(super) id: String,
    pub(super) pk: String,
    pub(super) value: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(super) score: Option<i64>,
}

pub(super) fn item(id: &str, pk: &str, value: i64) -> Item {
    Item {
        id: id.to_owned(),
        pk: pk.to_owned(),
        value,
        score: None,
    }
}

pub(super) fn write_options_with_content() -> ItemWriteOptions {
    let mut operation = OperationOptions::default();
    operation.content_response_on_write = Some(ContentResponseOnWrite::Enabled);
    ItemWriteOptions::default().with_operation_options(operation)
}

pub(super) async fn wait_for_item_replication(
    container: &ContainerClient,
    item_id: &str,
    expected: &Item,
) -> TestResult {
    let mut operation = OperationOptions::default();
    operation.read_consistency_strategy = Some(ReadConsistencyStrategy::Eventual);
    operation.availability_strategy = Some(AvailabilityStrategy::Disabled);
    operation.max_failover_retry_count = Some(0);
    operation.max_session_retry_count = Some(0);
    operation.excluded_regions =
        Some(azure_data_cosmos::options::ExcludedRegions::new().with_region(Region::EAST_US));
    let options = ItemReadOptions::default().with_operation_options(operation);
    let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(5);
    loop {
        if tokio::time::Instant::now() >= deadline {
            return Err(format!("item '{item_id}' did not replicate before the deadline").into());
        }
        match tokio::time::timeout_at(
            deadline,
            container.read_item("A", item_id, Some(options.clone())),
        )
        .await
        {
            Ok(Ok(response)) => {
                let actual = response.into_model::<Item>()?;
                if &actual == expected {
                    return Ok(());
                }
            }
            Ok(Err(error)) if error.status().status_code() == StatusCode::NotFound => {}
            Ok(Err(error)) => return Err(error.into()),
            Err(_) => {
                return Err(
                    format!("item '{item_id}' did not replicate before the deadline").into(),
                )
            }
        }
        tokio::time::sleep_until(
            deadline.min(tokio::time::Instant::now() + std::time::Duration::from_millis(50)),
        )
        .await;
    }
}

pub(super) async fn should_run(scenario_id: &str) -> TestResult<bool> {
    let Some(profile) = selected_scenario_profile(scenario_id).await? else {
        return Ok(false);
    };
    if profile.accounts.len() != 1 || profile.runtimes.len() != 1 || profile.clients.len() != 1 {
        return Err(format!(
            "scenario '{scenario_id}' requires a single-cell setup profile, got '{}'",
            profile.id
        )
        .into());
    }
    let runtime = &profile.runtimes[0];
    let client = &profile.clients[0];
    if runtime.gateway_v2 != "backendDefault"
        || runtime.ppcb != "sdkDefault"
        || runtime.default_read_consistency_strategy.is_some()
        || client.binary_encoding != "sdkDefault"
        || client.routing != "proximity"
        || client.default_read_consistency_strategy.is_some()
    {
        return Err(
            format!(
            "scenario '{scenario_id}' does not implement the runtime/client settings in profile '{}'",
            profile.id
            ).into()
        );
    }
    Ok(true)
}

pub(super) async fn selected_scenario_profile(scenario_id: &str) -> TestResult<Option<Profile>> {
    init_test_tracing();
    if std::env::var_os("AZURE_COSMOS_EMULATOR_FLAVOR").is_none()
        && std::env::var_os("AZURE_COSMOS_INMEMORY_MANAGEMENT_ENDPOINT").is_none()
        && !scenario_applies_to_backend(scenario_id, "azureLive")?
    {
        eprintln!("SKIP {scenario_id}: scenario is not applicable to Azure Live");
        return Ok(None);
    }
    let Some(profile) = selected_profile_for(scenario_id)? else {
        let selected =
            std::env::var("AZURE_COSMOS_E2E_PROFILE").unwrap_or_else(|_| "smokeTests".to_owned());
        eprintln!("SKIP {scenario_id}: profile '{selected}' does not select it");
        return Ok(None);
    };
    enforce_required_capabilities(scenario_id).await?;
    Ok(Some(profile))
}

fn init_test_tracing() {
    let filter = std::env::var("RUST_LOG")
        .map(tracing_subscriber::EnvFilter::new)
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::from_default_env());
    let _ = tracing_subscriber::fmt::fmt()
        .with_env_filter(filter)
        .try_init();
}

async fn enforce_required_capabilities(scenario_id: &str) -> TestResult {
    let management_endpoint = match std::env::var("AZURE_COSMOS_INMEMORY_MANAGEMENT_ENDPOINT") {
        Ok(endpoint) => endpoint,
        Err(std::env::VarError::NotPresent)
            if std::env::var_os("AZURE_COSMOS_EMULATOR_FLAVOR").is_none() =>
        {
            // Azure Live has no emulator management plane. Account/profile
            // suitability is owned by live pipeline selection, so only hosted
            // emulator runs perform capability-document validation here.
            return Ok(());
        }
        Err(error) => {
            return Err(error.into());
        }
    };
    let response = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?
        .get(url::Url::parse(&management_endpoint)?.join("capabilities")?)
        .send()
        .await?
        .error_for_status()?;
    let capabilities: CapabilityDocument = serde_json::from_slice(&response.bytes().await?)?;
    if capabilities.api_version != 1 {
        return Err(format!(
            "scenario '{scenario_id}' requires capabilities API version 1, got {}",
            capabilities.api_version
        )
        .into());
    }
    let backend = match std::env::var("AZURE_COSMOS_EMULATOR_FLAVOR")
        .ok()
        .as_deref()
    {
        Some("inmemory-v1") => "hostedEmulatorGatewayV1",
        Some("inmemory-v2") => "hostedEmulatorGatewayV2",
        Some(flavor) => {
            return Err(format!(
                "E2E scenario '{scenario_id}' does not support emulator flavor '{flavor}'"
            )
            .into());
        }
        None if capabilities.protocols.gateway_v2 => "hostedEmulatorGatewayV2",
        None => "hostedEmulatorGatewayV1",
    };
    let requirements = required_capabilities_for(scenario_id, backend)?;
    if requirements.is_empty() {
        return Ok(());
    }
    for requirement in requirements {
        let available = match requirement {
            Capability::Capabilities => true,
            Capability::GatewayV2 => capabilities.protocols.gateway_v2,
            Capability::ChangeFeed => capabilities.data_plane_contains("changeFeed"),
            Capability::Container => capabilities.data_plane_contains("container"),
            Capability::Database => capabilities.data_plane_contains("database"),
            Capability::Item => capabilities.data_plane_contains("item"),
            Capability::Patch => capabilities.data_plane_contains("patch"),
            Capability::Query => capabilities.data_plane_contains("query"),
            Capability::TransactionalBatch => {
                capabilities.data_plane_contains("transactionalBatch")
            }
        };
        if !available {
            return Err(format!(
                "required capability '{requirement:?}' is unavailable for scenario '{scenario_id}'"
            )
            .into());
        }
    }
    Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CapabilityDocument {
    api_version: u32,
    protocols: ProtocolCapabilities,
    data_plane: Vec<String>,
}

impl CapabilityDocument {
    fn data_plane_contains(&self, capability: &str) -> bool {
        self.data_plane.iter().any(|value| value == capability)
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ProtocolCapabilities {
    gateway_v2: bool,
}

pub(super) fn assert_critical_diagnostics(
    diagnostics: &DiagnosticsContext,
    operation_name: &str,
    status_code: StatusCode,
) {
    assert_eq!(diagnostics.operation_name(), Some(operation_name));
    assert!(!diagnostics.activity_id().to_string().is_empty());
    assert_eq!(
        diagnostics
            .effective_status()
            .map(|status| status.status_code()),
        Some(status_code)
    );
    assert!(diagnostics.request_count() >= 1);
    assert_diagnostics_transport(diagnostics, configured_emulator_transport());
}

fn configured_emulator_transport() -> Option<TransportKind> {
    match std::env::var("AZURE_COSMOS_EMULATOR_FLAVOR").as_deref() {
        Ok("inmemory-v1") => Some(TransportKind::Gateway),
        Ok("inmemory-v2") => Some(TransportKind::GatewayV2),
        _ => None,
    }
}

pub(super) fn assert_configured_transport(diagnostics: &DiagnosticsContext) {
    assert!(diagnostics.request_count() >= 1);
    assert_diagnostics_transport(diagnostics, configured_emulator_transport());
}

fn assert_diagnostics_transport(
    diagnostics: &DiagnosticsContext,
    expected_transport: Option<TransportKind>,
) {
    if let Some(expected_transport) = expected_transport {
        assert!(
            diagnostics
                .requests()
                .iter()
                .all(|request| request.transport_kind() == expected_transport),
            "completed requests must use {expected_transport:?}"
        );
    }
}

pub(super) fn assert_transport(diagnostics: &DiagnosticsContext, expected: TransportKind) {
    assert!(diagnostics.request_count() >= 1);
    assert!(
        diagnostics
            .requests()
            .iter()
            .all(|request| request.transport_kind() == expected),
        "completed requests must use {expected:?}"
    );
}

pub(super) async fn hosted_wire_counts() -> TestResult<Option<HostedWireCounts>> {
    if std::env::var_os("AZURE_COSMOS_INMEMORY_MANAGEMENT_ENDPOINT").is_none() {
        return Ok(None);
    }
    let endpoint = std::env::var("AZURE_COSMOS_INMEMORY_MANAGEMENT_ENDPOINT")?;
    let response = reqwest::Client::new()
        .get(url::Url::parse(&endpoint)?.join("health")?)
        .send()
        .await?
        .error_for_status()?;
    let counts: HostedWireCounts = serde_json::from_slice(&response.bytes().await?)?;
    Ok(Some(counts))
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct HostedWireCounts {
    pub(super) binary_negotiated_requests: u64,
    pub(super) binary_payload_requests: u64,
    pub(super) binary_response_payloads: u64,
    default_consistency_requests: u64,
    eventual_consistency_requests: u64,
    session_consistency_requests: u64,
    latest_committed_consistency_requests: u64,
    global_strong_consistency_requests: u64,
}

impl HostedWireCounts {
    pub(super) fn consistency_requests(self, strategy: ReadConsistencyStrategy) -> u64 {
        match strategy {
            ReadConsistencyStrategy::Default => self.default_consistency_requests,
            ReadConsistencyStrategy::Eventual => self.eventual_consistency_requests,
            ReadConsistencyStrategy::Session => self.session_consistency_requests,
            ReadConsistencyStrategy::LatestCommitted => self.latest_committed_consistency_requests,
            ReadConsistencyStrategy::GlobalStrong => self.global_strong_consistency_requests,
            _ => 0,
        }
    }
}

pub(super) async fn set_replication_paused(region: &str, paused: bool) -> TestResult<bool> {
    let Some(endpoint) = std::env::var_os("AZURE_COSMOS_INMEMORY_MANAGEMENT_ENDPOINT") else {
        return Ok(false);
    };
    let mut url = url::Url::parse(&endpoint.to_string_lossy())?;
    url.path_segments_mut()
        .map_err(|_| "management endpoint cannot be a base URL")?
        .extend([
            "regions",
            region,
            "replication",
            if paused { "pause" } else { "resume" },
        ]);
    reqwest::Client::new()
        .post(url)
        .send()
        .await?
        .error_for_status()?;
    Ok(true)
}

pub(super) async fn with_replication_paused_if<T, F>(
    should_pause: bool,
    region: &str,
    operation: F,
) -> TestResult<T>
where
    F: Future<Output = TestResult<T>>,
{
    if !should_pause || !set_replication_paused(region, true).await? {
        return operation.await;
    }

    let outcome = AssertUnwindSafe(operation).catch_unwind().await;
    let resume = set_replication_paused(region, false).await;
    match outcome {
        Ok(Ok(value)) => {
            resume?;
            Ok(value)
        }
        Ok(Err(test_error)) => match resume {
            Ok(_) => Err(test_error),
            Err(resume_error) => Err(format!(
                "E2E operation failed: {test_error}; replication resume also failed: {resume_error}"
            )
            .into()),
        },
        Err(panic) => {
            if let Err(error) = resume {
                eprintln!("replication resume after panic failed: {error}");
            }
            std::panic::resume_unwind(panic)
        }
    }
}

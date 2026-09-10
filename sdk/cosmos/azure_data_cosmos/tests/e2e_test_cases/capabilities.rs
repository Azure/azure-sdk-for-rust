// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::Deserialize;

use crate::e2e_test_cases::{fixture::TestResult, support::should_run};

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
    if !should_run("management.capabilities")? {
        return Ok(());
    }
    let management_endpoint = std::env::var("AZURE_COSMOS_INMEMORY_MANAGEMENT_ENDPOINT")?;
    let response = reqwest::Client::new()
        .get(url::Url::parse(&management_endpoint)?.join("capabilities")?)
        .send()
        .await?;
    assert_eq!(response.status(), reqwest::StatusCode::OK);
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

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{models, Audience, KeyVaultClient, KeyVaultClientOptions};
use azure_core::{
    cloud::{Audiences, CloudConfiguration, CustomConfiguration},
    http::{
        headers::{HeaderName, Headers, ACCEPT, AUTHORIZATION, CONTENT_TYPE},
        request::{options::ClientRequestId, Body},
        resourcemanager::PollerOptions,
        AsyncRawResponse, ClientOptions, Context, Method, RetryOptions, StatusCode, Transport,
    },
    Result,
};
use azure_core_test::{credentials::MockCredential, http::MockHttpClient};
use futures::{FutureExt as _, TryStreamExt as _};
use serde_json::{json, Value};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

mod enums;

const VAULT: &str =
    "/subscriptions/sub/resourceGroups/group/providers/Microsoft.KeyVault/vaults/vault";
const HSM: &str =
    "/subscriptions/sub/resourceGroups/group/providers/Microsoft.KeyVault/managedHSMs/hsm";
const OPERATION: &str = "/operations/1";
const REQUEST_ID: HeaderName = HeaderName::from_static("x-ms-client-request-id");

#[tokio::test]
async fn secret_update_includes_properties_and_value() -> Result<()> {
    let mut update = step(
        Method::Patch,
        &format!("{VAULT}/secrets/secret"),
        StatusCode::Ok,
        br#"{"name":"secret","properties":{"contentType":"text/plain"},"tags":{"updated":"true"}}"#,
    );
    update.request_body = Some(json!({
        "properties": {"contentType": "text/plain", "value": "test-value"},
        "tags": {"updated": "true"},
    }));
    let (client, pending) = scripted_client(vec![update])?;
    let secret = client
        .get_key_vault_secrets_client()
        .update(
            "group",
            "vault",
            "secret",
            models::SecretPatchParameters {
                properties: Some(models::SecretPatchProperties {
                    content_type: Some("text/plain".into()),
                    value: Some("test-value".into()),
                    ..Default::default()
                }),
                tags: Some([("updated".into(), "true".into())].into()),
            }
            .try_into()?,
            None,
        )
        .await?
        .into_model()?;
    assert_eq!(
        secret.properties.unwrap().content_type.as_deref(),
        Some("text/plain")
    );
    assert_eq!(secret.tags.unwrap().get("updated").unwrap(), "true");
    assert!(pending.lock().unwrap().is_empty());
    Ok(())
}

struct Step {
    method: Method,
    path: String,
    response: AsyncRawResponse,
    request_body: Option<Value>,
}

fn step(method: Method, path: &str, status: StatusCode, body: &'static [u8]) -> Step {
    let mut headers = Headers::new();
    headers.insert("retry-after-ms", "0");
    Step {
        method,
        path: path.into(),
        response: AsyncRawResponse::from_bytes(status, headers, body),
        request_body: None,
    }
}

fn accepted(method: Method, path: &str, header: &str) -> Step {
    let mut headers = Headers::new();
    headers.insert("retry-after-ms", "0");
    headers.insert(
        HeaderName::from(header.to_string()),
        "/operations/1?api-version=other",
    );
    let mut step = step(
        method,
        path,
        StatusCode::Accepted,
        br#"{"status":"InProgress"}"#,
    );
    step.response = AsyncRawResponse::from_bytes(
        StatusCode::Accepted,
        headers,
        br#"{"status":"InProgress"}"#.as_slice(),
    );
    step
}

fn scripted_client(steps: Vec<Step>) -> Result<(KeyVaultClient, Arc<Mutex<VecDeque<Step>>>)> {
    let pending = Arc::new(Mutex::new(VecDeque::from(steps)));
    let script = pending.clone();
    let mock = MockHttpClient::new(move |request| {
        let step = script
            .lock()
            .unwrap()
            .pop_front()
            .expect("unexpected request");
        assert_eq!(request.method(), step.method);
        assert_eq!(
            request.url().as_str(),
            format!(
                "https://management.azure.com{}?api-version=2026-05-15",
                step.path
            )
        );
        assert_eq!(
            request.headers().get_optional_str(&AUTHORIZATION),
            Some("Bearer TEST TOKEN https://management.azure.com/.default")
        );
        if let Some(expected) = step.request_body {
            assert_eq!(
                request.headers().get_optional_str(&ACCEPT),
                Some("application/json")
            );
            assert_eq!(
                request.headers().get_optional_str(&CONTENT_TYPE),
                Some("application/json")
            );
            let Body::Bytes(body) = request.body() else {
                panic!("expected a JSON request body");
            };
            assert_eq!(serde_json::from_slice::<Value>(body).unwrap(), expected);
        }
        async move { Ok(step.response) }.boxed()
    });
    let client = KeyVaultClient::new(
        "sub".into(),
        MockCredential::new()?,
        Some(KeyVaultClientOptions {
            client_options: ClientOptions {
                transport: Some(Transport::new(Arc::new(mock))),
                retry: RetryOptions::none(),
                ..Default::default()
            },
            ..Default::default()
        }),
    )?;
    Ok((client, pending))
}

#[test]
fn default_and_explicit_public_cloud() -> Result<()> {
    for cloud in [None, Some(Arc::new(CloudConfiguration::AzurePublic))] {
        let client = KeyVaultClient::new(
            "sub".into(),
            MockCredential::new()?,
            Some(KeyVaultClientOptions {
                client_options: ClientOptions {
                    cloud,
                    ..Default::default()
                },
                ..Default::default()
            }),
        )?;
        assert_eq!(client.endpoint().as_str(), "https://management.azure.com/");
    }
    let client = KeyVaultClient::new("sub".into(), MockCredential::new()?, None)?;
    assert_eq!(client.endpoint().as_str(), "https://management.azure.com/");
    Ok(())
}

#[tokio::test]
async fn custom_cloud_uses_supplied_service_root_and_scope() -> Result<()> {
    for audience in [
        "https://custom.example/resource",
        "https://custom.example/resource/",
    ] {
        let mut custom = CustomConfiguration::default();
        custom.authority_host = "https://login.example/".into();
        custom.audiences = Audiences::new().with::<Audience>(audience.into());
        let mock = MockHttpClient::new(|request| {
            assert_eq!(request.url().host_str(), Some("custom.example"));
            assert!(request.url().path().starts_with("/resource/"));
            assert_eq!(
                request.headers().get_optional_str(&AUTHORIZATION),
                Some("Bearer TEST TOKEN https://custom.example/resource/.default")
            );
            async {
                Ok(AsyncRawResponse::from_bytes(
                    StatusCode::Ok,
                    Headers::new(),
                    b"{}".as_slice(),
                ))
            }
            .boxed()
        });
        let client = KeyVaultClient::new(
            "sub".into(),
            MockCredential::new()?,
            Some(KeyVaultClientOptions {
                client_options: ClientOptions {
                    cloud: Some(Arc::new(custom.into())),
                    transport: Some(Transport::new(Arc::new(mock))),
                    ..Default::default()
                },
                ..Default::default()
            }),
        )?;
        assert_eq!(
            client.endpoint().as_str().trim_end_matches('/'),
            audience.trim_end_matches('/')
        );
        client
            .get_key_vault_vaults_client()
            .get("group", "vault", None)
            .await?;
    }
    Ok(())
}

#[test]
fn missing_custom_audience_is_an_error() -> Result<()> {
    let error = KeyVaultClient::new(
        "sub".into(),
        MockCredential::new()?,
        Some(KeyVaultClientOptions {
            client_options: ClientOptions {
                cloud: Some(Arc::new(CustomConfiguration::default().into())),
                ..Default::default()
            },
            ..Default::default()
        }),
    )
    .err()
    .expect("missing audience must fail");
    assert!(error
        .to_string()
        .contains("missing custom cloud configuration audience"));
    Ok(())
}

#[test]
fn unfinished_clouds_are_explicit() {
    for cloud in [
        CloudConfiguration::AzureGovernment,
        CloudConfiguration::AzureChina,
    ] {
        let result = std::panic::catch_unwind(|| {
            KeyVaultClient::new(
                "sub".into(),
                MockCredential::new().unwrap(),
                Some(KeyVaultClientOptions {
                    client_options: ClientOptions {
                        cloud: Some(Arc::new(cloud)),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            )
        });
        assert!(result.is_err());
    }
}

#[tokio::test]
async fn resource_pollers_fetch_final_resources() -> Result<()> {
    for operation in 0..3 {
        let (path, method) = match operation {
            0 => (VAULT, Method::Put),
            1 => (HSM, Method::Put),
            _ => (HSM, Method::Patch),
        };
        let mut initial = accepted(method, path, "azure-asyncoperation");
        initial.request_body = Some(json!({"location":"westus"}));
        let (client, pending) = scripted_client(vec![
            initial,
            step(
                Method::Get,
                OPERATION,
                StatusCode::Ok,
                br#"{"status":"Succeeded"}"#,
            ),
            step(
                Method::Get,
                path,
                StatusCode::Ok,
                br#"{"name":"result","properties":{"provisioningState":"Succeeded"}}"#,
            ),
        ])?;
        let name = match operation {
            0 => {
                client
                    .get_key_vault_vaults_client()
                    .begin_create_or_update(
                        "group",
                        "vault",
                        models::VaultCreateOrUpdateParameters {
                            location: Some("westus".into()),
                            ..Default::default()
                        }
                        .try_into()?,
                        None,
                    )?
                    .await?
                    .into_model()?
                    .name
            }
            1 => {
                client
                    .get_key_vault_managed_hsms_client()
                    .begin_create_or_update(
                        "group",
                        "hsm",
                        models::ManagedHsm {
                            location: Some("westus".into()),
                            ..Default::default()
                        }
                        .try_into()?,
                        None,
                    )?
                    .await?
                    .into_model()?
                    .name
            }
            _ => {
                client
                    .get_key_vault_managed_hsms_client()
                    .begin_update(
                        "group",
                        "hsm",
                        models::ManagedHsm {
                            location: Some("westus".into()),
                            ..Default::default()
                        }
                        .try_into()?,
                        None,
                    )?
                    .await?
                    .into_model()?
                    .name
            }
        };
        assert_eq!(name.as_deref(), Some("result"));
        assert!(pending.lock().unwrap().is_empty());
    }
    Ok(())
}

#[tokio::test]
async fn delete_and_purge_pollers_complete_without_final_get() -> Result<()> {
    for operation in 0..5 {
        let path = match operation {
            0 => HSM.into(),
            1 => "/subscriptions/sub/providers/Microsoft.KeyVault/locations/westus/deletedManagedHSMs/hsm/purge".into(),
            2 => "/subscriptions/sub/providers/Microsoft.KeyVault/locations/westus/deletedVaults/vault/purge".into(),
            3 => format!("{VAULT}/privateEndpointConnections/connection"),
            _ => format!("{HSM}/privateEndpointConnections/connection"),
        };
        let method = if matches!(operation, 1 | 2) {
            Method::Post
        } else {
            Method::Delete
        };
        let (client, pending) = scripted_client(vec![
            accepted(method, &path, "location"),
            step(Method::Get, OPERATION, StatusCode::NoContent, b""),
        ])?;
        let status = match operation {
            0 => client
                .get_key_vault_managed_hsms_client()
                .begin_delete("group", "hsm", None)?
                .await?
                .status(),
            1 => client
                .get_key_vault_managed_hsms_client()
                .begin_purge_deleted("westus", "hsm", None)?
                .await?
                .status(),
            2 => client
                .get_key_vault_vaults_client()
                .begin_purge_deleted("westus", "vault", None)?
                .await?
                .status(),
            3 => client
                .get_key_vault_private_endpoint_connections_client()
                .begin_delete("group", "vault", "connection", None)?
                .await?
                .status(),
            _ => client
                .get_key_vault_mhsm_private_endpoint_connections_client()
                .begin_delete("group", "hsm", "connection", None)?
                .await?
                .status(),
        };
        assert_eq!(status, StatusCode::NoContent);
        assert!(pending.lock().unwrap().is_empty());
    }
    Ok(())
}

#[tokio::test]
async fn synchronous_resource_completion_is_lazy_and_reuses_response() -> Result<()> {
    let (client, pending) = scripted_client(vec![step(
        Method::Put,
        VAULT,
        StatusCode::Created,
        br#"{"name":"vault","properties":{"provisioningState":"Succeeded"}}"#,
    )])?;
    let poller = client
        .get_key_vault_vaults_client()
        .begin_create_or_update(
            "group",
            "vault",
            models::VaultCreateOrUpdateParameters::default().try_into()?,
            None,
        )?;
    assert_eq!(pending.lock().unwrap().len(), 1);
    assert_eq!(poller.await?.into_model()?.name.as_deref(), Some("vault"));
    assert!(pending.lock().unwrap().is_empty());
    Ok(())
}

#[tokio::test]
async fn explicit_location_metadata_overrides_operation_resource_location() -> Result<()> {
    let mut headers = Headers::new();
    headers.insert("operation-location", "/operations/1?api-version=other");
    headers.insert("location", "/results/1?api-version=other");
    headers.insert("retry-after-ms", "0");
    let initial = Step {
        method: Method::Put,
        path: VAULT.into(),
        response: AsyncRawResponse::from_bytes(
            StatusCode::Accepted,
            headers,
            br#"{"status":"InProgress"}"#.as_slice(),
        ),
        request_body: None,
    };
    let (client, pending) = scripted_client(vec![
        initial,
        step(
            Method::Get,
            OPERATION,
            StatusCode::Ok,
            br#"{"status":"Succeeded","resourceLocation":"/wrong-result"}"#,
        ),
        step(
            Method::Get,
            "/results/1",
            StatusCode::Ok,
            br#"{"name":"vault"}"#,
        ),
    ])?;
    let vault = client
        .get_key_vault_vaults_client()
        .begin_create_or_update(
            "group",
            "vault",
            models::VaultCreateOrUpdateParameters::default().try_into()?,
            None,
        )?
        .await?
        .into_model()?;
    assert_eq!(vault.name.as_deref(), Some("vault"));
    assert!(pending.lock().unwrap().is_empty());
    Ok(())
}

#[tokio::test]
async fn streamed_monitors_can_be_followed_by_awaiting() -> Result<()> {
    let (client, pending) = scripted_client(vec![
        accepted(Method::Put, VAULT, "location"),
        step(
            Method::Get,
            OPERATION,
            StatusCode::Ok,
            br#"{"status":"Running"}"#,
        ),
        step(
            Method::Get,
            OPERATION,
            StatusCode::Ok,
            br#"{"status":"Succeeded","name":"vault"}"#,
        ),
    ])?;
    let mut poller = client
        .get_key_vault_vaults_client()
        .begin_create_or_update(
            "group",
            "vault",
            models::VaultCreateOrUpdateParameters::default().try_into()?,
            None,
        )?;
    let mut statuses = Vec::new();
    while let Some(response) = poller.try_next().await? {
        statuses.push(response.into_model()?.status.unwrap().to_string());
    }
    assert_eq!(statuses, ["InProgress", "Running", "Succeeded"]);
    assert_eq!(poller.await?.into_model()?.name.as_deref(), Some("vault"));
    assert!(pending.lock().unwrap().is_empty());
    Ok(())
}

#[tokio::test]
async fn failures_propagate_without_final_requests() -> Result<()> {
    for state in ["Failed", "Canceled"] {
        let body = if state == "Failed" {
            br#"{"status":"Failed"}"#.as_slice()
        } else {
            br#"{"status":"Canceled"}"#.as_slice()
        };
        let (client, pending) = scripted_client(vec![
            accepted(Method::Put, VAULT, "azure-asyncoperation"),
            step(Method::Get, OPERATION, StatusCode::Ok, body),
        ])?;
        let error = client
            .get_key_vault_vaults_client()
            .begin_create_or_update(
                "group",
                "vault",
                models::VaultCreateOrUpdateParameters::default().try_into()?,
                None,
            )?
            .await
            .err()
            .expect("terminal failure must propagate");
        assert!(error.to_string().contains(if state == "Failed" {
            "failed"
        } else {
            "canceled"
        }));
        assert!(pending.lock().unwrap().is_empty());
    }
    let (client, pending) = scripted_client(vec![step(
        Method::Put,
        VAULT,
        StatusCode::Forbidden,
        br#"{"error":{"code":"Forbidden","message":"denied"}}"#,
    )])?;
    let error = client
        .get_key_vault_vaults_client()
        .begin_create_or_update(
            "group",
            "vault",
            models::VaultCreateOrUpdateParameters::default().try_into()?,
            None,
        )?
        .await
        .err()
        .expect("HTTP failure must propagate");
    assert_eq!(error.http_status(), Some(StatusCode::Forbidden));
    assert!(pending.lock().unwrap().is_empty());
    Ok(())
}

#[tokio::test]
async fn borrowed_context_reaches_initial_poll_and_final_requests() -> Result<()> {
    let requests = Arc::new(Mutex::new(0));
    let count = requests.clone();
    let mock = MockHttpClient::new(move |request| {
        assert_eq!(
            request.headers().get_optional_str(&REQUEST_ID),
            Some("caller-id")
        );
        let mut count = count.lock().unwrap();
        *count += 1;
        let response = match *count {
            1 => accepted(Method::Put, VAULT, "azure-asyncoperation").response,
            2 => {
                step(
                    Method::Get,
                    OPERATION,
                    StatusCode::Ok,
                    br#"{"status":"Succeeded"}"#,
                )
                .response
            }
            3 => step(Method::Get, VAULT, StatusCode::Ok, br#"{"name":"vault"}"#).response,
            _ => panic!("unexpected request"),
        };
        async move { Ok(response) }.boxed()
    });
    let client = KeyVaultClient::new(
        "sub".into(),
        MockCredential::new()?,
        Some(KeyVaultClientOptions {
            client_options: ClientOptions {
                transport: Some(Transport::new(Arc::new(mock))),
                ..Default::default()
            },
            ..Default::default()
        }),
    )?;
    let context = Context::new().with_value(ClientRequestId::new("caller-id"));
    let poller = client
        .get_key_vault_vaults_client()
        .begin_create_or_update(
            "group",
            "vault",
            models::VaultCreateOrUpdateParameters::default().try_into()?,
            Some(models::KeyVaultVaultsClientBeginCreateOrUpdateOptions {
                method_options: PollerOptions {
                    context: context.to_borrowed(),
                    ..Default::default()
                },
            }),
        )?;
    drop(context);
    assert_eq!(poller.await?.into_model()?.name.as_deref(), Some("vault"));
    assert_eq!(*requests.lock().unwrap(), 3);
    Ok(())
}

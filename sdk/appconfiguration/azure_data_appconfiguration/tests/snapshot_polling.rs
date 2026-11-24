// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{
    credentials::{AccessToken, TokenCredential, TokenRequestOptions},
    http::{
        headers::Headers, AsyncRawResponse, ClientOptions, HttpClient, Method, Request, StatusCode,
        Transport,
    },
    time::{Duration, OffsetDateTime},
    Bytes, Result,
};
use azure_data_appconfiguration::{
    models::{
        ConfigurationSettingsFilter, ConfigurationSnapshot, CreateSnapshotRequestContentType,
    },
    ConfigurationClient, ConfigurationClientOptions,
};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[derive(Debug)]
struct Credential;

#[async_trait]
impl TokenCredential for Credential {
    async fn get_token(
        &self,
        _: &[&str],
        _: Option<TokenRequestOptions<'_>>,
    ) -> Result<AccessToken> {
        Ok(AccessToken::new(
            "synthetic-token",
            OffsetDateTime::now_utc() + Duration::hours(1),
        ))
    }
}

#[derive(Debug)]
struct SnapshotTransport {
    initial_status: &'static str,
    requests: Arc<AtomicUsize>,
}

#[async_trait]
impl HttpClient for SnapshotTransport {
    async fn execute_request(&self, request: &Request) -> Result<AsyncRawResponse> {
        let step = self.requests.fetch_add(1, Ordering::SeqCst);
        println!("request {step}: {} {}", request.method(), request.url());
        let mut headers = Headers::new();
        headers.insert("content-type", "application/json");
        headers.insert("retry-after", "0");
        let (status, body) = match step {
            0 => {
                assert_eq!(request.method(), Method::Put);
                assert_eq!(request.url().path(), "/snapshots/repro");
                headers.insert(
                    "operation-location",
                    "https://example.azconfig.io/operations/snapshot-repro",
                );
                (
                    StatusCode::Created,
                    format!(
                        r#"{{"name":"repro","status":"{}","filters":[{{"key":"repro:*"}}]}}"#,
                        self.initial_status
                    ),
                )
            }
            1 => {
                assert_eq!(request.method(), Method::Get);
                assert_eq!(request.url().path(), "/operations/snapshot-repro");
                (StatusCode::Ok, r#"{"status":"Succeeded"}"#.into())
            }
            2 => {
                assert_eq!(request.method(), Method::Get);
                assert_eq!(request.url().path(), "/snapshots/repro");
                (
                    StatusCode::Ok,
                    r#"{"name":"repro","status":"ready","filters":[{"key":"repro:*"}]}"#.into(),
                )
            }
            _ => panic!("unexpected request after final snapshot GET"),
        };
        Ok(AsyncRawResponse::from_bytes(
            status,
            headers,
            Bytes::from(body),
        ))
    }
}

async fn create_snapshot(initial_status: &'static str) -> Result<()> {
    let requests = Arc::new(AtomicUsize::new(0));
    let client = ConfigurationClient::new(
        "https://example.azconfig.io",
        Arc::new(Credential),
        Some(ConfigurationClientOptions {
            client_options: ClientOptions {
                transport: Some(Transport::new(Arc::new(SnapshotTransport {
                    initial_status,
                    requests: requests.clone(),
                }))),
                ..Default::default()
            },
            ..Default::default()
        }),
    )?;
    let snapshot = ConfigurationSnapshot {
        filters: Some(vec![ConfigurationSettingsFilter {
            key: Some("repro:*".into()),
            ..Default::default()
        }]),
        ..Default::default()
    };
    let result = client
        .begin_create_snapshot(
            CreateSnapshotRequestContentType::ApplicationJson,
            "repro",
            snapshot.try_into()?,
            None,
        )?
        .await;
    println!(
        "initial status={initial_status}, total requests={}, error={:?}",
        requests.load(Ordering::SeqCst),
        result.as_ref().err()
    );
    let snapshot = result?.into_model()?;
    assert_eq!(snapshot.name.as_deref(), Some("repro"));
    assert_eq!(requests.load(Ordering::SeqCst), 3);
    Ok(())
}

#[tokio::test]
async fn initial_snapshot_provisioning_follows_operation_location() -> Result<()> {
    create_snapshot("provisioning").await
}

#[tokio::test]
async fn initial_operation_in_progress_positive_control() -> Result<()> {
    create_snapshot("InProgress").await
}

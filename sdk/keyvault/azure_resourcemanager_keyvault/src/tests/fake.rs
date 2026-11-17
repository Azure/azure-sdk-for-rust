// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use super::{models, KeyVaultClient};
use crate::clients::fake::{KeyVaultKeysServer, KeyVaultOperationsServer, KeyVaultServer};
use async_trait::async_trait;
use azure_core::{
    http::{
        fake::{ResponseExt as _, ServerFactory},
        headers::Headers,
        pager::{PagerContinuation, PagerState},
        AsyncRawResponse, StatusCode,
    },
    Result,
};
use futures::TryStreamExt as _;
use std::sync::{Arc, Mutex};

const ENDPOINT: &str = "https://management.azure.com";
const NEXT_LINK: &str = "https://management.azure.com/operations?cursor=next";

#[derive(ServerFactory)]
struct KeysOnly;

#[async_trait]
impl KeyVaultKeysServer for KeysOnly {
    async fn get(
        &self,
        resource_group_name: &str,
        vault_name: &str,
        key_name: &str,
        _options: Option<models::KeyVaultKeysClientGetOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        assert_eq!(resource_group_name, "group");
        assert_eq!(vault_name, "vault");
        match key_name {
            "key" => Ok(AsyncRawResponse::fake_with_bytes(
                StatusCode::Ok,
                Headers::new(),
                r#"{"name":"key"}"#,
            )),
            _ => AsyncRawResponse::fake_with_error(
                StatusCode::NotFound,
                Headers::new(),
                "KeyNotFound",
                "The requested key was not found.",
            ),
        }
    }
}

#[tokio::test]
async fn keys_only_server_handles_success_and_service_error() -> Result<()> {
    let server: Arc<dyn KeyVaultServer> = Arc::new(KeysOnly);
    let client = KeyVaultClient::with_fake(ENDPOINT, server, "subscription".into(), None)?;
    let keys = client.get_key_vault_keys_client();
    let key = keys
        .get("group", "vault", "key", None)
        .await?
        .into_model()?;
    assert_eq!(key.name.as_deref(), Some("key"));

    let error = keys
        .get("group", "vault", "missing", None)
        .await
        .expect_err("missing key should return a service error");
    assert_eq!(error.http_status(), Some(StatusCode::NotFound));
    Ok(())
}

#[derive(Default)]
struct Operations {
    states: Mutex<Vec<PagerState>>,
}

#[async_trait]
impl KeyVaultOperationsServer for Operations {
    async fn list(
        &self,
        state: PagerState,
        options: Option<models::KeyVaultOperationsClientListOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        let options = options.expect("pager options are forwarded");
        self.states.lock().unwrap().push(state.clone());
        let body = match state {
            PagerState::Initial => {
                assert!(options.method_options.continuation.is_none());
                format!(r#"{{"value":[{{"name":"first"}}],"nextLink":"{NEXT_LINK}"}}"#)
            }
            PagerState::More(continuation) => {
                assert_eq!(continuation, PagerContinuation::Link(NEXT_LINK.parse()?));
                assert_eq!(options.method_options.continuation, Some(continuation));
                r#"{"value":[{"name":"second"}]}"#.into()
            }
        };
        Ok(AsyncRawResponse::fake_with_bytes(
            StatusCode::Ok,
            Headers::new(),
            body,
        ))
    }
}

#[derive(Default)]
struct DelegatingServer {
    operations: Operations,
}

impl KeyVaultServer for DelegatingServer {
    fn as_key_vault_operations(&self) -> &dyn KeyVaultOperationsServer {
        &self.operations
    }
}

#[tokio::test]
async fn separate_child_server_handles_paging() -> Result<()> {
    let server = Arc::new(DelegatingServer::default());
    let client = KeyVaultClient::with_fake(ENDPOINT, server.clone(), "subscription".into(), None)?;
    let operations = client.get_key_vault_operations_client();
    let mut pages = operations.list(None)?.into_pages();
    for expected in ["first", "second"] {
        let page = pages
            .try_next()
            .await?
            .expect("expected a page")
            .into_model()?;
        assert_eq!(page.value.len(), 1);
        assert_eq!(page.value[0].name.as_deref(), Some(expected));
    }
    assert!(pages.try_next().await?.is_none());
    assert_eq!(
        *server.operations.states.lock().unwrap(),
        [
            PagerState::Initial,
            PagerState::More(PagerContinuation::Link(NEXT_LINK.parse()?))
        ]
    );
    Ok(())
}

#[tokio::test]
#[should_panic(expected = "KeyVaultServer::as_key_vault_secrets is not implemented")]
async fn missing_child_server_panics_on_use() {
    let client =
        KeyVaultClient::with_fake(ENDPOINT, Arc::new(KeysOnly), "subscription".into(), None)
            .unwrap();
    let _ = client
        .get_key_vault_secrets_client()
        .get("group", "vault", "secret", None)
        .await;
}

#[tokio::test]
#[should_panic(expected = "KeyVaultKeysServer::get_version is not implemented")]
async fn missing_child_operation_panics_on_use() {
    let client =
        KeyVaultClient::with_fake(ENDPOINT, Arc::new(KeysOnly), "subscription".into(), None)
            .unwrap();
    let _ = client
        .get_key_vault_keys_client()
        .get_version("group", "vault", "key", "version", None)
        .await;
}

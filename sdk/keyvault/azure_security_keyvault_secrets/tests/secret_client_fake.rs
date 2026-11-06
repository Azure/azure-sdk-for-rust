// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{
    http::{
        fake::ResponseExt as _,
        headers::Headers,
        pager::{PagerContinuation, PagerOptions, PagerState},
        AsyncRawResponse, Context, StatusCode,
    },
    Result,
};
use azure_security_keyvault_secrets::{
    clients::fake::SecretServer, models::SecretClientListSecretPropertiesOptions, ResourceExt as _,
    SecretClient,
};
use futures::TryStreamExt as _;
use serde_json::json;
use std::sync::{Arc, Mutex};

const ENDPOINT: &str = "https://my-vault.vault.azure.net";
const NEXT_LINK: &str = "https://my-vault.vault.azure.net/opaque?cursor=a%2Fb%3D";

#[derive(Default)]
struct FakeSecrets {
    states: Mutex<Vec<PagerState>>,
    marker: Option<u32>,
}

#[async_trait]
impl SecretServer for FakeSecrets {
    async fn list_secret_properties(
        &self,
        state: PagerState,
        options: Option<SecretClientListSecretPropertiesOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        let options = options.expect("the fake transport forwards options");
        assert_eq!(
            options.method_options.context.value::<PagerState>(),
            Some(&state)
        );
        assert_eq!(
            options.method_options.context.value::<u32>().copied(),
            self.marker
        );
        self.states.lock().unwrap().push(state.clone());

        let body = match state {
            PagerState::Initial => {
                assert_eq!(options.method_options.continuation, None);
                json!({
                    "value": [{"id": format!("{ENDPOINT}/secrets/first")}],
                    "nextLink": NEXT_LINK,
                })
            }
            PagerState::More(continuation) => {
                assert_eq!(continuation, PagerContinuation::Link(NEXT_LINK.parse()?));
                assert_eq!(options.method_options.continuation, Some(continuation));
                json!({"value": [{"id": format!("{ENDPOINT}/secrets/second")}]})
            }
        };
        Ok(AsyncRawResponse::fake_with_bytes(
            StatusCode::Ok,
            Headers::new(),
            body.to_string(),
        ))
    }
}

#[tokio::test]
async fn pager_propagates_state_across_pages() -> Result<()> {
    let server = Arc::new(FakeSecrets::default());
    let client = SecretClient::with_fake(ENDPOINT, server.clone(), None)?;
    let mut pager = client.list_secret_properties(None)?;

    for name in ["first", "second"] {
        let secret = pager.try_next().await?.expect("expected a secret");
        assert_eq!(secret.resource_id()?.name, name);
    }
    assert!(pager.try_next().await?.is_none());
    assert_eq!(
        *server.states.lock().unwrap(),
        [
            PagerState::Initial,
            PagerState::More(PagerContinuation::Link(NEXT_LINK.parse()?)),
        ]
    );
    Ok(())
}

#[tokio::test]
async fn pager_resumes_on_a_fresh_fake_client() -> Result<()> {
    let server = Arc::new(FakeSecrets::default());
    let client = SecretClient::with_fake(ENDPOINT, server.clone(), None)?;
    let mut pages = client.list_secret_properties(None)?.into_pages();
    let page = pages.try_next().await?.expect("expected the first page");
    let page = page.into_model()?;
    assert_eq!(page.value.len(), 1);
    assert_eq!(page.value[0].resource_id()?.name, "first");
    let continuation = pages.into_continuation().expect("expected a continuation");
    assert_eq!(continuation, PagerContinuation::Link(NEXT_LINK.parse()?));
    assert_eq!(*server.states.lock().unwrap(), [PagerState::Initial]);

    // A continuation can resume paging on another application instance, such as a Kubernetes pod.
    let resumed_server = Arc::new(FakeSecrets {
        marker: Some(42),
        ..Default::default()
    });
    let resumed_client = SecretClient::with_fake(ENDPOINT, resumed_server.clone(), None)?;
    let context = Context::new()
        .with_value(42u32)
        .with_value(PagerState::Initial);
    let mut resumed =
        resumed_client.list_secret_properties(Some(SecretClientListSecretPropertiesOptions {
            method_options: PagerOptions {
                context: context.clone(),
                continuation: Some(continuation.clone()),
            },
            ..Default::default()
        }))?;

    let secret = resumed
        .try_next()
        .await?
        .expect("expected the remaining secret");
    assert_eq!(secret.resource_id()?.name, "second");
    assert!(resumed.try_next().await?.is_none());
    assert_eq!(
        *resumed_server.states.lock().unwrap(),
        [PagerState::More(continuation)]
    );
    assert_eq!(context.value::<PagerState>(), Some(&PagerState::Initial));
    Ok(())
}

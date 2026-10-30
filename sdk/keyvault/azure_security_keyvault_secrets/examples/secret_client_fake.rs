// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{
    http::{
        fake::ResponseExt as _, headers::Headers, pager::PagerState, AsyncRawResponse, StatusCode,
    },
    Result,
};
use azure_security_keyvault_secrets::{
    clients::fake::SecretServer,
    models::{SecretClientGetSecretOptions, SecretClientListSecretPropertiesOptions},
    ResourceExt as _, SecretClient,
};
use futures::TryStreamExt as _;
use serde_json::json;
use std::sync::Arc;

const ENDPOINT: &str = "https://example.vault.azure.net";
const SECRETS: [(&str, &str); 2] = [("first", "first-value"), ("second", "second-value")];

struct FakeSecrets;

// Only implement operations the application uses. Other operations panic by default.
#[async_trait]
impl SecretServer for FakeSecrets {
    async fn list_secret_properties(
        &self,
        _state: PagerState,
        _options: Option<SecretClientListSecretPropertiesOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        // Listing returns metadata, not values. Omitting nextLink ends the pager after this page.
        let secrets: Vec<_> = SECRETS
            .iter()
            .map(|(name, _)| json!({ "id": format!("{ENDPOINT}/secrets/{name}") }))
            .collect();
        AsyncRawResponse::fake_with_model(
            StatusCode::Ok,
            Headers::new(),
            json!({ "value": secrets }),
        )
    }

    async fn get_secret(
        &self,
        secret_name: &str,
        _options: Option<SecretClientGetSecretOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        match SECRETS.iter().find(|(name, _)| *name == secret_name) {
            Some((name, value)) => AsyncRawResponse::fake_with_model(
                StatusCode::Ok,
                Headers::new(),
                json!({ "id": format!("{ENDPOINT}/secrets/{name}"), "value": value }),
            ),
            // Return an HTTP response so the client applies its normal service-error handling.
            None => AsyncRawResponse::fake_with_error(
                StatusCode::NotFound,
                Headers::new(),
                "SecretNotFound",
                "The requested secret was not found.",
            ),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // The `with_fake` function is only available when the `unstable_fake` feature is enabled.
    let client = SecretClient::with_fake(ENDPOINT, Arc::new(FakeSecrets), None)?;

    // The usual client and pager APIs run without credentials or network access.
    let mut secrets = client.list_secret_properties(None)?;
    while let Some(properties) = secrets.try_next().await? {
        let name = properties.resource_id()?.name;
        let secret = client.get_secret(&name, None).await?.into_model()?;
        println!(
            "{name}: {}",
            secret.value.expect("the fake returns a value")
        );
    }

    // A missing name exercises the application's error path with the same client.
    let error = client
        .get_secret("missing", None)
        .await
        .expect_err("the fake returns a 404 for an unknown secret");
    assert_eq!(error.http_status(), Some(StatusCode::NotFound));
    println!("Expected error: {error}");
    Ok(())
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{
    http::{fake::ResponseExt as _, headers::Headers, AsyncRawResponse, StatusCode},
    Result,
};
use azure_security_keyvault_keys::{
    clients::fake::KeyServer,
    models::{KeyClientGetKeyOptions, KeyType},
    KeyClient,
};
use std::sync::Arc;

struct FakeKeys;

#[async_trait]
impl KeyServer for FakeKeys {
    async fn get_key(
        &self,
        key_name: &str,
        _options: Option<KeyClientGetKeyOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        match key_name {
            "my-key" => Ok(AsyncRawResponse::fake_with_bytes(
                StatusCode::Ok,
                Headers::new(),
                r#"{"key":{"kty":"RSA"}}"#,
            )),
            // Return an HTTP response so the client applies its normal service-error handling.
            _ => AsyncRawResponse::fake_with_error(
                StatusCode::NotFound,
                Headers::new(),
                "KeyNotFound",
                "The requested key was not found.",
            ),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = KeyClient::with_fake("https://example.vault.azure.net", Arc::new(FakeKeys), None)?;

    let key = client.get_key("my-key", None).await?.into_model()?;
    assert_eq!(key.key.and_then(|key| key.kty), Some(KeyType::Rsa));
    println!("Retrieved an RSA key.");

    let error = client
        .get_key("missing", None)
        .await
        .expect_err("the fake returns a 404 for an unknown key");
    assert_eq!(error.http_status(), Some(StatusCode::NotFound));
    println!("Expected error: {error}");
    Ok(())
}

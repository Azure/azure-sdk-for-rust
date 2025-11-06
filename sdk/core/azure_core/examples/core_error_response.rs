// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    credentials::TokenCredential,
    error::ErrorResponse,
    http::{headers::Headers, AsyncRawResponse, HttpClient, StatusCode, Transport},
    json,
};
use azure_core_test::{credentials::MockCredential, http::MockHttpClient, ErrorKind};
use azure_security_keyvault_secrets::{
    models::SetSecretParameters, SecretClient, SecretClientOptions,
};
use futures::FutureExt;
use std::sync::Arc;

/// This example demonstrates deserializing a standard Azure error response to get more details.
async fn test_error_response() -> Result<(), Box<dyn std::error::Error>> {
    let mut options = SecretClientOptions::default();

    // Ignore: this is only set up for testing.
    // You normally would create credentials from `azure_identity` and
    // use the default transport in production.
    let (credential, transport) = setup()?;
    options.client_options.transport = Some(Transport::new(transport));

    let client = SecretClient::new(
        "https://my-vault.vault.azure.net",
        credential,
        Some(options),
    )?;

    let secret = SetSecretParameters {
        value: Some("secret_value".into()),
        content_type: Some("text/plain".into()),
        ..Default::default()
    };

    // This would fail with a name like "secret_name".
    let error = client
        .set_secret("secret_name", secret.try_into()?, None)
        .await
        .unwrap_err();
    let ErrorKind::HttpResponse {
        status,
        error_code,
        raw_response: Some(raw_response),
    } = error.kind()
    else {
        panic!("expected HTTP error response");
    };

    assert_eq!(*status, StatusCode::BadRequest);
    assert_eq!(error_code.as_deref(), Some("BadParameter"));
    assert!(error
        .to_string()
        .contains("The request URI contains an invalid name: secret_name"));

    // Now deserialize the `raw_response` to get additional details.
    let error_response: ErrorResponse = json::from_json(raw_response.body())?;
    let details = error_response
        .error
        .expect("expected HTTP error response details");

    assert_eq!(details.code.as_deref(), Some("BadParameter"));
    assert_eq!(
        details.message.as_deref(),
        Some("The request URI contains an invalid name: secret_name")
    );
    assert_eq!(details.target.as_deref(), Some("secret-name"));
    assert_eq!(details.details.len(), 1);
    assert_eq!(
        details.details[0].message.as_deref(),
        Some("secret-name can contain only letters, numbers, and dashes")
    );

    Ok(())
}

// ----- BEGIN TEST SETUP -----
#[tokio::test]
async fn test_core_error_response() -> Result<(), Box<dyn std::error::Error>> {
    test_error_response().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test_error_response().await
}

#[allow(clippy::type_complexity)]
fn setup() -> Result<(Arc<dyn TokenCredential>, Arc<dyn HttpClient>), Box<dyn std::error::Error>> {
    let client = MockHttpClient::new(|_| {
        async move {
            Ok(AsyncRawResponse::from_bytes(
                StatusCode::BadRequest,
                Headers::new(),
                r#"{
                    "error": {
                        "code": "BadParameter",
                        "message": "The request URI contains an invalid name: secret_name",
                        "target": "secret-name",
                        "details": [
                            {"message": "secret-name can contain only letters, numbers, and dashes"}
                        ]
                    }
                }"#,
            ))
        }
        .boxed()
    });

    Ok((MockCredential::new()?, Arc::new(client)))
}
// ----- END TEST SETUP -----

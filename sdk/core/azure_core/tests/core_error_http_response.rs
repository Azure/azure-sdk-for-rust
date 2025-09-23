// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    error::{ErrorKind, ErrorResponse},
    http::{headers::Headers, BufResponse, ClientOptions, HttpClient, StatusCode, Transport},
    Bytes,
};
use azure_core_test::{credentials::MockCredential, http::MockHttpClient};
use azure_security_keyvault_secrets::{SecretClient, SecretClientOptions};
use futures::FutureExt as _;
use std::{
    str,
    sync::{Arc, LazyLock},
};

#[tokio::test]
async fn deconstruct_raw_response() -> Result<(), Box<dyn std::error::Error>> {
    let options = SecretClientOptions {
        client_options: ClientOptions {
            transport: Some(Transport::new(TRANSPORT.clone())),
            ..Default::default()
        },
        ..Default::default()
    };
    let client = SecretClient::new(
        "https://my-vault.vault.azure.net",
        MockCredential::new()?,
        Some(options),
    )?;

    let err = client.get_secret("secret-name", None).await.unwrap_err();

    // Deconstruct the HttpResponse.
    let ErrorKind::HttpResponse {
        status,
        error_code,
        raw_response,
    } = err.kind()
    else {
        panic!("expected ErrorKind::HttpResponse");
    };

    assert_eq!(status, &StatusCode::BadRequest);
    assert_eq!(error_code.as_deref(), Some("BadParameter"));
    assert!(
        matches!(raw_response, Some(r) if str::from_utf8(r.body())?.contains("Unknown parameter"))
    );

    Ok(())
}

#[tokio::test]
async fn deserialize_error_response() -> Result<(), Box<dyn std::error::Error>> {
    let options = SecretClientOptions {
        client_options: ClientOptions {
            transport: Some(Transport::new(TRANSPORT.clone())),
            ..Default::default()
        },
        ..Default::default()
    };
    let client = SecretClient::new(
        "https://my-vault.vault.azure.net",
        MockCredential::new()?,
        Some(options),
    )?;

    let err = client.get_secret("secret-name", None).await.unwrap_err();

    // Deconstruct the HttpResponse
    let ErrorKind::HttpResponse {
        status,
        error_code,
        raw_response: Some(raw_response),
    } = err.kind()
    else {
        panic!("expected ErrorKind::HttpResponse");
    };

    // Deserialize the RawResponse
    let error_response: ErrorResponse = raw_response.clone().json()?;

    assert_eq!(status, &StatusCode::BadRequest);
    assert_eq!(error_code.as_deref(), Some("BadParameter"));
    let Some(error) = error_response.error else {
        panic!("expected error");
    };
    assert_eq!(error.details.len(), 1);
    assert_eq!(error.details[0].code.as_deref(), Some("BadParameter"));
    assert_eq!(error.details[0].target.as_deref(), Some("foo"));

    Ok(())
}

static TRANSPORT: LazyLock<Arc<dyn HttpClient>> = LazyLock::new(|| {
    Arc::new(MockHttpClient::new(|_| {
        async {
            Ok(BufResponse::from_bytes(
                StatusCode::BadRequest,
                Headers::new(),
                Bytes::from_static(
                    br#"{
                    "error": {
                        "code": "BadParameter",
                        "message": "Unknown parameter",
                        "details": [
                            {
                                "code": "BadParameter",
                                "target": "foo"
                            }
                        ]
                    }
                }"#,
                ),
            ))
        }
        .boxed()
    }))
});

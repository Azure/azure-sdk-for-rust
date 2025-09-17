// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    credentials::TokenCredential,
    http::{
        headers::{Headers, RETRY_AFTER},
        BufResponse, HttpClient, Method, StatusCode, Transport,
    },
};
use azure_core_test::{credentials::MockCredential, http::MockHttpClient};
use azure_security_keyvault_certificates::{
    models::CreateCertificateParameters, CertificateClient, CertificateClientOptions,
};
use futures::FutureExt;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

// This example demonstrates using a Poller to create a certificate with the CertificateClient.
async fn test_poller() -> Result<(), Box<dyn std::error::Error>> {
    let mut options = CertificateClientOptions::default();

    // Ignore: this is only set up for testing.
    // You normally would create credentials from `azure_identity` and
    // use the default transport in production.
    let (credential, transport) = setup()?;
    options.client_options.transport = Some(Transport::new(transport));

    let client = CertificateClient::new(
        "https://my-vault.vault.azure.net",
        credential,
        Some(options),
    )?;

    // Minimal create parameters (empty policy for mock)
    let params = CreateCertificateParameters::default();

    // Start a create_certificate long-running operation.
    let operation = client
        .begin_create_certificate("my-cert", params.try_into()?, None)?
        .wait()
        .await?
        .into_body()
        .await?;
    assert_eq!(operation.status.as_deref(), Some("completed"));
    assert!(operation.target.is_some());

    Ok(())
}

// ----- BEGIN TEST SETUP -----
#[tokio::test]
async fn test_core_poller() -> Result<(), Box<dyn std::error::Error>> {
    test_poller().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test_poller().await
}

#[allow(clippy::type_complexity)]
fn setup() -> Result<(Arc<dyn TokenCredential>, Arc<dyn HttpClient>), Box<dyn std::error::Error>> {
    let credential: Arc<dyn TokenCredential> = MockCredential::new()?;
    let calls = Arc::new(AtomicUsize::new(0));
    let transport = {
        let calls = calls.clone();
        MockHttpClient::new(move |request| {
            let calls = calls.clone();
            async move {
                let idx = calls.fetch_add(1, Ordering::SeqCst);
                match idx {
                    0 => {
                        // Initial POST to start operation
                        assert_eq!(request.method(), Method::Post);
                        assert!(request.url().path().starts_with("/certificates/my-cert/create"));
                        let mut headers = Headers::new();
                        headers.insert(RETRY_AFTER, "0");
                        Ok(BufResponse::from_bytes(
                            StatusCode::Ok,
                            headers,
                            r#"{"id":"https://my-vault.vault.azure.net/certificates/my-cert/pending","status":"inProgress"}"#,
                        ))
                    }
                    1 => {
                        // Polling GET for status
                        assert_eq!(request.method(), Method::Get);
                        assert!(request.url().path().starts_with("/certificates/my-cert/pending"));
                        Ok(BufResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            r#"{"id":"https://my-vault.vault.azure.net/certificates/my-cert/pending","status":"completed","target":"https://my-vault.vault.azure.net/certificates/my-cert"}"#,
                        ))
                    }
                    _ => panic!("unexpected request count {idx}"),
                }
            }
            .boxed()
        })
    };
    Ok((credential, Arc::new(transport)))
}
// ----- END TEST SETUP -----

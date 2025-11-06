// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::Transport;
use azure_security_keyvault_certificates::{
    models::CreateCertificateParameters, CertificateClient, CertificateClientOptions,
};
use example::setup;
use futures::TryStreamExt as _;

/// This example demonstrates using a [`Poller`] to await a long-running operation (LRO) to create a certificate with the CertificateClient.
async fn example_poller() -> Result<(), Box<dyn std::error::Error>> {
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
    let certificate = client
        .create_certificate("my-cert", params.try_into()?, None)?
        .await?
        .into_model()?;
    assert_eq!(
        certificate.id,
        Some("https://my-vault.vault.azure.net/certificates/my-cert/version".into())
    );
    assert_eq!(certificate.cer, Some(b"test".to_vec()));

    Ok(())
}

/// This example demonstrates using a [`Poller`] to manually poll status for a long-running operation (LRO) to create a certificate with the CertificateClient.
///
/// If you want to manually poll status updates, you can use the `Poller` as a stream by calling [`try_next`](futures::TryStreamExt::try_next) on a mutable reference.
/// The stream will end when the operation completes, and the final status contains information about the completed operation.
async fn example_poller_stream() -> Result<(), Box<dyn std::error::Error>> {
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

    // Start a create_certificate long-running operation and manually poll status.
    let mut poller = client.create_certificate("my-cert", params.try_into()?, None)?;

    // Manually poll status updates until completion
    let mut final_status = None;
    while let Some(status) = poller.try_next().await? {
        let status = status.into_model()?;
        assert!(status.error.is_none());
        final_status = Some(status);
    }

    // The last status should indicate completion
    let status = final_status.expect("expected at least one status update");
    assert_eq!(status.status.as_deref(), Some("completed"));
    assert_eq!(
        status.target.as_deref(),
        Some("https://my-vault.vault.azure.net/certificates/my-cert")
    );

    Ok(())
}

// ----- BEGIN TEST SETUP -----
#[tokio::test]
async fn test_core_poller() -> Result<(), Box<dyn std::error::Error>> {
    example_poller().await
}

#[tokio::test]
async fn test_core_poller_stream() -> Result<(), Box<dyn std::error::Error>> {
    example_poller_stream().await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    example_poller().await?;
    example_poller_stream().await?;

    Ok(())
}

mod example {
    use azure_core::{
        credentials::TokenCredential,
        http::{
            headers::{Headers, RETRY_AFTER},
            AsyncRawResponse, HttpClient, Method, StatusCode,
        },
    };
    use azure_core_test::{credentials::MockCredential, http::MockHttpClient};
    use futures::FutureExt as _;
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    /// Setup for the await example - returns all 3 responses including the final target
    #[allow(clippy::type_complexity)]
    pub fn setup(
    ) -> Result<(Arc<dyn TokenCredential>, Arc<dyn HttpClient>), Box<dyn std::error::Error>> {
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
                        assert_eq!(request.url().path(), "/certificates/my-cert/create");
                        let mut headers = Headers::new();
                        headers.insert(RETRY_AFTER, "0");
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Ok,
                            headers,
                            r#"{"id":"https://my-vault.vault.azure.net/certificates/my-cert/pending","status":"inProgress"}"#,
                        ))
                    }
                    1 => {
                        // Polling GET for status
                        assert_eq!(request.method(), Method::Get);
                        assert_eq!(request.url().path(), "/certificates/my-cert/pending");
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            r#"{"id":"https://my-vault.vault.azure.net/certificates/my-cert/pending","status":"completed","target":"https://my-vault.vault.azure.net/certificates/my-cert"}"#,
                        ))
                    }
                    2 => {
                        // Final GET for the target
                        assert_eq!(request.method(), Method::Get);
                        assert_eq!(request.url().path(), "/certificates/my-cert");
                        Ok(AsyncRawResponse::from_bytes(
                            StatusCode::Ok,
                            Headers::new(),
                            r#"{"id":"https://my-vault.vault.azure.net/certificates/my-cert/version","cer":"dGVzdA=="}"#,
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
}
// ----- END TEST SETUP -----

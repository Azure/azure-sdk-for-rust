// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use async_trait::async_trait;
use azure_core::{
    http::{fake::ResponseExt as _, headers::Headers, AsyncRawResponse, StatusCode},
    Result,
};
use azure_security_keyvault_certificates::{
    clients::fake::CertificateServer, models::CertificateClientGetCertificateOptions,
    CertificateClient,
};
use std::sync::Arc;

struct FakeCertificates;

#[async_trait]
impl CertificateServer for FakeCertificates {
    async fn get_certificate(
        &self,
        certificate_name: &str,
        _options: Option<CertificateClientGetCertificateOptions<'_>>,
    ) -> Result<AsyncRawResponse> {
        match certificate_name {
            "my-certificate" => Ok(AsyncRawResponse::fake_with_bytes(
                StatusCode::Ok,
                Headers::new(),
                r#"{"id":"https://example.vault.azure.net/certificates/my-certificate"}"#,
            )),
            // Return an HTTP response so the client applies its normal service-error handling.
            _ => AsyncRawResponse::fake_with_error(
                StatusCode::NotFound,
                Headers::new(),
                "CertificateNotFound",
                "The requested certificate was not found.",
            ),
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let client = CertificateClient::with_fake(
        "https://example.vault.azure.net",
        Arc::new(FakeCertificates),
        None,
    )?;

    let certificate = client
        .get_certificate("my-certificate", None)
        .await?
        .into_model()?;
    assert_eq!(
        certificate.id.as_deref(),
        Some("https://example.vault.azure.net/certificates/my-certificate"),
    );
    println!("Certificate ID: {:?}", certificate.id);

    let error = client
        .get_certificate("missing", None)
        .await
        .expect_err("the fake returns a 404 for an unknown certificate");
    assert_eq!(error.http_status(), Some(StatusCode::NotFound));
    println!("Expected error: {error}");
    Ok(())
}

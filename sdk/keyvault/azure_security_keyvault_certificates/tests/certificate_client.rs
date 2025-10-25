// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg_attr(target_arch = "wasm32", allow(unused_imports))]

use azure_core::{http::StatusCode, Result};
use azure_core_test::{recorded, ErrorKind, TestContext, TestMode, SANITIZE_BODY_NAME};
use azure_security_keyvault_certificates::{
    models::{
        CertificateClientUpdateCertificatePropertiesOptions, CertificatePolicy,
        CreateCertificateParameters, CurveName, IssuerParameters, KeyProperties, KeyType,
        UpdateCertificatePropertiesParameters, X509CertificateProperties,
    },
    CertificateClient, CertificateClientOptions, ResourceExt as _,
};
use azure_security_keyvault_keys::{
    models::{SignParameters, SignatureAlgorithm},
    KeyClient, KeyClientOptions,
};
use azure_security_keyvault_test::Retry;
use futures::TryStreamExt;
use openssl::sha::sha256;
use std::{collections::HashMap, sync::LazyLock};

static DEFAULT_CERTIFICATE_POLICY: LazyLock<CertificatePolicy> =
    LazyLock::new(|| CertificatePolicy {
        x509_certificate_properties: Some(X509CertificateProperties {
            subject: Some("CN=DefaultPolicy".into()),
            ..Default::default()
        }),
        issuer_parameters: Some(IssuerParameters {
            name: Some("Self".into()),
            ..Default::default()
        }),
        ..Default::default()
    });

#[recorded::test]
async fn certificate_roundtrip(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    recording.remove_sanitizers(&[SANITIZE_BODY_NAME]).await?;

    let mut options = CertificateClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = CertificateClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create a self-signed certificate.
    let body = CreateCertificateParameters {
        certificate_policy: Some(DEFAULT_CERTIFICATE_POLICY.clone()),
        ..Default::default()
    };
    let certificate = client
        .create_certificate("certificate-roundtrip", body.try_into()?, None)?
        .await?
        .into_body()?;

    // Get the latest version of the certificate we just created.
    let version = certificate.resource_id()?.version;

    assert!(certificate.id.is_some());
    assert!(version.is_some());

    Ok(())
}

#[recorded::test]
async fn update_certificate_properties(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    recording.remove_sanitizers(&[SANITIZE_BODY_NAME]).await?;

    let mut options = CertificateClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = CertificateClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create a self-signed certificate.
    let body = CreateCertificateParameters {
        certificate_policy: Some(DEFAULT_CERTIFICATE_POLICY.clone()),
        ..Default::default()
    };
    let certificate = client
        .create_certificate("update-properties", body.try_into()?, None)?
        .await?
        .into_body()?;

    // Get the latest version of the certificate we just created.
    let certificate_version = certificate.resource_id()?.version;

    // Update certificate properties.
    let parameters = UpdateCertificatePropertiesParameters {
        certificate_attributes: certificate.attributes,
        tags: Some(HashMap::from_iter(vec![(
            "test-name".into(),
            "update_certificate_properties".into(),
        )])),
        ..Default::default()
    };
    let certificate = client
        .update_certificate_properties(
            "update-properties",
            parameters.try_into()?,
            Some(CertificateClientUpdateCertificatePropertiesOptions {
                certificate_version,
                ..Default::default()
            }),
        )
        .await?
        .into_body()?;

    assert_eq!(
        certificate.tags.expect("expected tags").get("test-name"),
        Some(&String::from("update_certificate_properties"))
    );

    Ok(())
}

#[recorded::test]
async fn list_certificates(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    recording.remove_sanitizers(&[SANITIZE_BODY_NAME]).await?;

    let mut options = CertificateClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = CertificateClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create several self-signed certificates.
    let mut names = vec!["list-certificates-1", "list-certificates-2"];
    let body = CreateCertificateParameters {
        certificate_policy: Some(DEFAULT_CERTIFICATE_POLICY.clone()),
        ..Default::default()
    };
    client
        .create_certificate("list-certificates-1", body.clone().try_into()?, None)?
        .await?;
    client
        .create_certificate("list-certificates-2", body.try_into()?, None)?
        .await?;

    // List certificates.
    let mut pager = client.list_certificate_properties(None)?.into_stream();
    while let Some(certificate) = pager.try_next().await? {
        // Get the certificate name from the ID.
        let name = certificate.resource_id()?.name;
        if let Some(idx) = names.iter().position(|n| name.eq(*n)) {
            names.remove(idx);
        }
    }
    assert!(names.is_empty());

    Ok(())
}

#[recorded::test]
async fn purge_certificate(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    recording.remove_sanitizers(&[SANITIZE_BODY_NAME]).await?;

    let mut options = CertificateClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = CertificateClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create a self-signed certificate.
    let body = CreateCertificateParameters {
        certificate_policy: Some(DEFAULT_CERTIFICATE_POLICY.clone()),
        ..Default::default()
    };
    const NAME: &str = "purge-certificate";
    client
        .create_certificate(NAME, body.try_into()?, None)?
        .await?;

    // Delete the certificate.
    client.delete_certificate(NAME.as_ref(), None).await?;

    // Because deletes may not happen right away, try purging in a loop.
    let mut retry = match recording.test_mode() {
        TestMode::Playback => Retry::immediate(),
        _ => Retry::progressive(None),
    };

    loop {
        match client.purge_deleted_certificate(NAME.as_ref(), None).await {
            Ok(_) => {
                println!("{NAME} has been purged");
                break;
            }
            Err(err) if matches!(err.http_status(), Some(StatusCode::Conflict)) => {
                println!(
                    "Retrying in {} seconds",
                    retry.duration().unwrap_or_default().as_secs_f32()
                );
                if retry.next().await.is_none() {
                    return Err(err);
                }
            }
            Err(err) => return Err(err),
        }
    }

    Ok(())
}

#[recorded::test]
async fn sign_jwt_with_ec_certificate(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    recording.remove_sanitizers(&[SANITIZE_BODY_NAME]).await?;

    let mut options = CertificateClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = CertificateClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create an EC certificate policy for signing.
    let policy = CertificatePolicy {
        x509_certificate_properties: Some(X509CertificateProperties {
            subject: Some("CN=DefaultPolicy".into()),
            ..Default::default()
        }),
        issuer_parameters: Some(IssuerParameters {
            name: Some("Self".into()),
            ..Default::default()
        }),
        key_properties: Some(KeyProperties {
            key_type: Some(KeyType::Ec),
            curve: Some(CurveName::P256),
            ..Default::default()
        }),
        ..Default::default()
    };

    // Create a self-signed certificate.
    let body = CreateCertificateParameters {
        certificate_policy: Some(policy),
        ..Default::default()
    };
    const NAME: &str = "ec-certificate-signer";
    client
        .create_certificate(NAME, body.try_into()?, None)?
        .await?;

    let mut key_options = KeyClientOptions::default();
    recording.instrument(&mut key_options.client_options);

    // Sign a JWT.
    let key_client = KeyClient::new(
        client.endpoint().as_str(),
        recording.credential(),
        Some(key_options),
    )?;

    // cspell:disable
    const JWT: &[u8] =
        b"eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJoZWF0aHMiLCJuYW1lIjoiSGVhdGggU3Rld2FydCIsImlhdCI6MTc0MzgzMzY5MX0";
    let digest = sha256(JWT).to_vec();

    let body = SignParameters {
        algorithm: Some(SignatureAlgorithm::Es256),
        value: Some(digest),
    };
    let signature = key_client
        .sign(NAME, body.try_into()?, None)
        .await?
        .into_body()?;
    assert!(signature.result.is_some());
    // example: 6AIg-utePBdmCU-uGvpjh4uKb3UV0yvdWKNLSp-EivC4oavdqpfxmfMB9GsR6dBMM1Ekp8ZBrzUMaCvShXWyog
    // cspell:enable

    Ok(())
}

#[recorded::test]
async fn create_invalid_certificate(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    recording.remove_sanitizers(&[SANITIZE_BODY_NAME]).await?;

    let mut options = CertificateClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = CertificateClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    let body = CreateCertificateParameters {
        certificate_policy: Some(DEFAULT_CERTIFICATE_POLICY.clone()),
        ..Default::default()
    };
    let err = client
        .create_certificate("create_invalid_certificate", body.try_into()?, None)?
        .await
        .expect_err("expected HTTP error");

    assert!(matches!(
        err.kind(),
        ErrorKind::HttpResponse { status, .. } if *status == StatusCode::BadRequest
    ));

    Ok(())
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg_attr(target_arch = "wasm32", allow(unused_imports))]

use azure_core::{base64, http::StatusCode, sleep::sleep, Result};
use azure_core_test::{recorded, TestContext, TestMode, SANITIZE_BODY_NAME};
use azure_security_keyvault_certificates::{
    models::{
        Action, CertificateCreateParameters, CertificatePolicy, CertificatePolicyAction,
        CertificateUpdateParameters, IssuerParameters, JsonWebKeyCurveName, JsonWebKeyType,
        KeyProperties, LifetimeAction, SecretProperties, Trigger, X509CertificateProperties,
    },
    CertificateClient, CertificateClientOptions, ResourceExt as _, ResourceId,
};
use azure_security_keyvault_keys::{
    models::{JsonWebKeySignatureAlgorithm, KeySignParameters},
    KeyClient, KeyClientOptions,
};
use azure_security_keyvault_test::Retry;
use futures::TryStreamExt;
use std::{collections::HashMap, sync::LazyLock, time::Duration};

static DEFAULT_POLICY: LazyLock<CertificatePolicy> = LazyLock::new(|| CertificatePolicy {
    x509_certificate_properties: Some(X509CertificateProperties {
        subject: Some("CN=Azure/azure-sdk-for-rust".into()),
        ..Default::default()
    }),
    issuer_parameters: Some(IssuerParameters {
        name: Some("Self".into()),
        ..Default::default()
    }),
    lifetime_actions: vec![LifetimeAction {
        action: Some(Action {
            action_type: Some(CertificatePolicyAction::AutoRenew),
        }),
        trigger: Some(Trigger {
            days_before_expiry: Some(90),
            ..Default::default()
        }),
    }],
    key_properties: Some(KeyProperties {
        key_type: Some(JsonWebKeyType::RSA),
        key_size: Some(2048),
        ..Default::default()
    }),
    secret_properties: Some(SecretProperties {
        content_type: Some("application/x-pem-file".into()),
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
    let body = CertificateCreateParameters {
        certificate_policy: Some(DEFAULT_POLICY.clone()),
        ..Default::default()
    };
    let _operation = client
        .create_certificate("certificate-roundtrip", body.try_into()?, None)
        .await?
        .into_body()
        .await?;

    // TODO: Actually wait for the certificate operation to complete.
    if recording.test_mode() != TestMode::Playback {
        sleep(Duration::from_secs(3)).await;
    }

    // Get the latest version of the certificate we just created.
    let certificate = client
        .get_certificate("certificate-roundtrip", "", None)
        .await?
        .into_body()
        .await?;
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
    let body = CertificateCreateParameters {
        certificate_policy: Some(DEFAULT_POLICY.clone()),
        ..Default::default()
    };
    let _operation = client
        .create_certificate("update-properties", body.try_into()?, None)
        .await?
        .into_body()
        .await?;

    // TODO: Actually wait for the certificate operation to complete.
    if recording.test_mode() != TestMode::Playback {
        sleep(Duration::from_secs(3)).await;
    }

    // Get the latest version of the certificate we just created.
    let certificate = client
        .get_certificate("update-properties", "", None)
        .await?
        .into_body()
        .await?;
    let version = certificate.resource_id()?.version;

    // Update certificate properties.
    let parameters = CertificateUpdateParameters {
        certificate_attributes: certificate.attributes,
        tags: HashMap::from_iter(vec![(
            "test-name".into(),
            "update_certificate_properties".into(),
        )]),
        ..Default::default()
    };
    let certificate = client
        .update_certificate(
            "update-properties",
            version.as_deref().unwrap_or(""),
            parameters.try_into()?,
            None,
        )
        .await?
        .into_body()
        .await?;

    assert_eq!(
        certificate.tags.get("test-name"),
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
    let body = CertificateCreateParameters {
        certificate_policy: Some(DEFAULT_POLICY.clone()),
        ..Default::default()
    };
    let _operation = client
        .create_certificate("list-certificates-1", body.clone().try_into()?, None)
        .await?
        .into_body()
        .await?;

    let _operation = client
        .create_certificate("list-certificates-2", body.try_into()?, None)
        .await?
        .into_body()
        .await?;

    // TODO: Actually wait for the certificate operation to complete.
    if recording.test_mode() != TestMode::Playback {
        sleep(Duration::from_secs(3)).await;
    }

    // List certificates.
    let mut pager = client.list_certificates(None)?.into_stream();
    while let Some(certificates) = pager.try_next().await? {
        let certificates = certificates.into_body().await?.value;
        for certificate in certificates {
            // Get the certificate name from the ID.
            let name = certificate.resource_id()?.name;
            if let Some(idx) = names.iter().position(|n| name.eq(*n)) {
                names.remove(idx);
            }
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
    let body = CertificateCreateParameters {
        certificate_policy: Some(DEFAULT_POLICY.clone()),
        ..Default::default()
    };
    let operation = client
        .create_certificate("purge-certificate", body.try_into()?, None)
        .await?
        .into_body()
        .await?;

    // TODO: Actually wait for the certificate operation to complete.
    if recording.test_mode() != TestMode::Playback {
        sleep(Duration::from_secs(3)).await;
    }

    // Delete the certificate.
    let name = operation.resource_id()?.name;
    client.delete_certificate(name.as_ref(), None).await?;

    // Because deletes may not happen right away, try purging in a loop.
    let mut retry = match recording.test_mode() {
        TestMode::Playback => Retry::immediate(),
        _ => Retry::progressive(None),
    };

    loop {
        match client.purge_deleted_certificate(name.as_ref(), None).await {
            Ok(_) => {
                println!("{name} has been purged");
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

    // Create an EC certificate policy for key encipherment.
    let policy = CertificatePolicy {
        x509_certificate_properties: Some(X509CertificateProperties {
            subject: Some("CN=Azure/azure-sdk-for-rust".into()),
            ..Default::default()
        }),
        issuer_parameters: Some(IssuerParameters {
            name: Some("Self".into()),
            ..Default::default()
        }),
        lifetime_actions: vec![LifetimeAction {
            action: Some(Action {
                action_type: Some(CertificatePolicyAction::AutoRenew),
            }),
            trigger: Some(Trigger {
                days_before_expiry: Some(90),
                ..Default::default()
            }),
        }],
        key_properties: Some(KeyProperties {
            key_type: Some(JsonWebKeyType::EC),
            curve: Some(JsonWebKeyCurveName::P256),
            ..Default::default()
        }),
        secret_properties: Some(SecretProperties {
            content_type: Some("application/x-pem-file".into()),
        }),
        ..Default::default()
    };

    // Create a self-signed certificate.
    let body = CertificateCreateParameters {
        certificate_policy: Some(policy),
        ..Default::default()
    };
    let operation = client
        .create_certificate("ec-certificate-signer", body.try_into()?, None)
        .await?
        .into_body()
        .await?;

    // TODO: Actually wait for the certificate operation to complete.
    if recording.test_mode() != TestMode::Playback {
        sleep(Duration::from_secs(3)).await;
    }

    let mut key_options = KeyClientOptions::default();
    recording.instrument(&mut key_options.client_options);

    // Sign a JWT.
    let ResourceId {
        vault_url, name, ..
    } = operation.resource_id()?;
    let key_client = KeyClient::new(&vault_url, recording.credential(), Some(key_options))?;

    // cspell:disable
    const _JWT: &str =
        "eyJhbGciOiJFUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJoZWF0aHMiLCJuYW1lIjoiSGVhdGggU3Rld2FydCIsImlhdCI6MTc0MzgzMzY5MX0";
    const DIGEST: &str = "GDOTWbe5x6KXgoykVcqygzMOAsjXcYUoZdzAkJR5a7Y";

    let body = KeySignParameters {
        algorithm: Some(JsonWebKeySignatureAlgorithm::ES256),
        value: Some(base64::decode_url_safe(DIGEST)?),
    };
    let signature = key_client
        .sign(&name, "", body.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert!(signature.result.is_some());
    // example: 6AIg-utePBdmCU-uGvpjh4uKb3UV0yvdWKNLSp-EivC4oavdqpfxmfMB9GsR6dBMM1Ekp8ZBrzUMaCvShXWyog
    // cspell:enable

    Ok(())
}

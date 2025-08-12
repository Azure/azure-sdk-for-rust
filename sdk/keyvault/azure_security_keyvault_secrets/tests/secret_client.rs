// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg_attr(target_arch = "wasm32", allow(unused_imports))]

use azure_core::{
    http::{InstrumentationOptions, StatusCode},
    Result,
};
use azure_core_test::{
    recorded,
    tracing::{InstrumentationInformation, InstrumentedApiInformation},
    TestContext, TestMode,
};
use azure_security_keyvault_secrets::{
    models::{SetSecretParameters, UpdateSecretPropertiesParameters},
    ResourceExt as _, SecretClient, SecretClientOptions,
};
use azure_security_keyvault_test::Retry;
use futures::TryStreamExt;
use std::collections::HashMap;

#[recorded::test]
async fn secret_roundtrip(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = SecretClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = SecretClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Set a secret.
    let body = SetSecretParameters {
        value: Some("secret-value".into()),
        ..Default::default()
    };
    let secret = client
        .set_secret("secret-roundtrip", body.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert_eq!(secret.value, Some("secret-value".into()));

    // Get a specific version of a secret.
    let version = secret.resource_id()?.version.unwrap_or_default();
    let secret = client
        .get_secret("secret-roundtrip", version.as_ref(), None)
        .await?
        .into_body()
        .await?;
    assert_eq!(secret.value, Some("secret-value".into()));

    Ok(())
}

#[recorded::test]
async fn update_secret_properties(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = SecretClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = SecretClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Set a secret.
    let body = SetSecretParameters {
        value: Some("secret-value".into()),
        ..Default::default()
    };
    let secret = client
        .set_secret("update-secret", body.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert_eq!(secret.value, Some("secret-value".into()));

    // Update secret properties.
    let properties = UpdateSecretPropertiesParameters {
        content_type: Some("text/plain".into()),
        secret_attributes: secret.attributes,
        tags: Some(HashMap::from_iter(vec![(
            "test-name".into(),
            "update_secret_properties".into(),
        )])),
    };
    let secret = client
        .update_secret_properties("update-secret", "", properties.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert_eq!(secret.content_type, Some("text/plain".into()));
    assert_eq!(
        secret.tags.expect("expected tags").get("test-name"),
        Some(&String::from("update_secret_properties"))
    );

    Ok(())
}

#[recorded::test]
async fn list_secrets(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = SecretClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = SecretClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create several secrets.
    let mut names = vec!["list-secrets-1", "list-secrets-2"];
    let secret1 = client
        .set_secret(names[0], r#"{"value":"secret-value-1"}"#.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert_eq!(secret1.value, Some("secret-value-1".into()));

    let secret2 = client
        .set_secret(names[1], r#"{"value":"secret-value-2"}"#.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert_eq!(secret2.value, Some("secret-value-2".into()));

    // List secrets.
    let mut pager = client.list_secret_properties(None)?.into_stream();
    while let Some(secret) = pager.try_next().await? {
        // Get the secret name from the ID.
        let name = secret.resource_id()?.name;
        if let Some(idx) = names.iter().position(|n| name.eq(*n)) {
            names.remove(idx);
        }
    }
    assert!(names.is_empty());

    Ok(())
}

#[recorded::test]
async fn purge_secret(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = SecretClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = SecretClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create a secret.
    let secret = client
        .set_secret(
            "purge-secret",
            SetSecretParameters {
                value: Some("secret-value".into()),
                ..Default::default()
            }
            .try_into()?,
            None,
        )
        .await?
        .into_body()
        .await?;

    // Delete the secret.
    let name = secret.resource_id()?.name;
    client.delete_secret(name.as_ref(), None).await?;

    // Because deletes may not happen right away, try purging in a loop.
    let mut retry = match recording.test_mode() {
        TestMode::Playback => Retry::immediate(),
        _ => Retry::progressive(None),
    };

    loop {
        match client.purge_deleted_secret(name.as_ref(), None).await {
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
async fn round_trip_secret_verify_telemetry(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    // Verify that the distributed tracing traces generated from the API call below match the expected traces.
    azure_core_test::tracing::assert_instrumentation_information(
        |tracer_provider| {
            let mut options = SecretClientOptions::default();
            recording.instrument(&mut options.client_options);
            options.client_options.instrumentation = Some(InstrumentationOptions {
                tracer_provider: Some(tracer_provider),
            });
            SecretClient::new(
                recording.var("AZURE_KEYVAULT_URL", None).as_str(),
                recording.credential(),
                Some(options),
            )
        },
        |client: SecretClient| {
            Box::pin(async move {
                // Set a secret.
                let body = SetSecretParameters {
                    value: Some("secret-value-instrument".into()),
                    ..Default::default()
                };
                let secret = client
                    .set_secret("secret-roundtrip-instrument", body.try_into()?, None)
                    .await?
                    .into_body()
                    .await?;
                assert_eq!(secret.value, Some("secret-value-instrument".into()));

                // Get a specific version of a secret.
                let version = secret.resource_id()?.version.unwrap_or_default();
                let secret = client
                    .get_secret("secret-roundtrip-instrument", version.as_ref(), None)
                    .await?
                    .into_body()
                    .await?;
                assert_eq!(secret.value, Some("secret-value-instrument".into()));
                Ok(())
            })
        },
        InstrumentationInformation {
            package_name: recording.var("CARGO_PKG_NAME", None),
            package_version: recording.var("CARGO_PKG_VERSION", None),
            package_namespace: Some("azure_security_keyvault_secrets"),
            api_calls: vec![
                InstrumentedApiInformation {
                    api_name: Some("KeyVault.setSecret"),
                    api_verb: azure_core::http::Method::Put,
                    ..Default::default()
                },
                InstrumentedApiInformation {
                    api_name: Some("KeyVault.getSecret"),
                    ..Default::default()
                },
            ],
        },
    )
    .await?;

    Ok(())
}

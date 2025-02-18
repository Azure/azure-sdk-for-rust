// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg_attr(target_arch = "wasm32", allow(unused_imports))]

use azure_core::{Result, StatusCode};
use azure_core_test::{recorded, TestContext, TestMode};
use azure_security_keyvault_secrets::{
    models::{SecretSetParameters, SecretUpdateParameters},
    ResourceExt as _, SecretClient, SecretClientOptions,
};
use azure_security_keyvault_test::Retry;
use futures::TryStreamExt;
use std::collections::HashMap;

const REMOVE_SANITIZERS: &[&str] = &[
    // BodyKeySanitizer("$..id"): the resource ID contains the required name and version.
    "AZSDK3430",
];

#[recorded::test]
async fn secret_roundtrip(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    recording.remove_sanitizers(REMOVE_SANITIZERS).await?;

    let mut options = SecretClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = SecretClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Set a secret.
    let body = SecretSetParameters {
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
    recording.remove_sanitizers(REMOVE_SANITIZERS).await?;

    let mut options = SecretClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = SecretClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Set a secret.
    let body = SecretSetParameters {
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
    let properties = SecretUpdateParameters {
        content_type: Some("text/plain".into()),
        secret_attributes: secret.attributes,
        tags: Some(HashMap::from_iter(vec![(
            "test-name".into(),
            "update_secret_properties".into(),
        )])),
    };
    let secret = client
        .update_secret("update-secret", "", properties.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert_eq!(secret.content_type, Some("text/plain".into()));
    assert_eq!(
        secret.tags.as_ref().and_then(|tags| tags.get("test-name")),
        Some(&String::from("update_secret_properties"))
    );

    Ok(())
}

#[recorded::test]
async fn list_secrets(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    recording.remove_sanitizers(REMOVE_SANITIZERS).await?;

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
    let mut pager = client.get_secrets(None)?.into_stream();
    while let Some(secrets) = pager.try_next().await? {
        let Some(secrets) = secrets.into_body().await?.value else {
            continue;
        };

        for secret in secrets {
            // Get the secret name from the ID.
            let name = secret.resource_id()?.name;
            if let Some(idx) = names.iter().position(|n| name.eq(*n)) {
                names.remove(idx);
            }
        }
    }
    assert!(names.is_empty());

    Ok(())
}

#[recorded::test]
async fn purge_secret(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    recording.remove_sanitizers(REMOVE_SANITIZERS).await?;

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
            SecretSetParameters {
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

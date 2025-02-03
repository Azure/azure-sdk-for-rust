// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg_attr(target_arch = "wasm32", allow(unused_imports))]

use azure_core::Result;
use azure_core_test::{recorded, TestContext};
use azure_security_keyvault_secrets::{
    models::{SecretSetParameters, SecretUpdateParameters},
    ResourceExt as _, SecretClient, SecretClientOptions,
};
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
        // TODO: https://github.com/Azure/typespec-rust/issues/223
        .set_secret("secret-roundtrip".into(), body.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert_eq!(secret.value, Some("secret-value".into()));

    // Get a specific version of a secret.
    let version = secret.resource_id()?.version.unwrap_or_default();
    let secret = client
        // TODO: https://github.com/Azure/typespec-rust/issues/223
        .get_secret("secret-roundtrip".into(), version, None)
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
        // TODO: https://github.com/Azure/typespec-rust/issues/223
        .set_secret("update-secret".into(), body.try_into()?, None)
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
        .update_secret(
            // TODO: https://github.com/Azure/typespec-rust/issues/223
            "update-secret".into(),
            "".into(),
            properties.try_into()?,
            None,
        )
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
        .set_secret(
            names[0].into(),
            r#"{"value":"secret-value-1"}"#.try_into()?,
            None,
        )
        .await?
        .into_body()
        .await?;
    assert_eq!(secret1.value, Some("secret-value-1".into()));

    let secret2 = client
        .set_secret(
            names[1].into(),
            r#"{"value":"secret-value-2"}"#.try_into()?,
            None,
        )
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

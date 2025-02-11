// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg_attr(target_arch = "wasm32", allow(unused_imports))]

use azure_core::{Result, StatusCode};
use azure_core_test::{recorded, TestContext, TestMode};
use azure_security_keyvault_keys::{
    models::{JsonWebKeyCurveName, JsonWebKeyType, KeyCreateParameters, KeyUpdateParameters},
    KeyClient, KeyClientOptions, ResourceExt as _,
};
use azure_security_keyvault_test::Retry;
use futures::TryStreamExt;
use std::collections::HashMap;

const REMOVE_SANITIZERS: &[&str] = &[
    // BodyKeySanitizer("$..id"): the resource ID contains the required name and version.
    "AZSDK3430",
];

#[recorded::test]
async fn key_roundtrip(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    recording.remove_sanitizers(REMOVE_SANITIZERS).await?;

    let mut options = KeyClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = KeyClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create an RSA key.
    let body = KeyCreateParameters {
        kty: Some(JsonWebKeyType::RSA),
        key_size: Some(2048),
        ..Default::default()
    };
    let key = client
        .create_key("key-roundtrip", body.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert!(matches!(key.key, Some(ref jwk) if jwk.e == Some(vec![1, 0, 1])));

    // Get a specific version of a key.
    let version = key.resource_id()?.version.unwrap_or_default();
    let key = client
        .get_key("key-roundtrip", version.as_ref(), None)
        .await?
        .into_body()
        .await?;
    assert!(matches!(key.key, Some(ref jwk) if jwk.e == Some(vec![1, 0, 1])));

    Ok(())
}

#[recorded::test]
async fn update_key_properties(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    recording.remove_sanitizers(REMOVE_SANITIZERS).await?;

    let mut options = KeyClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = KeyClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create an EC key.
    let body = KeyCreateParameters {
        kty: Some(JsonWebKeyType::EC),
        curve: Some(JsonWebKeyCurveName::P256),
        ..Default::default()
    };
    let key = client
        .create_key("update-key", body.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert!(matches!(key.key, Some(ref jwk) if jwk.x.is_some()));

    // Update key properties.
    let properties = KeyUpdateParameters {
        key_attributes: key.attributes,
        tags: Some(HashMap::from_iter(vec![(
            "test-name".into(),
            "update_key_properties".into(),
        )])),
        ..Default::default()
    };
    let key = client
        .update_key("update-key", "", properties.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert_eq!(
        key.tags.as_ref().and_then(|tags| tags.get("test-name")),
        Some(&String::from("update_key_properties"))
    );

    Ok(())
}

#[recorded::test]
async fn list_keys(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    recording.remove_sanitizers(REMOVE_SANITIZERS).await?;

    let mut options = KeyClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = KeyClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create several EC keys.
    let mut names = vec!["list-keys-1", "list-keys-2"];
    let secret1 = client
        .create_key(
            names[0],
            r#"{"kty":"EC","curve":"P-384"}"#.try_into()?,
            None,
        )
        .await?
        .into_body()
        .await?;
    assert!(matches!(secret1.key, Some(ref jwk) if jwk.x.is_some()));

    let secret2 = client
        .create_key(
            names[1],
            r#"{"kty":"EC","curve":"P-384"}"#.try_into()?,
            None,
        )
        .await?
        .into_body()
        .await?;
    assert!(matches!(secret2.key, Some(ref jwk) if jwk.x.is_some()));

    // List keys.
    let mut pager = client.get_keys(None)?.into_stream();
    while let Some(keys) = pager.try_next().await? {
        let Some(keys) = keys.into_body().await?.value else {
            continue;
        };

        for key in keys {
            // Get the key name from the ID.
            let name = key.resource_id()?.name;
            if let Some(idx) = names.iter().position(|n| name.eq(*n)) {
                names.remove(idx);
            }
        }
    }
    assert!(names.is_empty());

    Ok(())
}

#[recorded::test]
async fn purge_key(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();
    recording.remove_sanitizers(REMOVE_SANITIZERS).await?;

    let mut options = KeyClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = KeyClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create an RSA key.
    let body = KeyCreateParameters {
        kty: Some(JsonWebKeyType::RSA),
        key_size: Some(2048),
        ..Default::default()
    };
    let key = client
        .create_key("purge-key", body.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert!(matches!(key.key, Some(ref jwk) if jwk.e == Some(vec![1, 0, 1])));

    // Delete the key.
    let name = key.resource_id()?.name;
    client.delete_key(name.as_ref(), None).await?;

    // Because deletes may not happen right away, try purging in a loop.
    let mut retry = match recording.test_mode() {
        TestMode::Playback => Retry::immediate(),
        _ => Retry::progressive(None),
    };

    loop {
        match client.purge_deleted_key(name.as_ref(), None).await {
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

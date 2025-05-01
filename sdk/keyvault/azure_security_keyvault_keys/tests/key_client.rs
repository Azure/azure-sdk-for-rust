// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

#![cfg_attr(target_arch = "wasm32", allow(unused_imports))]

use azure_core::{http::StatusCode, Result};
use azure_core_test::{recorded, TestContext, TestMode};
use azure_security_keyvault_keys::{
    models::{
        CreateKeyParameters, CurveName, EncryptionAlgorithm, KeyOperationParameters, KeyType,
        SignParameters, SignatureAlgorithm, UpdateKeyPropertiesParameters, VerifyParameters,
    },
    KeyClient, KeyClientOptions, ResourceExt as _,
};
use azure_security_keyvault_test::Retry;
use futures::TryStreamExt;
use std::collections::HashMap;

#[recorded::test]
async fn key_roundtrip(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = KeyClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = KeyClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create an RSA key.
    let body = CreateKeyParameters {
        kty: Some(KeyType::RSA),
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

    let mut options = KeyClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = KeyClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create an EC key.
    let body = CreateKeyParameters {
        kty: Some(KeyType::EC),
        curve: Some(CurveName::P256),
        ..Default::default()
    };
    let key = client
        .create_key("update-key", body.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert!(matches!(key.key, Some(ref jwk) if jwk.x.is_some()));

    // Update key properties.
    let properties = UpdateKeyPropertiesParameters {
        key_attributes: key.attributes,
        tags: Some(HashMap::from_iter(vec![(
            "test-name".into(),
            "update_key_properties".into(),
        )])),
        ..Default::default()
    };
    let key = client
        .update_key_properties("update-key", "", properties.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert_eq!(
        key.tags.expect("expected tags").get("test-name"),
        Some(&String::from("update_key_properties"))
    );

    Ok(())
}

#[recorded::test]
async fn list_keys(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

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
    let mut pager = client.list_key_properties(None)?.into_stream();
    while let Some(keys) = pager.try_next().await? {
        let keys = keys.into_body().await?.value;
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

    let mut options = KeyClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = KeyClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create an RSA key.
    let body = CreateKeyParameters {
        kty: Some(KeyType::RSA),
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

#[recorded::test]
async fn encrypt_decrypt(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = KeyClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = KeyClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create an RSA key.
    let body = CreateKeyParameters {
        kty: Some(KeyType::RSA),
        key_size: Some(2048),
        ..Default::default()
    };

    const NAME: &str = "encrypt-decrypt";

    let key = client
        .create_key(NAME, body.try_into()?, None)
        .await?
        .into_body()
        .await?;
    let version = key.resource_id()?.version.unwrap_or_default();

    // Encrypt plaintext.
    let plaintext = b"plaintext".to_vec();
    let mut parameters = KeyOperationParameters {
        algorithm: Some(EncryptionAlgorithm::RsaOAEP256),
        value: Some(plaintext.clone()),
        ..Default::default()
    };
    let encrypted = client
        .encrypt(NAME, version.as_ref(), parameters.clone().try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert!(matches!(encrypted.result.as_ref(), Some(ciphertext) if !ciphertext.is_empty()));

    // Decrypt ciphertext.
    parameters.value = encrypted.result;
    let decrypted = client
        .decrypt(NAME, version.as_ref(), parameters.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert!(matches!(decrypted.result, Some(result) if result.eq(&plaintext)));

    Ok(())
}

#[recorded::test]
async fn sign_verify(ctx: TestContext) -> Result<()> {
    use sha2::{Digest as _, Sha256};

    let recording = ctx.recording();

    let mut options = KeyClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = KeyClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create an EC key.
    let body = CreateKeyParameters {
        kty: Some(KeyType::EC),
        curve: Some(CurveName::P256),
        ..Default::default()
    };

    const NAME: &str = "sign-verify";
    const ALG: Option<SignatureAlgorithm> = Some(SignatureAlgorithm::ES256);

    let key = client
        .create_key(NAME, body.try_into()?, None)
        .await?
        .into_body()
        .await?;
    let version = key.resource_id()?.version.unwrap_or_default();

    // Hash and sign plaintext.
    let plaintext = b"plaintext".to_vec();
    let digest = Sha256::digest(plaintext).to_vec();

    let parameters = SignParameters {
        algorithm: ALG,
        value: Some(digest.clone()),
    };
    let signed = client
        .sign(NAME, version.as_ref(), parameters.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert!(matches!(signed.result.as_ref(), Some(signature) if !signature.is_empty()));

    // Verify signature.
    let parameters = VerifyParameters {
        algorithm: ALG,
        digest: Some(digest),
        signature: signed.result,
    };
    let verified = client
        .verify(NAME, version.as_ref(), parameters.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert_eq!(verified.value, Some(true));

    Ok(())
}

#[recorded::test]
async fn wrap_key_unwrap_key(ctx: TestContext) -> Result<()> {
    let recording = ctx.recording();

    let mut options = KeyClientOptions::default();
    recording.instrument(&mut options.client_options);

    let client = KeyClient::new(
        recording.var("AZURE_KEYVAULT_URL", None).as_str(),
        recording.credential(),
        Some(options),
    )?;

    // Create a KEK using RSA.
    let body = CreateKeyParameters {
        kty: Some(KeyType::RSA),
        key_size: Some(2048),
        ..Default::default()
    };

    const NAME: &str = "wrap-key-unwrap-key";
    const ALG: Option<EncryptionAlgorithm> = Some(EncryptionAlgorithm::RsaOAEP256);

    let key = client
        .create_key(NAME, body.try_into()?, None)
        .await?
        .into_body()
        .await?;
    let version = key.resource_id()?.version.unwrap_or_default();

    // Generate a data encryption key.
    let dek = recording.random::<[u8; 32]>().to_vec();

    // Wrap the DEK.
    let mut parameters = KeyOperationParameters {
        algorithm: ALG,
        value: Some(dek.clone()),
        ..Default::default()
    };
    let wrapped = client
        .wrap_key(NAME, version.as_ref(), parameters.clone().try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert!(matches!(wrapped.result.as_ref(), Some(result) if !result.is_empty()));

    // Unwrap the DEK.
    parameters.value = wrapped.result;
    let unwrapped = client
        .unwrap_key(NAME, version.as_ref(), parameters.try_into()?, None)
        .await?
        .into_body()
        .await?;
    assert!(matches!(unwrapped.result, Some(result) if result.eq(&dek)));

    Ok(())
}

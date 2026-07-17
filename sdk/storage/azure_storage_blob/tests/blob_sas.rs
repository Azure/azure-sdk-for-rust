// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end tests for blob user delegation SAS generation.
//!
//! These tests build a SAS token from a real user delegation key with
//! [`SasBuilder`], assemble the authenticated URL by appending the token to the
//! resource URL, and then use that URL (via an unauthenticated client) to access
//! the resource. This proves the signature the SDK computes matches what the
//! service expects.

mod common;

use azure_core::{
    http::{RequestContent, Url, XmlFormat},
    time::OffsetDateTime,
};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{BlobClientGetPropertiesResultHeaders, KeyInfo};
use azure_storage_blob::{BlobClient, BlobServiceClient};
use azure_storage_sas::{SasBuilder, UserDelegationKey};
use common::{
    create_test_blob, get_blob_name, get_blob_service_client, get_container_client, StorageAccount,
};
use std::error::Error;
use time::Duration;

/// Gets a user delegation key valid for the next hour from the service.
async fn get_udk(service_client: &BlobServiceClient) -> Result<UserDelegationKey, Box<dyn Error>> {
    let now = OffsetDateTime::now_utc();
    let key_info = KeyInfo {
        start: Some(now),
        expiry: Some(now + Duration::hours(1)),
        ..Default::default()
    };
    let request_content: RequestContent<KeyInfo, XmlFormat> = key_info.try_into()?;
    let udk = service_client
        .get_user_delegation_key(request_content, None)
        .await?
        .into_model()?;
    Ok(udk)
}

/// Extracts the container name from a container client URL (its last non-empty
/// path segment).
fn extract_container_name(url: &Url) -> String {
    url.path_segments()
        .into_iter()
        .flatten()
        .rfind(|segment| !segment.is_empty())
        .unwrap_or_default()
        .to_string()
}

/// Base blob: generate a read SAS and download the blob through it.
#[recorded::test(live)]
async fn test_blob_user_delegation_sas_read(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let account_name = recording.var("AZURE_STORAGE_ACCOUNT_NAME", None);
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let container_name = extract_container_name(container_client.url());
    let blob_name = get_blob_name(recording);
    let blob_client = container_client.blob_client(&blob_name);

    let data = b"sas-e2e read content";
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(data.to_vec())),
        None,
    )
    .await?;

    let service_client = get_blob_service_client(recording, StorageAccount::Standard, None)?;
    let udk = get_udk(&service_client).await?;

    let token = SasBuilder::new(
        account_name.as_str(),
        &udk,
        OffsetDateTime::now_utc() + Duration::hours(1),
    )?
    .blob(&container_name, &blob_name)
    .read()
    .build();
    let mut sas_url = blob_client.url().clone();
    sas_url.set_query(Some(&token));

    // Download via an unauthenticated client using only the SAS URL.
    let sas_client = BlobClient::new(sas_url, None, None)?;
    let body = sas_client.download(None).await?.body.collect().await?;
    assert_eq!(data.to_vec(), body);

    container_client.delete(None).await?;
    Ok(())
}

/// Base blob: generate a write SAS and upload through it.
#[recorded::test(live)]
async fn test_blob_user_delegation_sas_write(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let account_name = recording.var("AZURE_STORAGE_ACCOUNT_NAME", None);
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let container_name = extract_container_name(container_client.url());
    let blob_name = get_blob_name(recording);
    let blob_client = container_client.blob_client(&blob_name);

    // Create the blob first (so the container/blob path exists), then overwrite via SAS.
    create_test_blob(&blob_client, None, None).await?;

    let service_client = get_blob_service_client(recording, StorageAccount::Standard, None)?;
    let udk = get_udk(&service_client).await?;

    let token = SasBuilder::new(
        account_name.as_str(),
        &udk,
        OffsetDateTime::now_utc() + Duration::hours(1),
    )?
    .blob(&container_name, &blob_name)
    .read()
    .write()
    .create()
    .build();
    let mut sas_url = blob_client.url().clone();
    sas_url.set_query(Some(&token));

    let sas_client = BlobClient::new(sas_url, None, None)?;
    let new_data = b"written through sas";
    sas_client
        .upload(RequestContent::from(new_data.to_vec()), None)
        .await?;

    // Read back with the authenticated client to confirm the write took effect.
    let body = blob_client.download(None).await?.body.collect().await?;
    assert_eq!(new_data.to_vec(), body);

    container_client.delete(None).await?;
    Ok(())
}

/// Blob version SAS (`sr=bv`): regression test for the bug where the version id
/// was not placed in the snapshot slot of the string-to-sign.
#[recorded::test(live)]
async fn test_blob_version_user_delegation_sas(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let account_name = recording.var("VERSIONED_AZURE_STORAGE_ACCOUNT_NAME", None);
    let container_client =
        get_container_client(recording, true, StorageAccount::Versioned, None).await?;
    let container_name = extract_container_name(container_client.url());
    let blob_name = get_blob_name(recording);
    let blob_client = container_client.blob_client(&blob_name);

    // Create two versions with distinct content.
    let data_v1 = b"version 1 content";
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(data_v1.to_vec())),
        None,
    )
    .await?;
    let version_1 = blob_client
        .get_properties(None)
        .await?
        .version_id()?
        .expect("version id for v1");

    let data_v2 = b"version 2 content";
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(data_v2.to_vec())),
        None,
    )
    .await?;

    let service_client = get_blob_service_client(recording, StorageAccount::Versioned, None)?;
    let udk = get_udk(&service_client).await?;

    // Generate a SAS scoped to version 1. The version id drives `sr=bv` and is
    // signed in the snapshot slot, but it is not emitted in the token, so the
    // base URL must carry `versionid=` (supplied here by `with_version`).
    let version_1_client = blob_client.with_version(&version_1)?;
    let token = SasBuilder::new(
        account_name.as_str(),
        &udk,
        OffsetDateTime::now_utc() + Duration::hours(1),
    )?
    .blob(&container_name, &blob_name)
    .version(&version_1)
    .read()
    .build();
    let mut sas_url = version_1_client.url().clone();
    // Preserve the existing `versionid=` query parameter.
    match sas_url.query() {
        Some(existing) if !existing.is_empty() => {
            sas_url.set_query(Some(&format!("{existing}&{token}")));
        }
        _ => sas_url.set_query(Some(&token)),
    }
    assert!(
        sas_url.query().is_some_and(|q| q.contains("sr=bv")),
        "expected sr=bv in SAS URL, got: {sas_url}"
    );

    // Download version 1 through the SAS; the bytes must match v1, not v2.
    let sas_client = BlobClient::new(sas_url, None, None)?;
    let body = sas_client.download(None).await?.body.collect().await?;
    assert_eq!(data_v1.to_vec(), body);

    container_client.delete(None).await?;
    Ok(())
}

/// Blob snapshot SAS (`sr=bs`): the snapshot timestamp is placed in the snapshot
/// slot and emitted as the `snapshot=` query parameter.
#[recorded::test(live)]
async fn test_blob_snapshot_user_delegation_sas(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    use azure_storage_blob::models::BlobClientCreateSnapshotResultHeaders;

    let recording = ctx.recording();
    let account_name = recording.var("AZURE_STORAGE_ACCOUNT_NAME", None);
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let container_name = extract_container_name(container_client.url());
    let blob_name = get_blob_name(recording);
    let blob_client = container_client.blob_client(&blob_name);

    let snapshot_data = b"snapshot content";
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(snapshot_data.to_vec())),
        None,
    )
    .await?;
    let snapshot = blob_client
        .create_snapshot(None)
        .await?
        .snapshot()?
        .expect("snapshot id");

    // Overwrite the base blob so a wrong-resource signature would return v2 data.
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(b"new base content".to_vec())),
        None,
    )
    .await?;

    let service_client = get_blob_service_client(recording, StorageAccount::Standard, None)?;
    let udk = get_udk(&service_client).await?;

    // The snapshot timestamp is signed in the snapshot slot and emitted as the
    // `snapshot=` query parameter of the token, so append the token to the plain
    // blob URL (the token already carries `snapshot=`).
    let token = SasBuilder::new(
        account_name.as_str(),
        &udk,
        OffsetDateTime::now_utc() + Duration::hours(1),
    )?
    .blob(&container_name, &blob_name)
    .snapshot(&snapshot)
    .read()
    .build();
    let mut sas_url = blob_client.url().clone();
    sas_url.set_query(Some(&token));
    assert!(
        sas_url.query().is_some_and(|q| q.contains("sr=bs")),
        "expected sr=bs in SAS URL, got: {sas_url}"
    );

    let sas_client = BlobClient::new(sas_url, None, None)?;
    let body = sas_client.download(None).await?.body.collect().await?;
    assert_eq!(snapshot_data.to_vec(), body);

    container_client.delete(None).await?;
    Ok(())
}

/// Container-level SAS: a SAS generated on the container grants access to a blob
/// within it.
#[recorded::test(live)]
async fn test_container_user_delegation_sas(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let account_name = recording.var("AZURE_STORAGE_ACCOUNT_NAME", None);
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let container_name = extract_container_name(container_client.url());
    let blob_name = get_blob_name(recording);
    let blob_client = container_client.blob_client(&blob_name);

    let data = b"container sas content";
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(data.to_vec())),
        None,
    )
    .await?;

    let service_client = get_blob_service_client(recording, StorageAccount::Standard, None)?;
    let udk = get_udk(&service_client).await?;

    let token = SasBuilder::new(
        account_name.as_str(),
        &udk,
        OffsetDateTime::now_utc() + Duration::hours(1),
    )?
    .container(&container_name)
    .read()
    .list()
    .build();

    // Build the blob URL within the container, then append the container SAS.
    let mut blob_url = container_client.url().clone();
    blob_url
        .path_segments_mut()
        .map_err(|_| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "cannot append blob name to container URL",
            )
        })?
        .push(&blob_name);
    blob_url.set_query(Some(&token));

    let sas_client = BlobClient::new(blob_url, None, None)?;
    let body = sas_client.download(None).await?.body.collect().await?;
    assert_eq!(data.to_vec(), body);

    container_client.delete(None).await?;
    Ok(())
}

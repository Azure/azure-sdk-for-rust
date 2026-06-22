// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! End-to-end tests for blob user delegation SAS generation.
//!
//! These tests generate a SAS URL from a real user delegation key and then use
//! the SAS (via an unauthenticated client) to access the resource. This proves
//! the signature the SDK computes matches what the service expects.

#![cfg(feature = "sas_builder")]

use azure_core::{
    http::{RequestContent, Url, XmlFormat},
    time::OffsetDateTime,
};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{
    sas::{BlobPermissions, ContainerPermissions, UserDelegationKey},
    BlobClientGetPropertiesResultHeaders, KeyInfo,
};
use azure_storage_blob::{BlobClient, BlobServiceClient};
use azure_storage_blob_test::{
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

/// Base blob: generate a read SAS and download the blob through it.
#[recorded::test(live)]
async fn test_blob_user_delegation_sas_read(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    let recording = ctx.recording();
    let account_name = recording.var("AZURE_STORAGE_ACCOUNT_NAME", None);
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    let data = b"sas-e2e read content";
    create_test_blob(
        &blob_client,
        Some(RequestContent::from(data.to_vec())),
        None,
    )
    .await?;

    let service_client = get_blob_service_client(recording, StorageAccount::Standard, None)?;
    let udk = get_udk(&service_client).await?;

    let sas_url = blob_client.generate_user_delegation_sas_url(
        &account_name,
        &udk,
        BlobPermissions::new().read(),
        OffsetDateTime::now_utc() + Duration::hours(1),
        |sas| sas,
    )?;

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
    let blob_client = container_client.blob_client(&get_blob_name(recording));

    // Create the blob first (so the container/blob path exists), then overwrite via SAS.
    create_test_blob(&blob_client, None, None).await?;

    let service_client = get_blob_service_client(recording, StorageAccount::Standard, None)?;
    let udk = get_udk(&service_client).await?;

    let sas_url = blob_client.generate_user_delegation_sas_url(
        &account_name,
        &udk,
        BlobPermissions::new().read().write().create(),
        OffsetDateTime::now_utc() + Duration::hours(1),
        |sas| sas,
    )?;

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
    let blob_client = container_client.blob_client(&get_blob_name(recording));

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

    // Generate a SAS scoped to version 1. `with_version` puts `versionid=` on the
    // endpoint, which drives `sr=bv` and places the version id in the signature.
    let version_1_client = blob_client.with_version(&version_1)?;
    let sas_url = version_1_client.generate_user_delegation_sas_url(
        &account_name,
        &udk,
        BlobPermissions::new().read(),
        OffsetDateTime::now_utc() + Duration::hours(1),
        |sas| sas,
    )?;
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
    let blob_client = container_client.blob_client(&get_blob_name(recording));

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

    let snapshot_client = blob_client.with_snapshot(&snapshot)?;
    let sas_url = snapshot_client.generate_user_delegation_sas_url(
        &account_name,
        &udk,
        BlobPermissions::new().read(),
        OffsetDateTime::now_utc() + Duration::hours(1),
        |sas| sas,
    )?;
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

    let container_sas_url = container_client.generate_user_delegation_sas_url(
        &account_name,
        &udk,
        ContainerPermissions::new().read().list(),
        OffsetDateTime::now_utc() + Duration::hours(1),
        |sas| sas,
    )?;

    // Build a blob URL that carries the container SAS query, then download.
    let mut blob_url = container_sas_url.clone();
    {
        let query = blob_url.query().map(str::to_owned);
        blob_url
            .path_segments_mut()
            .map_err(|_| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    "cannot append blob name to container SAS URL",
                )
            })?
            .push(&blob_name);
        blob_url.set_query(query.as_deref());
    }
    let _ = Url::parse(blob_url.as_str())?; // sanity: still a valid URL

    let sas_client = BlobClient::new(blob_url, None, None)?;
    let body = sas_client.download(None).await?.body.collect().await?;
    assert_eq!(data.to_vec(), body);

    container_client.delete(None).await?;
    Ok(())
}

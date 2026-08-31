// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod common;

use azure_core::{
    http::{RequestContent, StatusCode},
    time::{parse_rfc3339, to_rfc3339, OffsetDateTime},
};
use azure_core_test::{recorded, Matcher, TestContext, VarOptions};
use azure_storage_blob::format_filter_expression;
use azure_storage_blob::models::{
    AccessPolicy, AccountKind, BlobContainerClientAcquireLeaseResultHeaders,
    BlobContainerClientBreakLeaseOptions, BlobContainerClientChangeLeaseResultHeaders,
    BlobContainerClientCreateOptions, BlobContainerClientFindBlobsByTagsOptions,
    BlobContainerClientGetAccountInfoResultHeaders, BlobContainerClientGetPropertiesResultHeaders,
    BlobContainerClientSetMetadataOptions, BlockBlobClientUploadOptions, LeaseState,
    SignedIdentifiers, StorageErrorCode,
};
use azure_storage_blob::StorageError;
use common::{
    create_test_blob, get_blob_name, get_blob_service_client, get_container_client,
    get_container_name, poll_until, StorageAccount,
};
use futures::TryStreamExt;
use std::{collections::HashMap, error::Error, time::Duration};
use tokio::time;

#[recorded::test]
async fn test_create_container(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;

    container_client.create(None).await?;

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_container_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;

    // Container Doesn't Exists Scenario
    let response = container_client.get_properties(None).await;

    // Assert
    assert!(response.is_err());
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::NotFound, error.unwrap());
    assert!(!container_client.exists().await?);

    // Container Exists Scenario
    container_client.create(None).await?;
    let container_properties = container_client.get_properties(None).await?;
    let lease_state = container_properties.lease_state()?;
    let has_immutability_policy = container_properties.has_immutability_policy()?;

    // Assert
    assert_eq!(LeaseState::Available, lease_state.unwrap());
    assert!(!has_immutability_policy.unwrap());
    assert!(container_client.exists().await?);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_set_container_metadata(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    // Set Metadata With Values
    let update_metadata = HashMap::from([("hello".to_string(), "world".to_string())]);
    container_client
        .set_metadata(&update_metadata, None)
        .await?;

    // Assert
    let response = container_client.get_properties(None).await?;
    let response_metadata = response.metadata()?;
    assert_eq!(update_metadata, response_metadata);

    // Set Metadata No Values (Clear Metadata)
    container_client.set_metadata(&HashMap::new(), None).await?;

    // Assert
    let response = container_client.get_properties(None).await?;
    let response_metadata = response.metadata()?;
    assert_eq!(HashMap::new(), response_metadata);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_container_lease_operations(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let blob_service_client = get_blob_service_client(recording, StorageAccount::Standard, None)?;
    let container_name = get_container_name(recording);
    let container_client = blob_service_client.blob_container_client(&container_name.clone());
    let other_container_client = blob_service_client.blob_container_client(&container_name);
    container_client.create(None).await?;

    // Acquire Lease
    let acquire_response = container_client.acquire_lease(15, None).await?;
    let lease_id = acquire_response.lease_id()?.unwrap();
    let other_acquire_response = other_container_client.acquire_lease(15, None).await;
    // Assert
    let error = other_acquire_response.unwrap_err().http_status();
    assert_eq!(StatusCode::Conflict, error.unwrap());

    let update_metadata = HashMap::from([("hello".to_string(), "world".to_string())]);
    let set_metadata_options = BlobContainerClientSetMetadataOptions {
        lease_id: Some(lease_id.clone()),
        ..Default::default()
    };
    container_client
        .set_metadata(&update_metadata, Some(set_metadata_options))
        .await?;

    // Change Lease
    let proposed_lease_id = "00000000-1111-2222-3333-444444444444".to_string();
    let change_lease_response = container_client
        .change_lease(lease_id, proposed_lease_id.clone(), None)
        .await?;
    // Assert
    let lease_id = change_lease_response.lease_id()?.unwrap();
    assert_eq!(proposed_lease_id.clone().to_string(), lease_id);

    // Sleep until lease expires
    time::sleep(Duration::from_secs(15)).await;

    // Renew Lease
    container_client
        .renew_lease(proposed_lease_id.clone(), None)
        .await?;
    let other_acquire_response = other_container_client.acquire_lease(15, None).await;
    // Assert
    let error = other_acquire_response.unwrap_err().http_status();
    assert_eq!(StatusCode::Conflict, error.unwrap());

    // Break Lease
    container_client.break_lease(None).await?;
    let other_acquire_response = other_container_client.acquire_lease(15, None).await;
    // Assert
    let error = other_acquire_response.unwrap_err().http_status();
    assert_eq!(StatusCode::Conflict, error.unwrap());

    // Release Lease
    container_client
        .release_lease(proposed_lease_id.clone(), None)
        .await?;
    let other_acquire_response = other_container_client.acquire_lease(15, None).await;
    let lease_id = other_acquire_response?.lease_id().unwrap();
    other_container_client
        .release_lease(lease_id.unwrap(), None)
        .await?;

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_account_info(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    // Act
    let response = container_client.get_account_info(None).await?;

    // Assert
    let sku_name = response.sku_name()?;
    let account_kind = response.account_kind()?;

    assert!(sku_name.is_some());
    assert_eq!(AccountKind::StorageV2, account_kind.unwrap());

    Ok(())
}

#[recorded::test]
async fn test_find_blobs_by_tags(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    ctx.recording()
        .set_matcher(Matcher::HeaderlessMatcher)
        .await?;

    let container_client =
        get_container_client(ctx.recording(), true, StorageAccount::Standard, None).await?;

    // Create Test Blobs with Distinct Tags
    let blob1_name = get_blob_name(ctx.recording());
    let blob1_tags = HashMap::from([("fizz".to_string(), "buzz".to_string())]);
    create_test_blob(
        &container_client.blob_client(&blob1_name.clone()),
        Some(RequestContent::from("ferris the crab".as_bytes().into())),
        Some(BlockBlobClientUploadOptions::default().with_tags(blob1_tags.clone())),
    )
    .await?;

    // Create 3 blobs sharing the same tag to exercise the max-results option
    let shared_tag = HashMap::from([("env".to_string(), "test".to_string())]);
    for _ in 0..3 {
        let name = get_blob_name(ctx.recording());
        create_test_blob(
            &container_client.blob_client(&name),
            Some(RequestContent::from("data".as_bytes().into())),
            Some(BlockBlobClientUploadOptions::default().with_tags(shared_tag.clone())),
        )
        .await?;
    }

    let blob2_name = get_blob_name(ctx.recording());
    create_test_blob(
        &container_client.blob_client(&blob2_name.clone()),
        Some(RequestContent::from("hello world".as_bytes().into())),
        Some(
            BlockBlobClientUploadOptions::default().with_tags(HashMap::from([
                ("foo".to_string(), "bar".to_string()),
                ("alice".to_string(), "bob".to_string()),
            ])),
        ),
    )
    .await?;

    // Find "hello world" blob by its tag {"foo": "bar"}.
    // In live mode, poll until tags are indexed (up to 60s total timeout).
    // In record mode, use a fixed 15s sleep.
    poll_until(ctx.recording(), || async {
        let mut pager = container_client
            .find_blobs_by_tags("\"foo\"='bar'", None)?
            .into_pages();
        if let Some(resp) = pager.try_next().await? {
            let segment = resp.into_model()?;
            if segment
                .blob_items
                .iter()
                .any(|b| b.name.as_deref() == Some(blob2_name.as_str()))
            {
                return Ok(true);
            }
        }
        Ok(false)
    })
    .await?;
    // Final assertion (covers record/playback where poll_until doesn't check the condition)
    {
        let mut pager = container_client
            .find_blobs_by_tags("\"foo\"='bar'", None)?
            .into_pages();
        let filter_blob_segment = pager
            .try_next()
            .await?
            .expect("expected at least one page from find_blobs_by_tags")
            .into_model()?;
        let blobs = &filter_blob_segment.blob_items;
        assert!(
            blobs
                .iter()
                .any(|blob| blob.name.as_deref() == Some(blob2_name.as_str())),
            "Failed to find \"{blob2_name}\" in filtered blob results."
        );
    }

    // Find "ferris the crab" blob by its tag {"fizz": "buzz"}
    let mut pager = container_client
        .find_blobs_by_tags(&format_filter_expression(&blob1_tags)?, None)?
        .into_pages();
    let filter_blob_segment = pager
        .try_next()
        .await?
        .expect("expected at least one page from find_blobs_by_tags")
        .into_model()?;
    let blobs = &filter_blob_segment.blob_items;
    assert!(
        blobs
            .iter()
            .any(|blob| blob.name.as_deref() == Some(blob1_name.as_str())),
        "Failed to find \"{blob1_name}\" in filtered blob results."
    );

    // Max Results Scenario
    let options = BlobContainerClientFindBlobsByTagsOptions {
        maxresults: Some(2),
        ..Default::default()
    };
    let mut pager = container_client
        .find_blobs_by_tags("\"env\"='test'", Some(options))?
        .into_pages();
    let page = pager
        .try_next()
        .await?
        .expect("expected at least one page from find_blobs_by_tags")
        .into_model()?;
    let blobs = &page.blob_items;
    assert!(
        blobs.len() <= 2,
        "page should contain at most 2 blobs due to maxresults=2, got {}",
        blobs.len()
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_container_access_policy(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();

    ctx.recording()
        .set_matcher(Matcher::BodilessMatcher)
        .await?;

    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create(None).await?;

    // Set Access Policy w/ Multiple Policy Defined
    let expiry = recording.var(
        "expiry",
        Some(VarOptions {
            default_value: Some(
                to_rfc3339(&(OffsetDateTime::now_utc() + Duration::from_secs(10))).into(),
            ),
            ..Default::default()
        }),
    );
    let start = recording.var(
        "start",
        Some(VarOptions {
            default_value: Some(to_rfc3339(&OffsetDateTime::now_utc()).into()),
            ..Default::default()
        }),
    );
    let test_id_1: Option<String> = Some("testid_1".into());
    let test_id_2: Option<String> = Some("testid_2".into());
    let access_policy_1 = AccessPolicy {
        expiry: Some(parse_rfc3339(&expiry)?),
        permission: Some("rw".to_string()),
        start: Some(parse_rfc3339(&start)?),
    };
    let access_policy_2 = AccessPolicy {
        expiry: Some(parse_rfc3339(&expiry)?),
        permission: Some("cd".to_string()),
        start: Some(parse_rfc3339(&start)?),
    };
    let policies: HashMap<String, AccessPolicy> = HashMap::from([
        (test_id_1.clone().unwrap(), access_policy_1.clone()),
        (test_id_2.clone().unwrap(), access_policy_2.clone()),
    ]);
    container_client
        .set_access_policy(
            RequestContent::try_from(SignedIdentifiers::from(policies))?,
            None,
        )
        .await?;

    Ok(())
}

#[recorded::test]
async fn test_create_container_with_metadata(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;

    let metadata = HashMap::from([
        ("author".to_string(), "ferris".to_string()),
        ("project".to_string(), "azure-sdk-for-rust".to_string()),
    ]);
    container_client
        .create(Some(BlobContainerClientCreateOptions {
            metadata: Some(metadata.clone()),
            ..Default::default()
        }))
        .await?;

    let props = container_client.get_properties(None).await?;
    assert_eq!(metadata, props.metadata()?);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_break_lease_with_break_period(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    // Acquire Lease
    container_client.acquire_lease(60, None).await?;

    // Break Lease
    let options = BlobContainerClientBreakLeaseOptions {
        break_period: Some(0),
        ..Default::default()
    };
    container_client.break_lease(Some(options)).await?;

    // Assert
    let acquire_response = container_client.acquire_lease(15, None).await?;
    let new_lease_id = acquire_response.lease_id()?.unwrap();
    container_client.release_lease(new_lease_id, None).await?;

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_container_error_codes(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    // Do NOT create the container yet
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;

    // ContainerNotFound - get_properties before the container exists
    let err = container_client.get_properties(None).await.unwrap_err();
    let storage_error: StorageError = err.try_into()?;
    assert_eq!(
        storage_error.error_code.as_ref(),
        Some(&StorageErrorCode::ContainerNotFound),
        "expected ContainerNotFound error code"
    );

    // Create the container so it now exists
    container_client.create(None).await?;

    // ContainerAlreadyExists - create it a second time
    let err = container_client.create(None).await.unwrap_err();
    let storage_error: StorageError = err.try_into()?;
    assert_eq!(
        storage_error.error_code.as_ref(),
        Some(&StorageErrorCode::ContainerAlreadyExists),
        "expected ContainerAlreadyExists error code"
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_lease_already_present_error_code(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    // Acquire the lease on the container
    let acquire = container_client.acquire_lease(-1, None).await?;
    let lease_id = acquire.lease_id()?.expect("lease_id must be present");

    // Attempt a second acquire - should fail with LeaseAlreadyPresent
    let err = container_client.acquire_lease(-1, None).await.unwrap_err();
    let storage_error: StorageError = err.try_into()?;

    // Assert
    assert_eq!(
        storage_error.error_code.as_ref(),
        Some(&StorageErrorCode::LeaseAlreadyPresent),
        "expected LeaseAlreadyPresent error code"
    );

    // Clean up
    container_client.release_lease(lease_id, None).await?;
    container_client.delete(None).await?;
    Ok(())
}

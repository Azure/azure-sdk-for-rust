// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{RequestContent, StatusCode},
    time::{parse_rfc3339, to_rfc3339, OffsetDateTime},
};
use azure_core_test::{recorded, TestContext, TestMode, VarOptions};
use azure_storage_blob::format_filter_expression;
use azure_storage_blob::models::{
    AccessPolicy, AccountKind, BlobContainerClientAcquireLeaseResultHeaders,
    BlobContainerClientChangeLeaseResultHeaders, BlobContainerClientGetAccountInfoResultHeaders,
    BlobContainerClientGetPropertiesResultHeaders, BlobContainerClientListBlobFlatSegmentOptions,
    BlobContainerClientSetMetadataOptions, BlobType, BlockBlobClientUploadOptions, LeaseState,
    SignedIdentifiers,
};
use azure_storage_blob_test::{
    create_test_blob, get_blob_name, get_blob_service_client, get_container_client,
    get_container_name, StorageAccount,
};
use futures::{StreamExt, TryStreamExt};
use std::{collections::HashMap, error::Error, time::Duration};
use tokio::time;

#[recorded::test]
async fn test_create_container(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;

    container_client.create_container(None).await?;

    container_client.delete_container(None).await?;
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
    container_client.create_container(None).await?;
    let container_properties = container_client.get_properties(None).await?;
    let lease_state = container_properties.lease_state()?;
    let has_immutability_policy = container_properties.has_immutability_policy()?;

    // Assert
    assert_eq!(LeaseState::Available, lease_state.unwrap());
    assert!(!has_immutability_policy.unwrap());
    assert!(container_client.exists().await?);

    container_client.delete_container(None).await?;
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
        .set_metadata(update_metadata.clone(), None)
        .await?;

    // Assert
    let response = container_client.get_properties(None).await?;
    let response_metadata = response.metadata()?;
    assert_eq!(update_metadata, response_metadata);

    // Set Metadata No Values (Clear Metadata)
    container_client.set_metadata(HashMap::new(), None).await?;

    // Assert
    let response = container_client.get_properties(None).await?;
    let response_metadata = response.metadata()?;
    assert_eq!(HashMap::new(), response_metadata);

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    let blob_names = ["testblob1".to_string(), "testblob2".to_string()];

    container_client.create_container(None).await?;
    create_test_blob(
        &container_client.blob_client(&blob_names[0].clone()),
        None,
        None,
    )
    .await?;
    create_test_blob(
        &container_client.blob_client(&blob_names[1].clone()),
        None,
        None,
    )
    .await?;

    let mut list_blobs_response = container_client.list_blobs(None)?.into_pages();

    let page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = page.unwrap().into_model()?;
    let blob_list = list_blob_segment_response.segment.blob_items;
    for blob in blob_list {
        let blob_name = blob.name.unwrap().content.unwrap();
        let properties = blob.properties.unwrap();
        let blob_type = properties.blob_type.unwrap();
        let etag = properties.etag;
        assert!(blob_names.contains(&blob_name));
        assert_eq!(BlobType::BlockBlob, blob_type);
        assert!(etag.is_some());
    }

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_with_continuation(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    let blob_names = [
        "testblob1".to_string(),
        "testblob2".to_string(),
        "testblob3".to_string(),
        "testblob4".to_string(),
    ];

    container_client.create_container(None).await?;
    create_test_blob(
        &container_client.blob_client(&blob_names[0].clone()),
        None,
        None,
    )
    .await?;
    create_test_blob(
        &container_client.blob_client(&blob_names[1].clone()),
        None,
        None,
    )
    .await?;
    create_test_blob(
        &container_client.blob_client(&blob_names[2].clone()),
        None,
        None,
    )
    .await?;
    create_test_blob(
        &container_client.blob_client(&blob_names[3].clone()),
        None,
        None,
    )
    .await?;

    // Continuation Token with Token Provided
    let list_blobs_options = BlobContainerClientListBlobFlatSegmentOptions {
        maxresults: Some(2),
        ..Default::default()
    };
    let mut list_blobs_response = container_client
        .list_blobs(Some(list_blobs_options))?
        .into_pages();
    let first_page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = first_page.unwrap().into_model()?;
    let continuation_token = list_blob_segment_response.next_marker;
    let blob_list = list_blob_segment_response.segment.blob_items;
    assert_eq!(2, blob_list.len());
    for blob in blob_list {
        let blob_name = blob.name.unwrap().content.unwrap();
        let blob_type = blob.properties.unwrap().blob_type.unwrap();
        assert!(blob_names.contains(&blob_name));
        assert_eq!(BlobType::BlockBlob, blob_type);
    }
    let list_blobs_options = BlobContainerClientListBlobFlatSegmentOptions {
        marker: continuation_token,
        ..Default::default()
    };
    let mut list_blobs_response = container_client
        .list_blobs(Some(list_blobs_options.clone()))?
        .into_pages();
    let second_page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = second_page.unwrap().into_model()?;
    let blob_list = list_blob_segment_response.segment.blob_items;
    assert_eq!(2, blob_list.len());
    for blob in blob_list {
        let blob_name = blob.name.unwrap().content.unwrap();
        let blob_type = blob.properties.unwrap().blob_type.unwrap();
        assert!(blob_names.contains(&blob_name));
        assert_eq!(BlobType::BlockBlob, blob_type);
    }

    // Continuation Token, Automatic Paging
    let mut pager_response = container_client
        .list_blobs(Some(list_blobs_options))?
        .into_pages();
    let mut page_count = 0;

    while let Some(page) = pager_response.next().await {
        page_count += 1;
        let current_page = page.unwrap().into_model()?;
        match page_count {
            1 => {
                let blob_list = current_page.segment.blob_items;
                assert_eq!(2, blob_list.len());

                for blob in blob_list {
                    let blob_name = blob.name.unwrap().content.unwrap();
                    let blob_type = blob.properties.unwrap().blob_type.unwrap();
                    assert!(blob_names.contains(&blob_name));
                    assert_eq!(BlobType::BlockBlob, blob_type);
                }
            }
            2 => {
                let blob_list = current_page.segment.blob_items;
                assert_eq!(2, blob_list.len());

                for blob in blob_list {
                    let blob_name = blob.name.unwrap().content.unwrap();
                    let blob_type = blob.properties.unwrap().blob_type.unwrap();
                    assert!(blob_names.contains(&blob_name));
                    assert_eq!(BlobType::BlockBlob, blob_type);
                }
            }
            _ => {
                panic!("Unexpected page number reached.")
            }
        }
    }

    container_client.delete_container(None).await?;
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
    container_client.create_container(None).await?;

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
        .set_metadata(update_metadata, Some(set_metadata_options))
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

    container_client.delete_container(None).await?;
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
async fn test_find_blobs_by_tags_container(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup

    // Work around change to query parameter ordering introduced in https://github.com/Azure/azure-sdk-for-rust/pull/3437.
    // Tracking reversion: https://github.com/Azure/azure-sdk-for-rust/issues/3438.
    // Revert to `Matcher::HeaderlessMatcher`.
    ctx.recording()
        .set_matcher(
            azure_core_test::CustomDefaultMatcher {
                excluded_headers: vec!["x-ms-tags"],
                ignore_query_ordering: Some(true),
                ..Default::default()
            }
            .into(),
        )
        .await?;

    let container_client =
        get_container_client(ctx.recording(), true, StorageAccount::Standard, None).await?;

    // Create Test Blobs with Tags
    let blob1_name = get_blob_name(ctx.recording());
    create_test_blob(
        &container_client.blob_client(&blob1_name.clone()),
        Some(RequestContent::from("hello world".as_bytes().into())),
        Some(
            BlockBlobClientUploadOptions::default().with_tags(HashMap::from([
                ("foo".to_string(), "bar".to_string()),
                ("alice".to_string(), "bob".to_string()),
            ])),
        ),
    )
    .await?;
    let blob2_name = get_blob_name(ctx.recording());
    let blob2_tags = HashMap::from([("fizz".to_string(), "buzz".to_string())]);
    create_test_blob(
        &container_client.blob_client(&blob2_name.clone()),
        Some(RequestContent::from("ferris the crab".as_bytes().into())),
        Some(BlockBlobClientUploadOptions::default().with_tags(blob2_tags.clone())),
    )
    .await?;

    // Sleep in live mode to allow tags to be indexed on the service
    if ctx.recording().test_mode() == TestMode::Live
        || ctx.recording().test_mode() == TestMode::Record
    {
        time::sleep(Duration::from_secs(5)).await;
    }

    // Find "hello world" blob by its tag {"foo": "bar"}
    let response = container_client
        .find_blobs_by_tags("\"foo\"='bar'", None)
        .await?;
    let filter_blob_segment = response.into_model()?;
    let blobs = filter_blob_segment.blobs.unwrap();
    assert!(
        blobs
            .iter()
            .any(|blob| blob.name.as_ref().unwrap() == &blob1_name),
        "Failed to find \"{blob1_name}\" in filtered blob results."
    );

    // Find "ferris the crab" blob by its tag {"fizz": "buzz"}
    let response = container_client
        .find_blobs_by_tags(&format_filter_expression(&blob2_tags)?, None)
        .await?;
    let filter_blob_segment = response.into_model()?;
    let blobs = filter_blob_segment.blobs.unwrap();
    assert!(
        blobs
            .iter()
            .any(|blob| blob.name.as_ref().unwrap() == &blob2_name),
        "Failed to find \"{blob2_name}\" in filtered blob results."
    );

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_container_access_policy(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();

    // Work around change to query parameter ordering introduced in https://github.com/Azure/azure-sdk-for-rust/pull/3437.
    // Tracking reversion: https://github.com/Azure/azure-sdk-for-rust/issues/3438.
    // Revert to `Matcher::Matcher::BodilessMatcher`.
    recording
        .set_matcher(
            azure_core_test::CustomDefaultMatcher {
                compare_bodies: Some(false),
                ignore_query_ordering: Some(true),
                ..Default::default()
            }
            .into(),
        )
        .await?;

    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create_container(None).await?;

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

    // Sleep in live mode to allow signed identifiers to be indexed on the service
    if ctx.recording().test_mode() == TestMode::Live
        || ctx.recording().test_mode() == TestMode::Record
    {
        time::sleep(Duration::from_secs(5)).await;
    }

    // Assert
    let response = container_client.get_access_policy(None).await?;
    let signed_identifiers = response.into_model()?.items.unwrap();
    assert_eq!(2, signed_identifiers.len());

    let expected_policies = HashMap::from([
        (test_id_1.clone().unwrap(), access_policy_1.clone()),
        (test_id_2.clone().unwrap(), access_policy_2.clone()),
    ]);

    for signed_identifier in signed_identifiers {
        let id = signed_identifier.id.unwrap();
        let returned_policy = signed_identifier.access_policy.unwrap();
        let expected_policy = expected_policies.get(&id).expect("Unexpected ID returned");

        // Truncate start and expiry times to seconds precision for assertion
        assert_eq!(
            expected_policy
                .start
                .map(|dt| dt.replace_nanosecond(0).unwrap()),
            returned_policy
                .start
                .map(|dt| dt.replace_nanosecond(0).unwrap()),
            "Start times don't match (truncated to seconds precision)"
        );
        assert_eq!(
            expected_policy
                .expiry
                .map(|dt| dt.replace_nanosecond(0).unwrap()),
            returned_policy
                .expiry
                .map(|dt| dt.replace_nanosecond(0).unwrap()),
            "Expiry times don't match (truncated to seconds precision)"
        );
        assert_eq!(expected_policy.permission, returned_policy.permission);
    }

    // Clear Access Policy
    let clear_signed_identifiers: SignedIdentifiers = HashMap::<String, AccessPolicy>::new().into();
    container_client
        .set_access_policy(RequestContent::try_from(clear_signed_identifiers)?, None)
        .await?;

    // Assert
    let cleared_response = container_client.get_access_policy(None).await?;
    let cleared_signed_identifiers = cleared_response.into_model()?;
    assert!(cleared_signed_identifiers.items.is_none());

    Ok(())
}

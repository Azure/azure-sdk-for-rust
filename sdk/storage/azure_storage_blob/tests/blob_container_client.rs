// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::StatusCode,
    time::{Duration, OffsetDateTime},
};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::format_datetime;
use azure_storage_blob::models::{
    AccessPolicy, AccountKind, BlobContainerClientAcquireLeaseResultHeaders,
    BlobContainerClientChangeLeaseResultHeaders, BlobContainerClientGetAccountInfoResultHeaders,
    BlobContainerClientGetPropertiesResultHeaders, BlobContainerClientListBlobFlatSegmentOptions,
    BlobContainerClientSetMetadataOptions, BlobType, LeaseState, SignedIdentifier,
};
use azure_storage_blob_test::{
    create_test_blob, get_blob_service_client, get_container_client, get_container_name,
};
use futures::{StreamExt, TryStreamExt};
use std::{collections::HashMap, error::Error};
use tokio::time;

#[recorded::test]
async fn test_create_container(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, false).await?;

    container_client.create_container(None).await?;

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_container_properties(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, false).await?;

    // Container Doesn't Exists Scenario
    let response = container_client.get_properties(None).await;

    // Assert
    assert!(response.is_err());
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::NotFound, error.unwrap());

    // Container Exists Scenario
    container_client.create_container(None).await?;
    let container_properties = container_client.get_properties(None).await?;
    let lease_state = container_properties.lease_state()?;
    let has_immutability_policy = container_properties.has_immutability_policy()?;

    // Assert
    assert_eq!(LeaseState::Available, lease_state.unwrap());
    assert!(!has_immutability_policy.unwrap());

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_set_container_metadata(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client = get_container_client(recording, true).await?;

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
    let container_client = get_container_client(recording, false).await?;
    let blob_names = ["testblob1".to_string(), "testblob2".to_string()];

    container_client.create_container(None).await?;
    create_test_blob(
        &container_client.blob_client(blob_names[0].clone()),
        None,
        None,
    )
    .await?;
    create_test_blob(
        &container_client.blob_client(blob_names[1].clone()),
        None,
        None,
    )
    .await?;

    let mut list_blobs_response = container_client.list_blobs(None)?;

    let page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = page.unwrap().into_body().await?;
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
    let container_client = get_container_client(recording, false).await?;
    let blob_names = [
        "testblob1".to_string(),
        "testblob2".to_string(),
        "testblob3".to_string(),
        "testblob4".to_string(),
    ];

    container_client.create_container(None).await?;
    create_test_blob(
        &container_client.blob_client(blob_names[0].clone()),
        None,
        None,
    )
    .await?;
    create_test_blob(
        &container_client.blob_client(blob_names[1].clone()),
        None,
        None,
    )
    .await?;
    create_test_blob(
        &container_client.blob_client(blob_names[2].clone()),
        None,
        None,
    )
    .await?;
    create_test_blob(
        &container_client.blob_client(blob_names[3].clone()),
        None,
        None,
    )
    .await?;

    // Continuation Token with Token Provided
    let list_blobs_options = BlobContainerClientListBlobFlatSegmentOptions {
        maxresults: Some(2),
        ..Default::default()
    };
    let mut list_blobs_response = container_client.list_blobs(Some(list_blobs_options))?;
    let first_page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = first_page.unwrap().into_body().await?;
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
    let mut list_blobs_response = container_client.list_blobs(Some(list_blobs_options.clone()))?;
    let second_page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = second_page.unwrap().into_body().await?;
    let blob_list = list_blob_segment_response.segment.blob_items;
    assert_eq!(2, blob_list.len());
    for blob in blob_list {
        let blob_name = blob.name.unwrap().content.unwrap();
        let blob_type = blob.properties.unwrap().blob_type.unwrap();
        assert!(blob_names.contains(&blob_name));
        assert_eq!(BlobType::BlockBlob, blob_type);
    }

    // Continuation Token, Automatic Paging
    let mut pager_response = container_client.list_blobs(Some(list_blobs_options))?;
    let mut page_count = 0;

    while let Some(page) = pager_response.next().await {
        page_count += 1;
        let current_page = page.unwrap().into_body().await?;
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
    let blob_service_client = get_blob_service_client(recording)?;
    let container_name = get_container_name(recording);
    let container_client = blob_service_client.blob_container_client(container_name.clone());
    let other_container_client = blob_service_client.blob_container_client(container_name);
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
    time::sleep(std::time::Duration::from_secs(15)).await;

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
    let container_client = get_container_client(recording, true).await?;

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
async fn test_container_access_policy(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup

    use azure_core::time::parse_rfc3339;
    let recording = ctx.recording();
    let container_client = get_container_client(recording, false).await?;
    container_client.create_container(None).await?;

    // // Set Access Policy w/ Policy Defined
    // let access_policy = AccessPolicy {
    //     expiry: Some(format_datetime(
    //         OffsetDateTime::now_utc() + Duration::seconds(10),
    //     )?),
    //     permission: Some("rw".to_string()),
    //     start: Some(format_datetime(OffsetDateTime::now_utc())?),
    // };
    // let signed_identifier = SignedIdentifier {
    //     access_policy: Some(access_policy),
    //     id: Some("testid".into()),
    // };

    // Set Access Policy w/ Policy Defined
    let dt_start = parse_rfc3339("2025-07-22T19:01:10.622383-05:00").unwrap();
    let formatted_start = format_datetime(dt_start)?;
    let dt_expiry = parse_rfc3339("2025-12-22T19:01:20.622383-05:00").unwrap();
    let formatted_expiry = format_datetime(dt_expiry)?;
    let access_policy = AccessPolicy {
        expiry: Some(formatted_expiry),
        permission: Some("rw".to_string()),
        start: Some(formatted_start),
    };
    let signed_identifier = SignedIdentifier {
        access_policy: Some(access_policy),
        id: Some("testid".into()),
    };

    container_client
        .set_access_policy(vec![signed_identifier], None)
        .await?;

    container_client.delete_container(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_container_access_policy_datetime_comprehensive(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup - comprehensive test covering all datetime scenarios from parsers
    use azure_core::time::parse_rfc3339;
    let recording = ctx.recording();
    let container_client = get_container_client(recording, false).await?;
    container_client.create_container(None).await?;

    // Comprehensive test cases that match ALL scenarios from the parsers unit tests
    let comprehensive_test_cases = [
        // Precision levels (from test_format_datetime_with_7_decimals)
        (
            "no-fractional-utc",
            "2025-09-22T19:20:00Z",
            "2025-09-22T19:20:00.0000000Z",
            "r",
        ),
        (
            "single-digit-padding",
            "2025-09-22T19:20:00.1Z",
            "2025-09-22T19:20:00.1000000Z",
            "w",
        ),
        (
            "millisecond-padding",
            "2025-09-22T19:20:00.123Z",
            "2025-09-22T19:20:00.1230000Z",
            "d",
        ),
        (
            "microsecond-padding",
            "2025-09-22T19:20:10.622383Z",
            "2025-09-22T19:20:10.6223830Z",
            "l",
        ),
        (
            "exact-7-digits",
            "2025-09-22T19:20:00.1234567Z",
            "2025-09-22T19:20:00.1234567Z",
            "rw",
        ),
        (
            "nanosecond-truncation",
            "2025-09-22T19:20:00.622429456Z",
            "2025-09-22T19:20:00.6224294Z",
            "rd",
        ),
        // Timezone offset conversions (from test_format_datetime_no_fractional_seconds)
        (
            "negative-offset-conversion",
            "2025-09-22T19:20:00-05:00",
            "2025-09-23T00:20:00.0000000Z",
            "rwl",
        ),
        (
            "positive-offset-conversion",
            "2025-09-22T19:20:00+03:30",
            "2025-09-22T15:50:00.0000000Z",
            "wdl",
        ),
        // Edge cases (from test_format_datetime_edge_cases)
        (
            "boundary-timezone-positive",
            "2025-09-22T19:20:00.123+14:00",
            "2025-09-22T05:20:00.1230000Z",
            "rwdl",
        ),
        (
            "boundary-timezone-negative",
            "2019-10-12T00:20:50.52-08:00",
            "2019-10-12T08:20:50.5200000Z",
            "wd",
        ),
        // High precision with timezone offsets (from parser edge cases)
        (
            "microseconds-positive-offset",
            "1999-09-10T03:05:07.3845533+01:00",
            "1999-09-10T02:05:07.3845533Z",
            "dl",
        ),
        // UTC variants with different precisions
        (
            "utc-basic-fractional",
            "2019-10-12T07:20:50.52Z",
            "2019-10-12T07:20:50.5200000Z",
            "rwd",
        ),
        // Edge year (RFC 3339 example)
        (
            "rfc3339-example-year",
            "1985-04-12T23:20:50.52Z",
            "1985-04-12T23:20:50.5200000Z",
            "rl",
        ),
        // Equivalent time representations (same UTC moment in different formats)
        (
            "equiv-utc",
            "2022-08-26T18:38:00Z",
            "2022-08-26T18:38:00.0000000Z",
            "r",
        ),
        (
            "equiv-neg-offset",
            "2022-08-26T10:38:00-08:00",
            "2022-08-26T18:38:00.0000000Z",
            "w",
        ),
        (
            "equiv-pos-offset",
            "2022-08-26T20:38:00+02:00",
            "2022-08-26T18:38:00.0000000Z",
            "d",
        ),
        (
            "equiv-microseconds",
            "2022-08-26T18:38:00.000000Z",
            "2022-08-26T18:38:00.0000000Z",
            "l",
        ),
    ];

    for (test_name, input_datetime, expected_output, permission) in comprehensive_test_cases {
        // Test each scenario individually to ensure service accepts all formats
        let dt_start = parse_rfc3339(input_datetime).unwrap();
        let formatted_start = format_datetime(dt_start)?;
        let dt_expiry = parse_rfc3339("2025-12-31T23:59:59Z").unwrap();
        let formatted_expiry = format_datetime(dt_expiry)?;

        // Verify the formatter produces the expected precision/conversion
        assert_eq!(
            formatted_start, expected_output,
            "Failed for test case: {} with input: {}",
            test_name, input_datetime
        );

        let access_policy = AccessPolicy {
            expiry: Some(formatted_expiry),
            permission: Some(permission.to_string()),
            start: Some(formatted_start),
        };
        let signed_identifier = SignedIdentifier {
            access_policy: Some(access_policy),
            id: Some(format!("comprehensive-{}", test_name)),
        };

        // Test that the service accepts each datetime format
        container_client
            .set_access_policy(vec![signed_identifier], None)
            .await?;
    }

    container_client.delete_container(None).await?;
    Ok(())
}

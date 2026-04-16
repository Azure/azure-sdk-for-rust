// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{RequestContent, StatusCode},
    time::{parse_rfc3339, to_rfc3339, OffsetDateTime},
};
use azure_core_test::{recorded, Matcher, TestContext, TestMode, VarOptions};
use azure_storage_blob::format_filter_expression;
use azure_storage_blob::models::{
    AccessPolicy, AccountKind, BlobContainerClientAcquireLeaseResultHeaders,
    BlobContainerClientBreakLeaseOptions, BlobContainerClientChangeLeaseResultHeaders,
    BlobContainerClientCreateOptions, BlobContainerClientFindBlobsByTagsOptions,
    BlobContainerClientGetAccountInfoResultHeaders, BlobContainerClientGetPropertiesResultHeaders,
    BlobContainerClientListBlobsOptions, BlobContainerClientSetMetadataOptions, BlobType,
    BlockBlobClientUploadOptions, LeaseState, ListBlobsIncludeItem, SignedIdentifiers,
    StorageErrorCode,
};
use azure_storage_blob::StorageError;
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
async fn test_list_blobs(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    let blob_names = ["testblob1".to_string(), "testblob2".to_string()];

    container_client.create(None).await?;
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
        let blob_name = blob.name.unwrap();
        let properties = blob.properties.unwrap();
        let blob_type = properties.blob_type.unwrap();
        let etag = properties.etag;
        assert!(blob_names.contains(&blob_name));
        assert_eq!(BlobType::BlockBlob, blob_type);
        assert!(etag.is_some());
    }

    container_client.delete(None).await?;
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

    container_client.create(None).await?;
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
    let list_blobs_options = BlobContainerClientListBlobsOptions {
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
        let blob_name = blob.name.unwrap();
        let blob_type = blob.properties.unwrap().blob_type.unwrap();
        assert!(blob_names.contains(&blob_name));
        assert_eq!(BlobType::BlockBlob, blob_type);
    }
    let list_blobs_options = BlobContainerClientListBlobsOptions {
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
        let blob_name = blob.name.unwrap();
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
                    let blob_name = blob.name.unwrap();
                    let blob_type = blob.properties.unwrap().blob_type.unwrap();
                    assert!(blob_names.contains(&blob_name));
                    assert_eq!(BlobType::BlockBlob, blob_type);
                }
            }
            2 => {
                let blob_list = current_page.segment.blob_items;
                assert_eq!(2, blob_list.len());

                for blob in blob_list {
                    let blob_name = blob.name.unwrap();
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

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_list_blobs_decodes_xml_invalid_names(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;

    // Upload blobs with XML-invalid characters (U+FFFE and U+FFFF) in their names.
    // Per the Storage REST API (version 2021-02-12+), List Blobs will percent-encode
    // Name values containing these characters and set Encoded="true" on the element.
    let test_cases = [
        ("blob_with_fffe", "blob\u{FFFE}name".to_string()),
        ("blob_with_ffff", "blob\u{FFFF}name".to_string()),
        ("blob_with_both", "blob\u{FFFE}and\u{FFFF}chars".to_string()),
    ];

    for (_, blob_name) in &test_cases {
        let blob_client = container_client.blob_client(blob_name);
        create_test_blob(&blob_client, None, None).await?;
    }

    // List blobs and verify the names are correctly percent-decoded
    let mut list_blobs_response = container_client.list_blobs(None)?.into_pages();
    let page = list_blobs_response.try_next().await?;
    let list_blob_segment_response = page.unwrap().into_model()?;
    let blob_items = list_blob_segment_response.segment.blob_items;

    // Assert
    assert_eq!(test_cases.len(), blob_items.len());

    let listed_blob_names: Vec<String> = blob_items
        .iter()
        .map(|blob| blob.name.clone().unwrap())
        .collect();

    for (label, expected_name) in &test_cases {
        assert!(
            listed_blob_names.contains(expected_name),
            "Blob '{}' with name '{}' not found in listed names: {:?}",
            label,
            expected_name,
            listed_blob_names
        );
    }

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

    // Max Results Scenario
    let options = BlobContainerClientFindBlobsByTagsOptions {
        maxresults: Some(2),
        ..Default::default()
    };
    let response = container_client
        .find_blobs_by_tags("\"env\"='test'", Some(options))
        .await?;
    let page = response.into_model()?;
    let blobs = page.blobs.unwrap_or_default();
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

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
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
#[ignore = "need to investigate live test pipeline failures"]
async fn test_list_blobs_with_include_options(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create(None).await?;

    // Create a blob with metadata and one with tags
    let metadata_blob_name = get_blob_name(recording);
    let tags_blob_name = get_blob_name(recording);
    let metadata = HashMap::from([("team".to_string(), "sdk".to_string())]);
    create_test_blob(
        &container_client.blob_client(&metadata_blob_name),
        None,
        Some(BlockBlobClientUploadOptions {
            metadata: Some(metadata.clone()),
            ..Default::default()
        }),
    )
    .await?;
    create_test_blob(
        &container_client.blob_client(&tags_blob_name),
        None,
        Some(
            BlockBlobClientUploadOptions::default()
                .with_tags(HashMap::from([("env".to_string(), "test".to_string())])),
        ),
    )
    .await?;

    // List with both Metadata and Tags includes
    let page = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            include: Some(vec![
                ListBlobsIncludeItem::Metadata,
                ListBlobsIncludeItem::Tags,
            ]),
            ..Default::default()
        }))?
        .into_pages()
        .try_next()
        .await?
        .unwrap()
        .into_model()?;

    let items = page.segment.blob_items;

    // Metadata blob: metadata should be populated
    let meta_blob = items
        .iter()
        .find(|b| b.name.as_deref() == Some(metadata_blob_name.as_str()))
        .expect("expected metadata blob in listing");
    let blob_meta = meta_blob
        .metadata
        .as_ref()
        .expect("metadata should be populated");
    assert_eq!(Some(&metadata), blob_meta.additional_properties.as_ref());

    // Tags blob: blob_tags should be populated
    let tags_blob = items
        .iter()
        .find(|b| b.name.as_deref() == Some(tags_blob_name.as_str()))
        .expect("expected tags blob in listing");
    assert!(
        tags_blob.blob_tags.is_some(),
        "expected blob_tags to be populated with Tags include"
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_list_blobs_with_prefix(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create(None).await?;

    let prefix = "aa-";
    let blob_with_prefix = format!("{}{}", prefix, get_blob_name(recording));
    let blob_no_prefix = format!("zz-{}", get_blob_name(recording));

    create_test_blob(&container_client.blob_client(&blob_with_prefix), None, None).await?;
    create_test_blob(&container_client.blob_client(&blob_no_prefix), None, None).await?;

    let page = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            prefix: Some(prefix.to_string()),
            ..Default::default()
        }))?
        .into_pages()
        .try_next()
        .await?
        .unwrap()
        .into_model()?;

    let names: Vec<String> = page
        .segment
        .blob_items
        .into_iter()
        .filter_map(|b| b.name)
        .collect();
    assert_eq!(1, names.len());
    assert_eq!(blob_with_prefix, names[0]);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_list_blobs_with_uncommitted_blobs_include(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create(None).await?;

    // Stage a block without committing - creates an uncommitted blob entry
    let blob_name = get_blob_name(recording);
    let block_blob_client = container_client.blob_client(&blob_name).block_blob_client();
    let block_id: Vec<u8> = b"block1".to_vec();
    block_blob_client
        .stage_block(&block_id, 5, RequestContent::from(b"hello".to_vec()), None)
        .await?;

    // Without UncommittedBlobs Include Scenario
    let page_without = container_client
        .list_blobs(None)?
        .into_pages()
        .try_next()
        .await?
        .unwrap()
        .into_model()?;
    assert!(
        page_without
            .segment
            .blob_items
            .iter()
            .all(|b| b.name.as_deref() != Some(blob_name.as_str())),
        "uncommitted blob should not appear without UncommittedBlobs include"
    );

    // With UncommittedBlobs Include Scenario
    let page_with = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            include: Some(vec![ListBlobsIncludeItem::UncommittedBlobs]),
            ..Default::default()
        }))?
        .into_pages()
        .try_next()
        .await?
        .unwrap()
        .into_model()?;
    assert!(
        page_with
            .segment
            .blob_items
            .iter()
            .any(|b| b.name.as_deref() == Some(blob_name.as_str())),
        "uncommitted blob should appear with UncommittedBlobs include"
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_list_blobs_with_deleted_include(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // TODO: requires an account with blob soft-delete enabled (set via Set Blob Service Properties,
    // deleteRetentionPolicy.enabled = true). Record this test against such an account.

    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create(None).await?;

    let blob_name = get_blob_name(recording);
    let blob_client = container_client.blob_client(&blob_name);
    create_test_blob(&blob_client, None, None).await?;

    // Soft-delete the blob
    blob_client.delete(None).await?;

    // Without Deleted Include Scenario
    let page_without = container_client
        .list_blobs(None)?
        .into_pages()
        .try_next()
        .await?
        .unwrap()
        .into_model()?;
    assert!(
        page_without
            .segment
            .blob_items
            .iter()
            .all(|b| b.name.as_deref() != Some(blob_name.as_str())),
        "deleted blob should not appear without Deleted include"
    );

    // With Deleted Include Scenario
    let page_with = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            include: Some(vec![ListBlobsIncludeItem::Deleted]),
            ..Default::default()
        }))?
        .into_pages()
        .try_next()
        .await?
        .unwrap()
        .into_model()?;
    let deleted_blob = page_with
        .segment
        .blob_items
        .into_iter()
        .find(|b| b.name.as_deref() == Some(blob_name.as_str()))
        .expect("soft-deleted blob should appear with Deleted include");
    assert!(
        deleted_blob.deleted.unwrap_or(false),
        "blob should be marked as deleted"
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_list_blobs_with_copy_include(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;
    container_client.create(None).await?;

    // Create source blob and copy it to a destination
    let source_name = get_blob_name(recording);
    let dest_name = get_blob_name(recording);
    let source_blob_client = container_client.blob_client(&source_name);
    create_test_blob(&source_blob_client, None, None).await?;

    let dest_blob_client = container_client.blob_client(&dest_name);
    dest_blob_client
        .block_blob_client()
        .upload_blob_from_url(source_blob_client.url().as_str().into(), None)
        .await?;

    // Copy Include Scenario
    let page = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            include: Some(vec![ListBlobsIncludeItem::Copy]),
            ..Default::default()
        }))?
        .into_pages()
        .try_next()
        .await?
        .unwrap()
        .into_model()?;

    // Assert
    // Note: copy_status/copy_id/copy_source are only populated for async Copy Blob
    // operations, not synchronous Put Blob From URL. The Copy include flag is
    // accepted and the destination blob still appears in the listing.
    let dest_blob = page
        .segment
        .blob_items
        .into_iter()
        .find(|b| b.name.as_deref() == Some(dest_name.as_str()))
        .expect("destination blob should appear in listing");
    assert!(
        dest_blob.properties.is_some(),
        "dest blob should have properties"
    );

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
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
#[ignore = "need to investigate live test pipeline failures"]
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
#[ignore = "need to investigate live test pipeline failures"]
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

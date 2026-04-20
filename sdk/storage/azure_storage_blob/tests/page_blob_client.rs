// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::http::{headers::CONTENT_TYPE, RequestContent, StatusCode};
use azure_core_test::{recorded, TestContext};
use azure_storage_blob::models::{
    BlobClientCreateSnapshotResultHeaders, BlobClientGetPropertiesResultHeaders, BlobType,
    HttpRange, PageBlobClientCreateOptions, PageBlobClientGetPageRangesOptions,
    PageBlobClientSetSequenceNumberOptions, PageBlobClientSetSequenceNumberResultHeaders,
    PageBlobClientUploadPagesFromUrlOptions, PageBlobClientUploadPagesOptions,
    SequenceNumberActionType,
};
use azure_storage_blob_test::{get_blob_name, get_container_client, StorageAccount};
use std::{collections::HashMap, error::Error};

#[recorded::test]
async fn test_create_page_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup

    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();

    // Regular Create Scenario
    page_blob_client.create(1024, None).await?;
    // Assert
    let blob_properties = blob_client.get_properties(None).await?;
    let blob_type = blob_properties.blob_type()?;
    let content_length = blob_properties.content_length()?;
    assert_eq!(1024, content_length.unwrap());
    assert_eq!(BlobType::PageBlob, blob_type.unwrap());

    // Create If Not Exists Scenario
    let create_options = PageBlobClientCreateOptions::default().with_if_not_exists();
    let response = page_blob_client
        .create(1024, Some(create_options.clone()))
        .await;
    // Assert
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::Conflict, error.unwrap());

    blob_client.delete(None).await?;
    page_blob_client.create(1024, Some(create_options)).await?;
    // Assert
    let blob_properties = blob_client.get_properties(None).await?;
    let blob_type = blob_properties.blob_type()?;
    let content_length = blob_properties.content_length()?;
    assert_eq!(1024, content_length.unwrap());
    assert_eq!(BlobType::PageBlob, blob_type.unwrap());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_upload_page(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();
    page_blob_client.create(512, None).await?;
    let data = vec![b'A'; 512];
    page_blob_client
        .upload_pages(
            RequestContent::from(data.clone()),
            512,
            HttpRange::new(0, 512).to_string(),
            None,
        )
        .await?;

    // Assert
    let response = blob_client.download(None).await?;
    assert_eq!(512, response.properties.content_length.unwrap());
    let body_data = response.body.collect().await?;
    assert_eq!(data, body_data);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_clear_page(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();
    page_blob_client.create(512, None).await?;
    let data = vec![b'A'; 512];
    page_blob_client
        .upload_pages(
            RequestContent::from(data),
            512,
            HttpRange::new(0, 512).to_string(),
            None,
        )
        .await?;

    page_blob_client
        .clear_pages(HttpRange::new(0, 512).to_string(), None)
        .await?;

    // Assert
    let response = blob_client.download(None).await?;
    assert_eq!(512, response.properties.content_length.unwrap());
    let body_data = response.body.collect().await?;
    assert_eq!(vec![0; 512], body_data);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_resize_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();

    // Blob Too Small Scenario
    page_blob_client.create(512, None).await?;
    let data = vec![b'A'; 1024];
    let response = page_blob_client
        .upload_pages(
            RequestContent::from(data.clone()),
            1024,
            HttpRange::new(0, 1024).to_string(),
            None,
        )
        .await;
    // Assert
    let error = response.unwrap_err().http_status();
    assert_eq!(StatusCode::RequestedRangeNotSatisfiable, error.unwrap());

    page_blob_client.resize(1024, None).await?;
    page_blob_client
        .upload_pages(
            RequestContent::from(data.clone()),
            1024,
            HttpRange::new(0, 1024).to_string(),
            None,
        )
        .await?;

    // Truncate Blob Scenario
    page_blob_client.resize(512, None).await?;
    // Assert
    let response = blob_client.download(None).await?;
    assert_eq!(512, response.properties.content_length.unwrap());
    let body_data = response.body.collect().await?;
    assert_eq!(vec![b'A'; 512], body_data);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_set_sequence_number(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup

    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();

    // Update Action
    page_blob_client.create(1024, None).await?;
    let sequence_number_options = PageBlobClientSetSequenceNumberOptions {
        blob_sequence_number: Some(7),
        ..Default::default()
    };
    let response = page_blob_client
        .set_sequence_number(
            SequenceNumberActionType::Update,
            Some(sequence_number_options),
        )
        .await?;
    let blob_sequence_number = response.blob_sequence_number()?;
    assert_eq!(7, blob_sequence_number.unwrap());

    // Increment Action
    let response = page_blob_client
        .set_sequence_number(SequenceNumberActionType::Increment, None)
        .await?;
    let blob_sequence_number = response.blob_sequence_number()?;
    assert_eq!(8, blob_sequence_number.unwrap());

    // Set Max Action
    let sequence_number_options = PageBlobClientSetSequenceNumberOptions {
        blob_sequence_number: Some(5),
        ..Default::default()
    };
    page_blob_client
        .set_sequence_number(SequenceNumberActionType::Max, Some(sequence_number_options))
        .await?;
    let blob_sequence_number = response.blob_sequence_number()?;
    assert_eq!(8, blob_sequence_number.unwrap());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_upload_page_from_url(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client_1 = container_client.blob_client(&get_blob_name(recording));
    let blob_client_2 = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client_1 = blob_client_1.page_blob_client();
    let page_blob_client_2 = blob_client_2.page_blob_client();

    // Act
    page_blob_client_1.create(512, None).await?;
    let data_b = vec![b'B'; 512];
    page_blob_client_1
        .upload_pages(
            RequestContent::from(data_b.clone()),
            512,
            HttpRange::new(0, 512).to_string(),
            None,
        )
        .await?;

    page_blob_client_2.create(1024, None).await?;
    let mut data_a = vec![b'A'; 512];
    page_blob_client_2
        .upload_pages(
            RequestContent::from(data_a.clone()),
            512,
            HttpRange::new(0, 512).to_string(),
            None,
        )
        .await?;
    page_blob_client_2
        .upload_pages_from_url(
            blob_client_1.url().as_str().into(),
            HttpRange::new(0, data_b.len() as u64).to_string(),
            data_b.len() as u64,
            HttpRange::new(512, data_b.len() as u64).to_string(),
            None,
        )
        .await?;

    // Assert
    let response = blob_client_2.download(None).await?;
    assert_eq!(1024, response.properties.content_length.unwrap());
    data_a.extend(&data_b);
    let body_data = response.body.collect().await?;
    assert_eq!(data_a, body_data);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
async fn test_get_page_ranges(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();
    page_blob_client.create(1024, None).await?;

    // Empty Page Range Scenario
    let get_page_ranges_response = page_blob_client.get_page_ranges(None).await?;
    // Assert
    let page_ranges = get_page_ranges_response.into_model()?;
    let page_range = page_ranges.page_range;
    assert!(page_range.is_none());

    // Non-Empty Page Range Scenario
    let data = vec![b'A'; 512];
    page_blob_client
        .upload_pages(
            RequestContent::from(data.clone()),
            512,
            HttpRange::new(0, 512).to_string(),
            None,
        )
        .await?;
    let get_page_ranges_response = page_blob_client.get_page_ranges(None).await?;
    // Assert
    let page_ranges = get_page_ranges_response.into_model()?;
    let page_range = page_ranges.page_range.unwrap();
    for range in page_range {
        assert_eq!(0, range.start.unwrap());
        assert_eq!(511, range.end.unwrap());
    }

    // Range Filter Scenario
    page_blob_client
        .upload_pages(
            RequestContent::from(vec![b'B'; 512]),
            512,
            HttpRange::new(512, 512).to_string(),
            None,
        )
        .await?;
    let response = page_blob_client
        .get_page_ranges(Some(PageBlobClientGetPageRangesOptions {
            range: Some(HttpRange::new(0, 512).to_string()),
            ..Default::default()
        }))
        .await?
        .into_model()?;
    // Assert
    let page_range = response.page_range.unwrap();
    assert_eq!(1, page_range.len());
    assert_eq!(0, page_range[0].start.unwrap());
    assert_eq!(511, page_range[0].end.unwrap());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_create_page_blob_content_headers(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();

    // Create with Content Headers
    // Note: blob_content_md5 is validated against actual content on create and is excluded
    // here; it is tested as stored metadata via set_properties in blob_client tests.
    // Use a single cache-control directive to avoid service-side reordering.
    page_blob_client
        .create(
            512,
            Some(PageBlobClientCreateOptions {
                blob_cache_control: Some("no-cache".to_string()),
                blob_content_disposition: Some("inline".to_string()),
                blob_content_encoding: Some("identity".to_string()),
                blob_content_language: Some("ja-JP".to_string()),
                blob_content_type: Some("image/png".to_string()),
                ..Default::default()
            }),
        )
        .await?;

    // Assert Content Headers Roundtrip
    let props = blob_client.get_properties(None).await?;
    assert_eq!(Some("no-cache".to_string()), props.cache_control()?);
    assert_eq!(Some("inline".to_string()), props.content_disposition()?);
    assert_eq!(Some("identity".to_string()), props.content_encoding()?);
    assert_eq!(Some("ja-JP".to_string()), props.content_language()?);
    let content_type: Option<String> = props.headers().get_optional_as(&CONTENT_TYPE)?;
    assert_eq!(Some("image/png".to_string()), content_type);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_upload_pages_sequence_number_condition(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();

    // Create blob with sequence number 5
    page_blob_client
        .create(
            512,
            Some(PageBlobClientCreateOptions {
                blob_sequence_number: Some(5),
                ..Default::default()
            }),
        )
        .await?;
    let data = vec![b'A'; 512];

    // Sequence Number Equal To Mismatch Scenario
    let response = page_blob_client
        .upload_pages(
            RequestContent::from(data.clone()),
            512,
            HttpRange::new(0, 512).to_string(),
            Some(PageBlobClientUploadPagesOptions {
                if_sequence_number_equal_to: Some(3),
                ..Default::default()
            }),
        )
        .await;

    // Assert
    assert_eq!(
        StatusCode::PreconditionFailed,
        response.unwrap_err().http_status().unwrap()
    );

    // Sequence Number Equal To Match Scenario
    page_blob_client
        .upload_pages(
            RequestContent::from(data.clone()),
            512,
            HttpRange::new(0, 512).to_string(),
            Some(PageBlobClientUploadPagesOptions {
                if_sequence_number_equal_to: Some(5),
                ..Default::default()
            }),
        )
        .await?;

    // Sequence Number Less Than Mismatch Scenario (seq=5, lt=3 fails)
    let response = page_blob_client
        .upload_pages(
            RequestContent::from(data.clone()),
            512,
            HttpRange::new(0, 512).to_string(),
            Some(PageBlobClientUploadPagesOptions {
                if_sequence_number_less_than: Some(3),
                ..Default::default()
            }),
        )
        .await;

    // Assert
    assert_eq!(
        StatusCode::PreconditionFailed,
        response.unwrap_err().http_status().unwrap()
    );

    // Sequence Number Less Than Match Scenario (seq=5, lt=6 succeeds)
    page_blob_client
        .upload_pages(
            RequestContent::from(data.clone()),
            512,
            HttpRange::new(0, 512).to_string(),
            Some(PageBlobClientUploadPagesOptions {
                if_sequence_number_less_than: Some(6),
                ..Default::default()
            }),
        )
        .await?;

    // Sequence Number Less Than Or Equal To Mismatch Scenario (seq=5, lte=3 fails)
    let response = page_blob_client
        .upload_pages(
            RequestContent::from(data.clone()),
            512,
            HttpRange::new(0, 512).to_string(),
            Some(PageBlobClientUploadPagesOptions {
                if_sequence_number_less_than_or_equal_to: Some(3),
                ..Default::default()
            }),
        )
        .await;

    // Assert
    assert_eq!(
        StatusCode::PreconditionFailed,
        response.unwrap_err().http_status().unwrap()
    );

    // Sequence Number Less Than Or Equal To Match Scenario
    page_blob_client
        .upload_pages(
            RequestContent::from(data.clone()),
            512,
            HttpRange::new(0, 512).to_string(),
            Some(PageBlobClientUploadPagesOptions {
                if_sequence_number_less_than_or_equal_to: Some(5),
                ..Default::default()
            }),
        )
        .await?;

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_get_page_ranges_snapshot(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();

    // Create and fill a page blob, then snapshot it
    page_blob_client.create(512, None).await?;
    page_blob_client
        .upload_pages(
            RequestContent::from(vec![b'A'; 512]),
            512,
            HttpRange::new(0, 512).to_string(),
            None,
        )
        .await?;
    let snapshot_id = blob_client
        .create_snapshot(None)
        .await?
        .snapshot()?
        .unwrap();

    // Snapshot Query Scenario
    let response = page_blob_client
        .get_page_ranges(Some(PageBlobClientGetPageRangesOptions {
            snapshot: Some(snapshot_id),
            ..Default::default()
        }))
        .await?
        .into_model()?;

    // Assert
    let page_range = response.page_range.unwrap();
    assert_eq!(1, page_range.len());
    assert_eq!(0, page_range[0].start.unwrap());
    assert_eq!(511, page_range[0].end.unwrap());

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_upload_pages_transactional_checksums(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();
    page_blob_client.create(512, None).await?;

    let data = vec![b'A'; 512];
    // MD5(512 × b'A') well-known vector
    let correct_md5: Vec<u8> = vec![
        220, 80, 134, 184, 71, 40, 155, 168, 184, 189, 225, 73, 184, 56, 129, 117,
    ];
    // CRC64-ECMA-182 of 512 × b'A', server-confirmed (base64: twYjY3c/3gM=)
    let correct_crc64: Vec<u8> = vec![183, 6, 35, 99, 119, 63, 222, 3];

    // MD5 Mismatch Scenario
    let response = page_blob_client
        .upload_pages(
            RequestContent::from(data.clone()),
            512,
            HttpRange::new(0, 512).to_string(),
            Some(PageBlobClientUploadPagesOptions {
                transactional_content_md5: Some(vec![0u8; 16]),
                ..Default::default()
            }),
        )
        .await;
    assert_eq!(
        StatusCode::BadRequest,
        response.unwrap_err().http_status().unwrap()
    );

    // MD5 Match Scenario
    page_blob_client
        .upload_pages(
            RequestContent::from(data.clone()),
            512,
            HttpRange::new(0, 512).to_string(),
            Some(PageBlobClientUploadPagesOptions {
                transactional_content_md5: Some(correct_md5),
                ..Default::default()
            }),
        )
        .await?;

    // CRC64 Mismatch Scenario
    let response = page_blob_client
        .upload_pages(
            RequestContent::from(data.clone()),
            512,
            HttpRange::new(0, 512).to_string(),
            Some(PageBlobClientUploadPagesOptions {
                transactional_content_crc64: Some(vec![0u8; 8]),
                ..Default::default()
            }),
        )
        .await;
    assert_eq!(
        StatusCode::BadRequest,
        response.unwrap_err().http_status().unwrap()
    );

    // CRC64 Match Scenario
    page_blob_client
        .upload_pages(
            RequestContent::from(data.clone()),
            512,
            HttpRange::new(0, 512).to_string(),
            Some(PageBlobClientUploadPagesOptions {
                transactional_content_crc64: Some(correct_crc64),
                ..Default::default()
            }),
        )
        .await?;

    // No Checksum Scenario
    page_blob_client
        .upload_pages(
            RequestContent::from(data),
            512,
            HttpRange::new(0, 512).to_string(),
            None,
        )
        .await?;

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_create_page_blob_with_tags(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let blob_client = container_client.blob_client(&get_blob_name(recording));
    let page_blob_client = blob_client.page_blob_client();

    let expected = HashMap::from([("env".to_string(), "test".to_string())]);
    page_blob_client
        .create(
            512,
            Some(PageBlobClientCreateOptions {
                blob_tags_string: Some("env=test".to_string()),
                ..Default::default()
            }),
        )
        .await?;

    // Assert
    let map: HashMap<String, String> = blob_client.get_tags(None).await?.into_model()?.into();
    assert_eq!(expected, map);

    container_client.delete(None).await?;
    Ok(())
}

#[recorded::test]
#[ignore = "need to investigate live test pipeline failures"]
async fn test_upload_pages_from_url_source_if_match(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let container_client =
        get_container_client(recording, true, StorageAccount::Standard, None).await?;
    let source_blob_client = container_client.blob_client(&get_blob_name(recording));
    let source_page_blob = source_blob_client.page_blob_client();

    // Create source page blob with content
    source_page_blob.create(512, None).await?;
    source_page_blob
        .upload_pages(
            RequestContent::from(vec![b'S'; 512]),
            512,
            HttpRange::new(0, 512).to_string(),
            None,
        )
        .await?;
    let etag = source_blob_client
        .get_properties(None)
        .await?
        .etag()?
        .unwrap()
        .to_string();

    let dest_blob_client = container_client.blob_client(&get_blob_name(recording));
    let dest_page_blob = dest_blob_client.page_blob_client();
    dest_page_blob.create(512, None).await?;

    // Source If-Match Scenario
    dest_page_blob
        .upload_pages_from_url(
            source_blob_client.url().as_str().into(),
            HttpRange::new(0, 512).to_string(),
            512,
            HttpRange::new(0, 512).to_string(),
            Some(PageBlobClientUploadPagesFromUrlOptions {
                source_if_match: Some(etag.clone().into()),
                ..Default::default()
            }),
        )
        .await?;

    // Source If-None-Match Scenario (ETag matches, so condition is not satisfied)
    let response = dest_page_blob
        .upload_pages_from_url(
            source_blob_client.url().as_str().into(),
            HttpRange::new(0, 512).to_string(),
            512,
            HttpRange::new(0, 512).to_string(),
            Some(PageBlobClientUploadPagesFromUrlOptions {
                source_if_none_match: Some(etag.into()),
                ..Default::default()
            }),
        )
        .await;

    // Assert
    assert_eq!(
        StatusCode::NotModified,
        response.unwrap_err().http_status().unwrap()
    );

    container_client.delete(None).await?;
    Ok(())
}

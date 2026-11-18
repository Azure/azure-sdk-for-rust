// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod common;

use azure_core::http::StatusCode;
use azure_core_test::{recorded, TestContext, VarOptions};
use azure_storage_blob::{
    models::{
        BlobClientDownloadOptions, BlobClientGetPropertiesResultHeaders,
        BlobContainerClientCreateOptions, BlobContainerClientGetPropertiesResultHeaders,
        BlobContainerClientListBlobsOptions, BlobServiceClientListContainersOptions,
        StorageErrorCode,
    },
    StorageError,
};
use common::{get_blob_service_client, get_container_client, get_container_name, StorageAccount};
use futures::TryStreamExt;
use std::error::Error;

const EXPECTED_CONTENT: &[u8] = b"is an awesome feature!";

// Creates access-point and ordinary containers, then gets and lists their properties.
// Expected: matching access-point bindings, absent ordinary-container fields, and paginated results.
#[recorded::test]
async fn blob_access_point_create_and_verify_container_properties(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let configuration_name = recording.var("BLOB_ACCESS_POINT_CONFIGURATION_NAME", None);
    let service_client = get_blob_service_client(recording, StorageAccount::Standard, None)?;

    // Get Existing Configuration Binding
    let fixture_container_client = service_client
        .blob_container_client(&recording.var("BLOB_ACCESS_POINT_CONTAINER_NAME", None));
    let container_properties = fixture_container_client.get_properties(None).await?;
    assert_eq!(
        container_properties.is_blob_access_point_container()?,
        Some(true)
    );
    assert_eq!(
        container_properties.blob_access_point_configuration_name()?,
        Some(configuration_name.clone())
    );
    let configuration_unique_id = container_properties
        .blob_access_point_configuration_unique_id()?
        .expect("fixture configuration unique ID");
    assert!(!configuration_unique_id.is_empty());
    let prefix = get_container_name(recording);
    let container_names = [format!("{prefix}-id"), format!("{prefix}-ordinary")];
    let mut created_container_names = Vec::new();

    // Keep cleanup outside the assertions so failed assertions also remove test-owned containers.
    let outcome = std::panic::AssertUnwindSafe(async {
        // Create Access Point Container
        let container_client = service_client.blob_container_client(&container_names[0]);
        container_client
            .create(Some(BlobContainerClientCreateOptions {
                blob_access_point_configuration_name: Some(configuration_name.clone()),
                blob_access_point_configuration_unique_id: Some(configuration_unique_id.clone()),
                ..Default::default()
            }))
            .await?;
        created_container_names.push(container_names[0].clone());
        let container_properties = container_client.get_properties(None).await?;

        // Assert
        assert_eq!(
            container_properties.is_blob_access_point_container()?,
            Some(true)
        );
        assert_eq!(
            container_properties.blob_access_point_configuration_name()?,
            Some(configuration_name.clone())
        );
        assert_eq!(
            container_properties.blob_access_point_configuration_unique_id()?,
            Some(configuration_unique_id.clone())
        );

        // Create Ordinary Container
        let ordinary_container_client = service_client.blob_container_client(&container_names[1]);
        ordinary_container_client.create(None).await?;
        created_container_names.push(container_names[1].clone());
        let container_properties = ordinary_container_client.get_properties(None).await?;

        // Assert
        assert_eq!(container_properties.is_blob_access_point_container()?, None);
        assert_eq!(
            container_properties.blob_access_point_configuration_name()?,
            None
        );
        assert_eq!(
            container_properties.blob_access_point_configuration_unique_id()?,
            None
        );

        // List Containers With Continuation
        let mut pager_response = service_client
            .list_containers(Some(BlobServiceClientListContainersOptions {
                prefix: Some(prefix),
                maxresults: Some(1),
                ..Default::default()
            }))?
            .into_pages();
        let mut containers_found = [false; 2];
        let mut page_count = 0;
        while let Some(page) = pager_response.try_next().await? {
            page_count += 1;
            for container_item in page.into_model()?.container_items {
                let container_name = container_item.name.expect("container name");
                let container_index = container_names
                    .iter()
                    .position(|expected| *expected == container_name)
                    .expect("only this test's containers should match the prefix");
                assert!(
                    !containers_found[container_index],
                    "duplicate container: {container_name}"
                );
                containers_found[container_index] = true;
                let container_properties = container_item.properties.expect("container properties");
                if container_index == 1 {
                    assert_eq!(container_properties.is_blob_access_point_container, None);
                    assert_eq!(
                        container_properties.blob_access_point_configuration_name,
                        None
                    );
                    assert_eq!(
                        container_properties.blob_access_point_configuration_unique_id,
                        None
                    );
                } else {
                    assert_eq!(
                        container_properties.is_blob_access_point_container,
                        Some(true)
                    );
                    assert_eq!(
                        container_properties
                            .blob_access_point_configuration_name
                            .as_deref(),
                        Some(configuration_name.as_str())
                    );
                    assert_eq!(
                        container_properties
                            .blob_access_point_configuration_unique_id
                            .as_deref(),
                        Some(configuration_unique_id.as_str())
                    );
                }
            }
        }

        // Assert
        assert_eq!(containers_found, [true; 2]);
        assert!(page_count >= 2, "the test must exercise continuation");
        Ok::<(), Box<dyn Error>>(())
    });
    let outcome = futures::FutureExt::catch_unwind(outcome).await;

    // Cleanup
    let mut cleanup_error = None;
    for container_name in created_container_names {
        let container_client = service_client.blob_container_client(&container_name);
        if let Err(error) = container_client.delete(None).await {
            eprintln!(
                "Failed to delete test container {}: {error}",
                container_client.url()
            );
            cleanup_error = Some(error);
        }
    }
    match outcome {
        Ok(result) => result?,
        Err(panic) => std::panic::resume_unwind(panic),
    }
    if let Some(error) = cleanup_error {
        return Err(error.into());
    }
    Ok(())
}

// Creates a new access-point container using only an existing configuration name.
// Expected: creation succeeds and the service resolves the configuration unique ID.
// Spec Discrepancy: a live run instead returned HTTP 400 MissingRequiredHeader for x-ms-blob-access-point-configuration-unique-id.
#[recorded::test]
#[ignore = "name-only creation differs from the spec; awaiting confirmation from spec owners"]
async fn blob_access_point_create_name_only(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let configuration_name = recording.var("BLOB_ACCESS_POINT_CONFIGURATION_NAME", None);
    let container_client =
        get_container_client(recording, false, StorageAccount::Standard, None).await?;

    // Create Container With Configuration Name Only
    container_client
        .create(Some(BlobContainerClientCreateOptions {
            blob_access_point_configuration_name: Some(configuration_name.clone()),
            ..Default::default()
        }))
        .await?;
    let container_properties = container_client.get_properties(None).await;
    container_client.delete(None).await?;
    let container_properties = container_properties?;

    // Assert
    assert_eq!(
        container_properties.is_blob_access_point_container()?,
        Some(true)
    );
    assert_eq!(
        container_properties.blob_access_point_configuration_name()?,
        Some(configuration_name)
    );
    assert!(!container_properties
        .blob_access_point_configuration_unique_id()?
        .expect("the service must resolve the configuration unique ID")
        .is_empty());
    Ok(())
}

// Lists and reads the reference blob through an existing access-point container.
// Expected: matching length, ETag, and last-modified time, a populated tier, and exact full and ranged content.
#[recorded::test]
async fn blob_access_point_list_and_download_blob(ctx: TestContext) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let configuration_name = recording.var("BLOB_ACCESS_POINT_CONFIGURATION_NAME", None);
    let container_name = recording.var("BLOB_ACCESS_POINT_CONTAINER_NAME", None);
    let blob_name = recording.var(
        "BLOB_ACCESS_POINT_BLOB_NAME",
        Some(VarOptions {
            default_value: Some("blob_access_point.txt".into()),
            ..Default::default()
        }),
    );
    let service_client = get_blob_service_client(recording, StorageAccount::Standard, None)?;
    let container_client = service_client.blob_container_client(&container_name);

    // Get Container Properties
    let container_properties = container_client.get_properties(None).await?;
    assert_eq!(
        container_properties.is_blob_access_point_container()?,
        Some(true)
    );
    assert_eq!(
        container_properties.blob_access_point_configuration_name()?,
        Some(configuration_name)
    );
    assert!(!container_properties
        .blob_access_point_configuration_unique_id()?
        .expect("configuration unique ID")
        .is_empty());

    // Get Blob Properties
    let blob_client = container_client.blob_client(&blob_name);
    let blob_properties = blob_client.get_properties(None).await?;
    assert_eq!(
        blob_properties.content_length()?,
        Some(EXPECTED_CONTENT.len() as u64)
    );
    let etag = blob_properties.etag()?.expect("blob ETag");
    let last_modified = blob_properties
        .last_modified()?
        .expect("blob last-modified time");

    // List Blobs
    let mut pager_response = container_client
        .list_blobs(Some(BlobContainerClientListBlobsOptions {
            prefix: Some(blob_name.clone()),
            ..Default::default()
        }))?
        .into_pages();
    let mut blob_found = false;
    while let Some(page) = pager_response.try_next().await? {
        for blob_item in page.into_model()?.blob_items {
            if blob_item.name.as_deref() == Some(blob_name.as_str()) {
                assert!(!blob_found, "duplicate fixture blob");
                blob_found = true;
                let listed_properties = blob_item.properties.expect("listed blob properties");
                assert_eq!(
                    listed_properties.content_length,
                    Some(EXPECTED_CONTENT.len() as u64)
                );
                let listed_etag = listed_properties.etag.expect("listed blob ETag");
                // List and Get Properties expose different surrounding ETag quoting.
                assert_eq!(
                    listed_etag.as_ref().trim_matches('"'),
                    etag.as_ref().trim_matches('"')
                );
                assert_eq!(listed_properties.last_modified, Some(last_modified));
                assert!(
                    listed_properties.access_tier.is_some(),
                    "listed blob access tier"
                );
            }
        }
    }
    assert!(blob_found, "the access point must list {blob_name}");

    // Full Blob Download
    let download_response = blob_client.download(None).await?;
    assert_eq!(
        download_response.properties.content_length,
        Some(EXPECTED_CONTENT.len() as u64)
    );
    assert_eq!(
        &download_response.body.collect().await?[..],
        EXPECTED_CONTENT
    );

    // Range Blob Download
    let download_response = blob_client
        .download(Some(BlobClientDownloadOptions {
            range: Some((3..13u64).into()),
            ..Default::default()
        }))
        .await?;
    assert_eq!(
        &download_response.body.collect().await?[..],
        &EXPECTED_CONTENT[3..13]
    );
    Ok(())
}

// Creates a container using a nonexistent configuration name and a valid configuration ID.
// Expected: HTTP 409 with BlobAccessPointConfigurationNameNotFound.
#[recorded::test]
async fn blob_access_point_create_with_nonexistent_configuration_name(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let unknown_configuration_name = get_container_name(recording);
    let service_client = get_blob_service_client(recording, StorageAccount::Standard, None)?;
    let container_client = service_client
        .blob_container_client(&recording.var("BLOB_ACCESS_POINT_CONTAINER_NAME", None));
    let container_properties = container_client.get_properties(None).await?;
    assert_eq!(
        container_properties.is_blob_access_point_container()?,
        Some(true)
    );
    let configuration_unique_id = container_properties
        .blob_access_point_configuration_unique_id()?
        .expect("fixture configuration unique ID");
    assert!(!configuration_unique_id.is_empty());

    // Supply a valid ID so missing-header validation does not mask unknown-name handling.
    let storage_error = create_with_invalid_binding(
        ctx,
        Some(unknown_configuration_name),
        Some(configuration_unique_id),
    )
    .await?;

    // Assert
    assert_eq!(storage_error.status_code, StatusCode::Conflict);
    assert_eq!(
        storage_error.error_code,
        Some(StorageErrorCode::BlobAccessPointConfigurationNameNotFound)
    );
    Ok(())
}

// Creates a container using a valid configuration ID without a configuration name.
// Expected: HTTP 400 with MissingRequiredHeader identifying the configuration-name header.
#[recorded::test]
async fn blob_access_point_create_without_configuration_name(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let service_client = get_blob_service_client(recording, StorageAccount::Standard, None)?;
    let container_client = service_client
        .blob_container_client(&recording.var("BLOB_ACCESS_POINT_CONTAINER_NAME", None));
    let configuration_unique_id = container_client
        .get_properties(None)
        .await?
        .blob_access_point_configuration_unique_id()?
        .expect("fixture configuration unique ID");
    assert!(!configuration_unique_id.is_empty());

    // Create Container Without Configuration Name
    let storage_error =
        create_with_invalid_binding(ctx, None, Some(configuration_unique_id)).await?;

    // Assert
    assert_eq!(storage_error.status_code, StatusCode::BadRequest);
    assert_eq!(
        storage_error.error_code,
        Some(StorageErrorCode::MissingRequiredHeader)
    );
    assert_eq!(
        storage_error
            .additional_error_info
            .get("HeaderName")
            .map(String::as_str),
        Some("x-ms-blob-access-point-configuration-name")
    );
    Ok(())
}

// Creates a container using a configuration name and a valid ID from another configuration.
// Expected: HTTP 409 with BlobAccessPointConfigurationUniqueIdMismatch.
#[recorded::test]
async fn blob_access_point_create_with_mismatched_configuration_id(
    ctx: TestContext,
) -> Result<(), Box<dyn Error>> {
    // Recording Setup
    let recording = ctx.recording();
    let configuration_name = recording.var("BLOB_ACCESS_POINT_CONFIGURATION_NAME", None);
    let container_name = recording.var("BLOB_ACCESS_POINT_CONTAINER_NAME", None);
    let other_container_name = recording.var("BLOB_ACCESS_POINT_OTHER_CONTAINER_NAME", None);
    assert_ne!(
        container_name, other_container_name,
        "the test requires two different fixture containers"
    );
    let service_client = get_blob_service_client(recording, StorageAccount::Standard, None)?;

    // Get Primary Configuration Binding
    let container_client = service_client.blob_container_client(&container_name);
    let container_properties = container_client.get_properties(None).await?;
    assert_eq!(
        container_properties.is_blob_access_point_container()?,
        Some(true)
    );
    assert_eq!(
        container_properties.blob_access_point_configuration_name()?,
        Some(configuration_name.clone()),
        "the primary container must be bound to the supplied configuration name"
    );
    let configuration_unique_id = container_properties
        .blob_access_point_configuration_unique_id()?
        .expect("primary configuration unique ID");
    assert!(!configuration_unique_id.is_empty());

    // Get Other Configuration Binding
    let other_container_client = service_client.blob_container_client(&other_container_name);
    let other_container_properties = other_container_client.get_properties(None).await?;
    assert_eq!(
        other_container_properties.is_blob_access_point_container()?,
        Some(true)
    );
    let other_configuration_name = other_container_properties
        .blob_access_point_configuration_name()?
        .expect("other configuration name");
    let other_configuration_unique_id = other_container_properties
        .blob_access_point_configuration_unique_id()?
        .expect("other configuration unique ID");
    assert!(!other_configuration_name.is_empty());
    assert!(!other_configuration_unique_id.is_empty());
    assert_ne!(
        configuration_name, other_configuration_name,
        "fixture configuration names must differ"
    );
    assert_ne!(
        configuration_unique_id, other_configuration_unique_id,
        "fixture configuration unique IDs must differ"
    );

    // Create Container With Mismatched Configuration ID
    let storage_error = create_with_invalid_binding(
        ctx,
        Some(configuration_name),
        Some(other_configuration_unique_id),
    )
    .await?;

    // Assert
    assert_eq!(storage_error.status_code, StatusCode::Conflict);
    assert_eq!(
        storage_error.error_code,
        Some(StorageErrorCode::BlobAccessPointConfigurationUniqueIdMismatch)
    );
    Ok(())
}

async fn create_with_invalid_binding(
    ctx: TestContext,
    configuration_name: Option<String>,
    configuration_unique_id: Option<String>,
) -> Result<StorageError, Box<dyn Error>> {
    let container_client =
        get_container_client(ctx.recording(), false, StorageAccount::Standard, None).await?;
    let response = container_client
        .create(Some(BlobContainerClientCreateOptions {
            blob_access_point_configuration_name: configuration_name,
            blob_access_point_configuration_unique_id: configuration_unique_id,
            ..Default::default()
        }))
        .await;
    let error = match response {
        Ok(_) => {
            container_client.delete(None).await?;
            panic!("invalid binding unexpectedly succeeded");
        }
        Err(error) => error,
    };
    Ok(error.try_into()?)
}

#[cfg(test)]
mod tests {
    use super::{BlobContainerClientCreateOptions, BlobContainerClientGetPropertiesResultHeaders};
    use azure_core::{
        http::{
            headers::{HeaderName, Headers},
            AsyncRawResponse, ClientOptions, Method, StatusCode, Transport, Url,
        },
        Bytes,
    };
    use azure_core_test::http::MockHttpClient;
    use azure_storage_blob::{BlobContainerClient, BlobContainerClientOptions};
    use futures::FutureExt;
    use std::{error::Error, sync::Arc};

    const CONFIGURATION_NAME: HeaderName =
        HeaderName::from_static("x-ms-blob-access-point-configuration-name");
    const CONFIGURATION_ID: HeaderName =
        HeaderName::from_static("x-ms-blob-access-point-configuration-unique-id");

    // Exercises create-request headers with neither option, name only, ID only, and both options.
    // Expected: configuration headers are sent only when their corresponding options are supplied.
    #[tokio::test]
    async fn wire_create_headers() -> Result<(), Box<dyn Error>> {
        for (configuration_name, configuration_unique_id) in [
            (None, None),
            (Some("configuration"), None),
            (Some("configuration"), Some("configuration-id")),
            (None, Some("configuration-id")),
        ] {
            let mock_client = Arc::new(MockHttpClient::new(move |request| {
                assert_eq!(request.method(), Method::Put);
                assert_eq!(request.url().path(), "/container");
                assert_eq!(request.url().query(), Some("restype=container"));
                assert_eq!(
                    request.headers().get_optional_str(&CONFIGURATION_NAME),
                    configuration_name
                );
                assert_eq!(
                    request.headers().get_optional_str(&CONFIGURATION_ID),
                    configuration_unique_id
                );
                async {
                    Ok(AsyncRawResponse::from_bytes(
                        StatusCode::Created,
                        Headers::new(),
                        Bytes::new(),
                    ))
                }
                .boxed()
            }));
            let container_client = BlobContainerClient::new(
                Url::parse("https://example.blob.core.windows.net/container")?,
                None,
                Some(BlobContainerClientOptions {
                    client_options: ClientOptions {
                        transport: Some(Transport::new(mock_client)),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            )?;
            container_client
                .create(Some(BlobContainerClientCreateOptions {
                    blob_access_point_configuration_name: configuration_name.map(str::to_owned),
                    blob_access_point_configuration_unique_id: configuration_unique_id
                        .map(str::to_owned),
                    ..Default::default()
                }))
                .await?;
        }
        Ok(())
    }

    // Parses container properties with absent, true, false, and invalid access-point status headers.
    // Expected: optional values are preserved and an invalid boolean returns a parsing error.
    #[tokio::test]
    async fn wire_get_properties_headers() -> Result<(), Box<dyn Error>> {
        for access_point_status in [None, Some("true"), Some("false"), Some("invalid")] {
            let mock_client = Arc::new(MockHttpClient::new(move |request| {
                assert_eq!(request.method(), Method::Get);
                let mut headers = Headers::new();
                if let Some(access_point_status) = access_point_status {
                    headers.insert("x-ms-is-blob-access-point-container", access_point_status);
                    headers.insert(CONFIGURATION_NAME, "configuration");
                    headers.insert(CONFIGURATION_ID, "configuration-id");
                }
                async move {
                    Ok(AsyncRawResponse::from_bytes(
                        StatusCode::Ok,
                        headers,
                        Bytes::new(),
                    ))
                }
                .boxed()
            }));
            let container_client = BlobContainerClient::new(
                Url::parse("https://example.blob.core.windows.net/container")?,
                None,
                Some(BlobContainerClientOptions {
                    client_options: ClientOptions {
                        transport: Some(Transport::new(mock_client)),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            )?;
            let container_properties = container_client.get_properties(None).await?;
            if access_point_status == Some("invalid") {
                assert!(container_properties
                    .is_blob_access_point_container()
                    .is_err());
            } else {
                assert_eq!(
                    container_properties.is_blob_access_point_container()?,
                    access_point_status.map(|value| value == "true")
                );
            }
            assert_eq!(
                container_properties
                    .blob_access_point_configuration_name()?
                    .as_deref(),
                access_point_status.map(|_| "configuration")
            );
            assert_eq!(
                container_properties
                    .blob_access_point_configuration_unique_id()?
                    .as_deref(),
                access_point_status.map(|_| "configuration-id")
            );
        }
        Ok(())
    }
}

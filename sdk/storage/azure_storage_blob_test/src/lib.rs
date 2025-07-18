// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{ClientOptions, NoFormat, RequestContent, Response},
    Result,
};
use azure_core_test::Recording;
use azure_storage_blob::{
    models::{BlobTags, BlockBlobClientUploadResult},
    BlobClient, BlobContainerClient, BlobContainerClientOptions, BlobServiceClient,
    BlobServiceClientOptions,
};
use std::collections::HashMap;

/// Takes in a Recording instance and returns an instrumented options bag and endpoint.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
fn recorded_test_setup(recording: &Recording) -> (ClientOptions, String) {
    let mut client_options = ClientOptions::default();
    recording.instrument(&mut client_options);
    let endpoint = format!(
        "https://{}.blob.core.windows.net/",
        recording.var("AZURE_STORAGE_ACCOUNT_NAME", None).as_str()
    );

    (client_options, endpoint)
}

/// Takes in a Recording instance and returns a randomized blob name with prefix "blob" of length 16.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
pub fn get_blob_name(recording: &Recording) -> String {
    recording
        .random_string::<12>(Some("blob"))
        .to_ascii_lowercase()
}

/// Takes in a Recording instance and returns a randomized container name with prefix "container" of length 16.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
pub fn get_container_name(recording: &Recording) -> String {
    recording
        .random_string::<17>(Some("container"))
        .to_ascii_lowercase()
}

/// Returns an instance of a BlobServiceClient.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
pub fn get_blob_service_client(recording: &Recording) -> Result<BlobServiceClient> {
    let (options, endpoint) = recorded_test_setup(recording);
    let service_client_options = BlobServiceClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    BlobServiceClient::new(
        &endpoint,
        recording.credential(),
        Some(service_client_options),
    )
}

/// Returns an instance of a BlobContainerClient.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
/// * `create` - An optional flag to determine whether the container should also be created.
pub async fn get_container_client(
    recording: &Recording,
    create: bool,
) -> Result<BlobContainerClient> {
    let container_name = get_container_name(recording);
    let (options, endpoint) = recorded_test_setup(recording);
    let container_client_options = BlobContainerClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    let container_client = BlobContainerClient::new(
        &endpoint,
        container_name,
        recording.credential(),
        Some(container_client_options),
    )?;
    if create {
        container_client.create_container(None).await?;
    }
    Ok(container_client)
}

/// Creates a test blob with no options, containing the data "b'hello rusty world'" with content length 17.
///
/// # Arguments
///
/// * `blob_client` - A reference to a BlobClient instance.
pub async fn create_test_blob(
    blob_client: &BlobClient,
) -> Result<Response<BlockBlobClientUploadResult, NoFormat>> {
    blob_client
        .upload(
            RequestContent::from(b"hello rusty world".to_vec()),
            true,
            17,
            None,
        )
        .await
}

/// Takes in two separate BlobTags instances and compares their contents to check for equality.
///
/// # Arguments
///
/// * `tags1` - The first BlobTags to be compared.
/// * `tags2` - The second BlobTags to be compared.
pub fn test_blob_tag_equality(tags1: BlobTags, tags2: BlobTags) -> bool {
    let mut count_map = HashMap::new();
    // Iterate through first set of tags, populate HashMap
    for blob_tag in tags1.blob_tag_set.unwrap() {
        count_map.insert(blob_tag.key.unwrap(), blob_tag.value.unwrap());
    }
    // Iterate through second set of tags
    for blob_tag in tags2.blob_tag_set.unwrap() {
        // If tag is not found, return false
        if !count_map.contains_key(&blob_tag.key.clone().unwrap()) {
            return false;
        } else {
            count_map.remove(&blob_tag.key.unwrap());
        }
    }
    // Ensure HashMap has been completely consumed
    count_map.is_empty()
}

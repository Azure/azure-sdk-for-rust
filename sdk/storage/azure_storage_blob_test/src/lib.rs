// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{ClientOptions, NoFormat, RequestContent, Response},
    Bytes, Result,
};
use azure_core_test::Recording;
use azure_storage_blob::{
    models::{BlockBlobClientUploadOptions, BlockBlobClientUploadResult},
    BlobClient, BlobContainerClient, BlobContainerClientOptions, BlobServiceClient,
    BlobServiceClientOptions,
};

/// Takes in a Recording instance and returns an instrumented options bag and endpoint.
///
/// # Arguments
///
/// * `recording` - A reference to a Recording instance.
pub fn recorded_test_setup(recording: &Recording) -> (ClientOptions, String) {
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
        &container_name,
        recording.credential(),
        Some(container_client_options),
    )?;
    if create {
        container_client.create_container(None).await?;
    }
    Ok(container_client)
}

/// Creates a test blob with no options, containing the data "b'hello rusty world'" with content length 17 if no data specified.
///
/// # Arguments
///
/// * `blob_client` - A reference to a BlobClient instance.
/// * `data` - Blob content to be uploaded.
/// * `options` - Optional configuration for the upload request.
pub async fn create_test_blob(
    blob_client: &BlobClient,
    data: Option<RequestContent<Bytes, NoFormat>>,
    options: Option<BlockBlobClientUploadOptions<'_>>,
) -> Result<Response<BlockBlobClientUploadResult, NoFormat>> {
    match data {
        Some(content) => {
            blob_client
                .upload(content.clone(), true, content.body().len() as u64, options)
                .await
        }
        None => {
            blob_client
                .upload(
                    RequestContent::from(b"hello rusty world".to_vec()),
                    true,
                    17,
                    options,
                )
                .await
        }
    }
}

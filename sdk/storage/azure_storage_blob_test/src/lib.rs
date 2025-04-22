// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    http::{ClientOptions, RequestContent, Response},
    Result,
};
use azure_core_test::Recording;
use azure_storage_blob::{
    models::BlockBlobClientUploadResult, BlobClient, BlobClientOptions, BlobContainerClient,
    BlobContainerClientOptions, BlobServiceClient, BlobServiceClientOptions,
};

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
pub fn get_container_client(recording: &Recording) -> Result<BlobContainerClient> {
    let container_name = recording
        .random_string::<17>(Some("container"))
        .to_ascii_lowercase();
    let (options, endpoint) = recorded_test_setup(recording);
    let container_client_options = BlobContainerClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    BlobContainerClient::new(
        &endpoint,
        container_name,
        recording.credential(),
        Some(container_client_options),
    )
}

/// Returns an instance of a BlobClient.
///
/// # Arguments
///
/// * `container_name` - The name of the container containing this blob.
/// * `recording` - A reference to a Recording instance.
pub fn get_blob_client(
    container_name: Option<String>,
    recording: &Recording,
) -> Result<BlobClient> {
    let container_name = container_name.unwrap_or(
        recording
            .random_string::<17>(Some("container"))
            .to_ascii_lowercase(),
    );
    let blob_name = recording
        .random_string::<12>(Some("blob"))
        .to_ascii_lowercase();
    let (options, endpoint) = recorded_test_setup(recording);
    let blob_client_options = BlobClientOptions {
        client_options: options.clone(),
        ..Default::default()
    };
    BlobClient::new(
        &endpoint,
        container_name,
        blob_name,
        recording.credential(),
        Some(blob_client_options),
    )
}

/// Creates a test blob with no options, containing the data "b'hello rusty world'" with content length 17.
///
/// # Arguments
///
/// * `blob_client` - A reference to a BlobClient instance.
pub async fn create_test_blob(
    blob_client: &BlobClient,
) -> Result<Response<BlockBlobClientUploadResult>> {
    blob_client
        .upload(
            RequestContent::from(b"hello rusty world".to_vec()),
            true,
            17,
            None,
        )
        .await
}

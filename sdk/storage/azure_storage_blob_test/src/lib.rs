// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{http::ClientOptions, Result};
use azure_core_test::Recording;
use azure_storage_blob::{
    BlobClient, BlobClientOptions, BlobContainerClient, BlobContainerClientOptions,
    BlobServiceClient, BlobServiceClientOptions,
};

pub fn _recorded_test_setup(recording: &Recording) -> (ClientOptions, String) {
    let mut client_options = ClientOptions::default();
    recording.instrument(&mut client_options);
    let endpoint = format!(
        "https://{}.blob.core.windows.net/",
        recording.var("AZURE_STORAGE_ACCOUNT_NAME", None).as_str()
    );

    (client_options, endpoint)
}

pub fn get_blob_service_client(recording: &Recording) -> Result<BlobServiceClient> {
    let (options, endpoint) = _recorded_test_setup(recording);
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

pub fn get_container_client(recording: &Recording) -> Result<BlobContainerClient> {
    let container_name = recording
        .random_string::<17>(Some("container"))
        .to_ascii_lowercase();
    let (options, endpoint) = _recorded_test_setup(recording);
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
    let (options, endpoint) = _recorded_test_setup(recording);
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

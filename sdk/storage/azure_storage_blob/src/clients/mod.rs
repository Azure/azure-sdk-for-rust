// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Clients used to communicate with Azure Blob Storage.

use azure_core::http::{new_http_client, ClientOptions, HttpClientOptions, Transport};

mod append_blob_client;
mod blob_client;
mod blob_container_client;
mod blob_service_client;
mod block_blob_client;
mod page_blob_client;

pub use append_blob_client::{AppendBlobClient, AppendBlobClientOptions};
pub use blob_client::{BlobClient, BlobClientOptions};
pub use blob_container_client::{BlobContainerClient, BlobContainerClientOptions};
pub use blob_service_client::{BlobServiceClient, BlobServiceClientOptions};
pub use block_blob_client::{BlockBlobClient, BlockBlobClientOptions};
pub use page_blob_client::{PageBlobClient, PageBlobClientOptions};

#[allow(clippy::needless_update)]
fn apply_client_defaults(options: &mut ClientOptions) {
    if options.transport.is_none() {
        options.transport = Some(Transport::new(new_http_client(Some(HttpClientOptions {
            automatic_decompression: false,
            ..Default::default()
        }))))
    }
}

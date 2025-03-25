// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

mod blob_client;
mod blob_container_client;
mod blob_service_client;

pub use blob_client::BlobClient;
pub use blob_container_client::BlobContainerClient as ContainerClient;
pub use blob_service_client::BlobServiceClient as ServiceClient;

pub(crate) use crate::generated::BlobClient as GeneratedBlobClient;
pub use crate::generated::{
    AppendBlobClient, BlobClientOptions, BlobContainerClient, BlobServiceClient, BlockBlobClient,
    PageBlobClient,
};

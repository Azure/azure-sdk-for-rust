// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Model types for Azure Blob Storage.

pub mod error;
mod extensions;
pub(crate) mod http_ranges;
pub mod method_options;

pub use crate::generated::models::*;

// TODO: This will all go away as a result of having our handwritten replacements.
pub use crate::generated::models::{
    BlobClientDownloadInternalOptions as BlobClientDownloadOptions,
    BlobClientDownloadInternalResult as BlobClientDownloadResult,
    BlobClientDownloadInternalResultHeaders as BlobClientDownloadResultHeaders,
    BlockBlobClientUploadInternalOptions as BlockBlobClientUploadOptions,
    BlockBlobClientUploadInternalResult as BlockBlobClientUploadResult,
    BlockBlobClientUploadInternalResultHeaders as BlockBlobClientUploadResultHeaders,
};

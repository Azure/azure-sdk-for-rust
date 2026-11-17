// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::fmt::SafeDebug;

use crate::models::BlockBlobClientStageBlockInternalResult;

/// Contains results for `BlockBlobClient::stage_block_internal()`
#[derive(SafeDebug)]
pub struct BlockBlobClientStageBlockResult;

impl From<BlockBlobClientStageBlockInternalResult> for BlockBlobClientStageBlockResult {
    fn from(_value: BlockBlobClientStageBlockInternalResult) -> Self {
        BlockBlobClientStageBlockResult
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::num::NonZeroU32;

use azure_data_cosmos_driver::{models::ContinuationToken, Result};

use super::{client_error, CheckpointError, OwnedLease};

pub(super) fn validate_checkpoint_receipt(
    expected: &OwnedLease,
    candidate: &ContinuationToken,
    receipt: &OwnedLease,
) -> Result<()> {
    if receipt.id() != expected.id()
        || receipt.owner() != expected.owner()
        || receipt.epoch() != expected.epoch()
        || receipt.range() != expected.range()
        || receipt.checkpoint() != candidate
        || receipt.revision() == expected.revision()
    {
        return Err(client_error(
            "checkpoint receipt does not confirm the candidate and ownership revision",
        ));
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum CheckpointRetry {
    Retry,
    Fail,
}

pub(super) fn checkpoint_retry(
    error: &CheckpointError,
    attempt: u32,
    limit: NonZeroU32,
) -> CheckpointRetry {
    match error {
        CheckpointError::Retryable(_) if attempt < limit.get() => CheckpointRetry::Retry,
        _ => CheckpointRetry::Fail,
    }
}

#[cfg(test)]
mod tests;

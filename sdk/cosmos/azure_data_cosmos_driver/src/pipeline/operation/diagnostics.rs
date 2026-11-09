// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Maps operation progress to diagnostics.

use crate::{
    diagnostics::ExecutionContext, pipeline::operation::recovery::state::OperationRetryState,
};

/// Records terminal recovery progress without consuming the operation journal.
pub(crate) fn record_abort(
    operation: &crate::models::CosmosOperation,
    state: &OperationRetryState,
    config: &crate::pipeline::operation::config::OperationPipelineConfig<'_>,
    error: &crate::error::CosmosError,
) {
    tracing::debug!(
        activity_id = %config.activity_id,
        status = ?error.status(),
        error = %error,
        operation_type = ?operation.operation_type(),
        resource_type = ?operation.resource_type(),
        is_read_only = operation.is_read_only(),
        is_idempotent = operation.is_idempotent(),
        failover_retries = state.failover_retry_count,
        session_retries = state.session_token_retry_count,
        pk_range_id = ?state.partition_key_range_id,
        "operation aborted",
    );
}

/// Maps attempt progress to the transport diagnostic context.
///
/// Session retries take precedence over region failover because a session retry
/// can retain a previous attempt's failover count.
pub(crate) fn execution_context(retry_state: &OperationRetryState) -> ExecutionContext {
    if retry_state.failover_retry_count == 0 && retry_state.session_token_retry_count == 0 {
        ExecutionContext::Initial
    } else if retry_state.session_token_retry_count > 0 {
        ExecutionContext::OperationRetry
    } else {
        ExecutionContext::RegionFailover
    }
}

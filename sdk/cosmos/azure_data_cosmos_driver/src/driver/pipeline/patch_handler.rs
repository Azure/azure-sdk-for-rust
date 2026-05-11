// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-side handler for [`OperationType::Patch`] operations.
//!
//! See `docs/PATCH_HANDLER_SPEC.md` for the full behavior contract. The
//! short version:
//!
//! 1. Validate the patch spec (no ops that target partition-key paths).
//! 2. Issue an internal [`OperationType::Read`] for the target item.
//! 3. Capture the response ETag (refuse to RMW if there isn't one).
//! 4. Parse the JSON body into a [`serde_json::Value`], apply the ops locally
//!    using [`apply_patch_ops`], and re-serialize.
//! 5. Issue an internal ETag-guarded [`OperationType::Replace`].
//! 6. On `412 Precondition Failed`, restart from step 2 — up to
//!    `max_attempts` (default 5) total tries.
//! 7. Synthesize a [`CosmosResponse`] from the locally-merged body plus the
//!    transport headers/status/diagnostics of the final Replace.
//!
//! This is the only place in the driver allowed to deserialize a data plane
//! response body. It is gated behind the `Patch` operation type so the
//! schema-agnostic invariant continues to hold for every other code path.
//!
//! [`OperationType::Read`]: crate::models::OperationType::Read
//! [`OperationType::Replace`]: crate::models::OperationType::Replace
//! [`OperationType::Patch`]: crate::models::OperationType::Patch
//! [`apply_patch_ops`]: super::patch_eval::apply_patch_ops

use crate::driver::pipeline::from_local_body::from_local_body_and_driver_headers;
use crate::driver::pipeline::patch_eval::apply_patch_ops;
use crate::driver::CosmosDriver;
use crate::models::{
    CosmosOperation, CosmosResponse, PartitionKeyKind, PatchOp, PatchSpec, Precondition,
};
use crate::options::OperationOptions;
use azure_core::http::StatusCode;
use std::num::NonZeroU8;

/// Default cap on the number of RMW attempts before surfacing the latest
/// `412 PreconditionFailed` to the caller.
pub const DEFAULT_PATCH_MAX_ATTEMPTS: u8 = 5;

/// Executes a PATCH operation by running the Read-Modify-Write loop.
///
/// `max_attempts` is the *total* number of attempts (not retries). `None`
/// uses [`DEFAULT_PATCH_MAX_ATTEMPTS`].
pub(crate) async fn execute(
    driver: &CosmosDriver,
    operation: CosmosOperation,
    options: OperationOptions,
    max_attempts: Option<NonZeroU8>,
) -> azure_core::Result<CosmosResponse> {
    // -- 1. Parse and validate the patch spec --
    let body = operation
        .body()
        .ok_or_else(|| missing_body_error("PATCH operation requires a PatchSpec body"))?;
    let spec: PatchSpec = serde_json::from_slice(body).map_err(|err| {
        azure_core::Error::with_message(
            azure_core::error::ErrorKind::DataConversion,
            format!("failed to parse PATCH body as PatchSpec: {err}"),
        )
    })?;

    if spec.operations.is_empty() {
        return Err(azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "PATCH operation must include at least one PatchOp",
        ));
    }

    let item_ref = operation
        .partition_key()
        .cloned()
        .and_then(|pk| operation.resource_reference().try_into_item_reference(pk))
        .ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "PATCH dispatch requires an item-level operation with a partition key",
            )
        })?;

    validate_partition_key_paths(&spec.operations, &item_ref)?;

    let attempts = max_attempts
        .map(|n| n.get())
        .unwrap_or(DEFAULT_PATCH_MAX_ATTEMPTS);

    // -- 2..6. RMW loop --
    let mut last_412: Option<azure_core::Error> = None;
    for _ in 0..attempts {
        // Read the current item.
        let read_op = CosmosOperation::read_item(item_ref.clone());

        let read_resp = driver
            .execute_operation(read_op, OperationOptions::default())
            .await?;
        let status = read_resp.status();
        if !status.is_success() {
            // Surface the underlying Read failure verbatim (NotFound, throttling,
            // etc.). The error already carries diagnostics.
            return Err(read_status_error(status));
        }
        let etag = read_resp.headers().etag.clone().ok_or_else(|| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::Other,
                "PATCH cannot proceed: the Read response did not include an ETag",
            )
        })?;

        // Locally apply the patch ops.
        let mut value: serde_json::Value =
            serde_json::from_slice(read_resp.body()).map_err(|err| {
                azure_core::Error::with_message(
                    azure_core::error::ErrorKind::DataConversion,
                    format!("PATCH could not deserialize current item body: {err}"),
                )
            })?;
        apply_patch_ops(&mut value, &spec.operations)?;
        let merged_bytes = serde_json::to_vec(&value).map_err(|err| {
            azure_core::Error::with_message(
                azure_core::error::ErrorKind::DataConversion,
                format!("PATCH could not serialize merged item: {err}"),
            )
        })?;

        // Issue the ETag-guarded Replace.
        let replace_op = CosmosOperation::replace_item(item_ref.clone())
            .with_body(merged_bytes.clone())
            .with_precondition(Precondition::if_match(etag));

        let replace_resp = driver
            .execute_operation(replace_op, options.clone())
            .await?;
        let replace_status = replace_resp.status();
        if replace_status.status_code() == StatusCode::PreconditionFailed {
            // 412 — someone raced us. Restart the loop.
            last_412 = Some(precondition_failed_error());
            continue;
        }
        if !replace_status.is_success() {
            return Err(read_status_error(replace_status));
        }

        // Synthesize the final response: use the merged body we just sent (the
        // PATCH post-image) plus the driver-routed transport metadata from the
        // Replace.
        return Ok(from_local_body_and_driver_headers(
            merged_bytes,
            replace_resp.headers().clone(),
            replace_resp.status(),
            replace_resp.diagnostics(),
        ));
    }

    Err(last_412.unwrap_or_else(|| {
        azure_core::Error::with_message(
            azure_core::error::ErrorKind::Other,
            "PATCH exhausted attempts without success",
        )
    }))
}

fn missing_body_error(msg: &'static str) -> azure_core::Error {
    azure_core::Error::with_message(azure_core::error::ErrorKind::Other, msg)
}

fn read_status_error(status: crate::models::CosmosStatus) -> azure_core::Error {
    azure_core::Error::with_message(
        azure_core::error::ErrorKind::Other,
        format!(
            "PATCH inner operation failed with status {}",
            status.status_code()
        ),
    )
}

fn precondition_failed_error() -> azure_core::Error {
    azure_core::Error::with_message(
        azure_core::error::ErrorKind::Other,
        "PATCH exhausted attempts: every Read-Modify-Write attempt was preempted (412 PreconditionFailed)",
    )
}

/// Rejects patches that try to mutate the partition key.
///
/// A PATCH that crosses the partition key path can't be implemented safely by
/// a client-side RMW loop — mutating the partition key means the item moves
/// partitions, which can't be done atomically through a Replace. Fail fast
/// rather than silently produce an inconsistent state.
fn validate_partition_key_paths(
    ops: &[PatchOp],
    item_ref: &crate::models::ItemReference,
) -> azure_core::Result<()> {
    let pk_def = item_ref.container().partition_key_definition();
    let pk_paths: Vec<&str> = pk_def.paths().iter().map(|p| p.as_ref()).collect();
    // Hash and MultiHash treat each path as a JSON Pointer rooted at the
    // document. Range PKs are deprecated and never reached the public API, but
    // we treat them identically for safety.
    let kind = pk_def.kind();
    debug_assert!(matches!(
        kind,
        PartitionKeyKind::Hash | PartitionKeyKind::MultiHash | PartitionKeyKind::Range
    ));
    let _ = kind;

    for op in ops {
        let path = op.path();
        for pk_path in &pk_paths {
            if path_overlaps_partition_key(path, pk_path) {
                return Err(azure_core::Error::with_message(
                    azure_core::error::ErrorKind::Other,
                    format!(
                        "PATCH op '{path}' overlaps partition key path '{pk_path}'; \
                         cannot mutate partition key with a client-side Read-Modify-Write"
                    ),
                ));
            }
        }
    }
    Ok(())
}

fn path_overlaps_partition_key(op_path: &str, pk_path: &str) -> bool {
    // Both should already start with '/'. Equal paths overlap; an op path that
    // is an ancestor (e.g., '/account' when PK is '/account/tenantId') also
    // overlaps; an op path that descends into a PK subtree
    // (e.g., '/account/tenantId/extra' on PK '/account/tenantId') also
    // overlaps. The check is symmetric on prefixes split at '/'.
    if op_path == pk_path {
        return true;
    }
    let with_slash = |p: &str| {
        if p.ends_with('/') {
            p.to_string()
        } else {
            format!("{p}/")
        }
    };
    let a = with_slash(op_path);
    let b = with_slash(pk_path);
    a.starts_with(&b) || b.starts_with(&a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_overlap_detection() {
        // Equal paths overlap.
        assert!(path_overlaps_partition_key("/pk", "/pk"));
        // Descendants of the PK overlap.
        assert!(path_overlaps_partition_key("/pk/inner", "/pk"));
        // Ancestors of the PK overlap (PATCH would shadow the PK).
        assert!(path_overlaps_partition_key("/account", "/account/tenantId"));
        // Sibling paths do not overlap.
        assert!(!path_overlaps_partition_key("/pkOther", "/pk"));
        assert!(!path_overlaps_partition_key("/other", "/pk"));
    }
}

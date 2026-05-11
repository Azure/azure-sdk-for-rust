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
use azure_core::error::ErrorKind;
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

    // Capture the caller's session token (if any). The PATCH outer
    // CosmosOperation carries it on its request headers because the SDK
    // wrapper applies it via `apply_item_options`. We propagate it to the
    // internal Read so we get a session-consistent view of the current item,
    // then override with the Read's response session token on the Replace —
    // closing the SE-004 TOCTOU window.
    let caller_session_token = operation.request_headers().session_token.clone();

    // -- 2..6. RMW loop --
    let mut last_412: Option<azure_core::Error> = None;
    for _ in 0..attempts {
        // Read the current item, propagating the caller's session token.
        let read_op = build_read_sub_op(item_ref.clone(), caller_session_token.clone());

        // Any non-2xx Read response is mapped by the driver pipeline into
        // `Err(ErrorKind::HttpResponse { .. })` (see retry_evaluation.rs's
        // `build_http_error`). Propagating with `?` is sufficient — the
        // caller wants the original error verbatim, complete with
        // `raw_response` and diagnostics — and there is nothing useful the
        // PATCH handler can do on a Read failure.
        let read_resp = driver.execute_operation(read_op, options.clone()).await?;
        let etag = read_resp.headers().etag.clone().ok_or_else(|| {
            azure_core::Error::with_message(
                ErrorKind::Other,
                "PATCH cannot proceed: the Read response did not include an ETag",
            )
        })?;
        // R3-DRIVER: forward the session token returned by the Read on the
        // Replace, so the write commits against the same replica view we
        // just read from. This is what mitigates SE-004 (session token
        // TOCTOU across read->write).
        let read_session_token = read_resp.headers().session_token.clone();

        // Locally apply the patch ops.
        let mut value: serde_json::Value =
            serde_json::from_slice(read_resp.body()).map_err(|err| {
                azure_core::Error::with_message(
                    ErrorKind::DataConversion,
                    format!("PATCH could not deserialize current item body: {err}"),
                )
            })?;
        apply_patch_ops(&mut value, &spec.operations)?;
        let merged_bytes = serde_json::to_vec(&value).map_err(|err| {
            azure_core::Error::with_message(
                ErrorKind::DataConversion,
                format!("PATCH could not serialize merged item: {err}"),
            )
        })?;

        // Issue the ETag-guarded Replace, forwarding the Read response's
        // session token (overriding any caller-supplied value).
        let replace_op = build_replace_sub_op(
            item_ref.clone(),
            merged_bytes.clone(),
            etag,
            read_session_token,
        );

        // The driver pipeline returns `Err(ErrorKind::HttpResponse { .. })`
        // for any non-2xx Replace response (412 included — `OperationAction::Abort`
        // is the terminal disposition for 412). So the success / 412 split
        // happens on the `Result` itself, not on a status code we never get
        // to inspect.
        match driver.execute_operation(replace_op, options.clone()).await {
            Ok(replace_resp) => {
                // Synthesize the final response: use the merged body we just
                // sent (the PATCH post-image) plus the driver-routed
                // transport metadata from the Replace.
                return Ok(from_local_body_and_driver_headers(
                    merged_bytes,
                    replace_resp.headers().clone(),
                    replace_resp.status(),
                    replace_resp.diagnostics(),
                ));
            }
            Err(err) if is_precondition_failed(&err) => {
                // 412 — someone raced us. Stash the real service error so
                // exhaustion_error can chain it as the underlying cause.
                last_412 = Some(err);
                continue;
            }
            Err(err) => return Err(err),
        }
    }

    Err(exhaustion_error(attempts, last_412))
}

fn missing_body_error(msg: &'static str) -> azure_core::Error {
    azure_core::Error::with_message(ErrorKind::Other, msg)
}

/// Returns `true` if `err` is the driver pipeline's representation of a
/// `412 Precondition Failed` HTTP response (i.e. our ETag-guarded Replace
/// lost the race against a concurrent writer).
///
/// The driver pipeline maps every non-2xx response — 412 included — into
/// `Err(azure_core::Error { kind: ErrorKind::HttpResponse { status, .. }, .. })`
/// via `retry_evaluation::build_http_error`, and 412 specifically resolves
/// to `OperationAction::Abort` (it is never retried at the pipeline layer).
/// The patch handler's RMW loop is the *one* place where 412 needs to be
/// recovered into a retry, so we narrow on the kind here instead of relying
/// on a status check that the `await?` above would never reach.
fn is_precondition_failed(err: &azure_core::Error) -> bool {
    matches!(
        err.kind(),
        ErrorKind::HttpResponse { status, .. } if *status == StatusCode::PreconditionFailed
    )
}

/// Builds the internal Read sub-operation used by the RMW loop, propagating
/// the caller's session token so the read sees a session-consistent view.
fn build_read_sub_op(
    item_ref: crate::models::ItemReference,
    caller_session_token: Option<crate::models::SessionToken>,
) -> CosmosOperation {
    let mut op = CosmosOperation::read_item(item_ref);
    if let Some(token) = caller_session_token {
        op = op.with_session_token(token);
    }
    op
}

/// Builds the internal Replace sub-operation used by the RMW loop. The
/// session token comes from the Read response (NOT the caller's options) so
/// the write commits against the same replica view we just read from. This
/// is the SE-004 TOCTOU mitigation.
fn build_replace_sub_op(
    item_ref: crate::models::ItemReference,
    merged_bytes: Vec<u8>,
    etag: crate::models::ETag,
    read_response_session_token: Option<crate::models::SessionToken>,
) -> CosmosOperation {
    let mut op = CosmosOperation::replace_item(item_ref)
        .with_body(merged_bytes)
        .with_precondition(Precondition::if_match(etag));
    if let Some(token) = read_response_session_token {
        op = op.with_session_token(token);
    }
    op
}

/// Builds the final error returned to callers when the RMW loop exhausted
/// `attempts` retries without ever landing a Replace. The underlying 412 is
/// preserved as the source so `Error::source()` / debug formatting still
/// surfaces the original cause.
fn exhaustion_error(attempts: u8, last_412: Option<azure_core::Error>) -> azure_core::Error {
    let message = format!("patch_item: ETag conflict after {attempts} attempts");
    match last_412 {
        Some(source) => azure_core::Error::with_error(
            ErrorKind::HttpResponse {
                status: StatusCode::PreconditionFailed,
                error_code: None,
                raw_response: None,
            },
            source,
            message,
        ),
        None => azure_core::Error::with_message(
            ErrorKind::HttpResponse {
                status: StatusCode::PreconditionFailed,
                error_code: None,
                raw_response: None,
            },
            message,
        ),
    }
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
        // For most ops, only the destination `path` mutates the document.
        // For `MoveOp`, the source `from` is *also* mutated (the field is
        // removed at `from` after being inserted at `path`), so a move
        // *out of* a PK path is just as illegal as a move *into* one — it
        // would silently delete the partition key field.
        let dest = op.path();
        let from = match op {
            PatchOp::MoveOp { from, .. } => Some(from.as_str()),
            _ => None,
        };
        for path in std::iter::once(dest).chain(from) {
            for pk_path in &pk_paths {
                if path_overlaps_partition_key(path, pk_path) {
                    return Err(azure_core::Error::with_message(
                        ErrorKind::Other,
                        format!(
                            "PATCH op '{path}' overlaps partition key path '{pk_path}'; \
                             cannot mutate partition key with a client-side Read-Modify-Write"
                        ),
                    ));
                }
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
    use crate::models::{
        AccountReference, ContainerProperties, ContainerReference, ETag, ItemReference,
        OperationType, PartitionKey, PartitionKeyDefinition, SessionToken, SystemProperties,
    };
    use azure_core::http::Url;
    use std::borrow::Cow;

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "test-key",
        )
    }

    fn test_partition_key_definition(path: &str) -> PartitionKeyDefinition {
        serde_json::from_str(&format!(r#"{{"paths":["{path}"]}}"#)).unwrap()
    }

    fn test_container() -> ContainerReference {
        let props = ContainerProperties {
            id: "testcontainer".into(),
            partition_key: test_partition_key_definition("/pk"),
            system_properties: SystemProperties::default(),
        };
        ContainerReference::new(
            test_account(),
            "testdb",
            "testdb_rid",
            "testcontainer",
            "testcontainer_rid",
            &props,
        )
    }

    fn test_item_ref() -> ItemReference {
        ItemReference::from_name(&test_container(), PartitionKey::from("pk1"), "doc1")
    }

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

    #[test]
    fn read_sub_op_propagates_caller_session_token() {
        // R3-DRIVER / SE-004: caller's session token must reach the internal Read so
        // we get a session-consistent view of the current item.
        let caller_token = SessionToken(Cow::Owned("0:1#42".into()));
        let op = build_read_sub_op(test_item_ref(), Some(caller_token.clone()));

        assert_eq!(op.operation_type(), OperationType::Read);
        assert_eq!(
            op.request_headers().session_token.as_ref(),
            Some(&caller_token)
        );
    }

    #[test]
    fn read_sub_op_omits_token_when_caller_has_none() {
        let op = build_read_sub_op(test_item_ref(), None);

        assert_eq!(op.operation_type(), OperationType::Read);
        assert!(op.request_headers().session_token.is_none());
    }

    #[test]
    fn replace_sub_op_uses_read_response_session_token() {
        // SE-004 TOCTOU mitigation: the Replace must commit against the same replica
        // view we just read from, so the session token comes from the Read response,
        // not from the caller's options.
        let read_response_token = SessionToken(Cow::Owned("0:1#99".into()));
        let etag = ETag::from("\"abc\"");
        let body = b"{\"id\":\"doc1\"}".to_vec();

        let op = build_replace_sub_op(
            test_item_ref(),
            body.clone(),
            etag.clone(),
            Some(read_response_token.clone()),
        );

        assert_eq!(op.operation_type(), OperationType::Replace);
        assert_eq!(op.body(), Some(body.as_slice()));
        assert_eq!(
            op.request_headers().session_token.as_ref(),
            Some(&read_response_token)
        );
        // F16: assert the If-Match precondition was applied. A future refactor
        // that silently dropped `.with_precondition(...)` would downgrade the
        // RMW to a non-conditional Replace — precisely the bug R3-DRIVER's
        // ETag guard exists to prevent.
        assert_eq!(op.precondition(), Some(&Precondition::if_match(etag)));
    }

    #[test]
    fn replace_sub_op_omits_token_when_read_response_has_none() {
        let etag = ETag::from("\"abc\"");
        let op = build_replace_sub_op(test_item_ref(), Vec::new(), etag, None);

        assert_eq!(op.operation_type(), OperationType::Replace);
        assert!(op.request_headers().session_token.is_none());
    }

    #[test]
    fn is_precondition_failed_matches_real_412() {
        // F-C1: the RMW loop's 412 detection runs on the `Err(_)` produced
        // by the driver pipeline. The pipeline's `build_http_error` builds
        // `ErrorKind::HttpResponse { status, error_code, raw_response: Some(_) }`
        // for any non-2xx; on a 412 the status field is the discriminator
        // we need to retry on.
        use azure_core::Error;

        let err = Error::with_message(
            ErrorKind::HttpResponse {
                status: StatusCode::PreconditionFailed,
                error_code: None,
                raw_response: None,
            },
            "412 from server",
        );
        assert!(is_precondition_failed(&err));
    }

    #[test]
    fn is_precondition_failed_rejects_other_http_statuses() {
        use azure_core::Error;

        for status in [
            StatusCode::NotFound,
            StatusCode::Conflict,
            StatusCode::TooManyRequests,
            StatusCode::ServiceUnavailable,
        ] {
            let err = Error::with_message(
                ErrorKind::HttpResponse {
                    status,
                    error_code: None,
                    raw_response: None,
                },
                "non-412 service error",
            );
            assert!(
                !is_precondition_failed(&err),
                "should not match status {status:?}",
            );
        }
    }

    #[test]
    fn is_precondition_failed_rejects_non_http_error_kinds() {
        use azure_core::Error;

        for err in [
            Error::with_message(ErrorKind::Other, "synthetic"),
            Error::with_message(ErrorKind::DataConversion, "bad json"),
            Error::with_message(ErrorKind::Io, "tcp reset"),
        ] {
            assert!(
                !is_precondition_failed(&err),
                "should not match {:?}",
                err.kind()
            );
        }
    }

    #[test]
    fn pk_guard_rejects_move_from_pk_path() {
        // F-C2: moving FROM a PK path mutates the partition key (the field
        // is removed after being copied to the destination), so the
        // preflight guard must reject it just like a move TO a PK path.
        // Reuses the `/pk` flat PK fixture.
        let item_ref = test_item_ref();
        let ops = vec![PatchOp::move_op("/pk", "/somewhere_else")];

        let err = validate_partition_key_paths(&ops, &item_ref)
            .expect_err("MoveOp from /pk on a /pk PK must be rejected");
        let msg = format!("{err}").to_ascii_lowercase();
        assert!(
            msg.contains("partition key"),
            "error should mention partition key; got: {err}"
        );
    }

    #[test]
    fn pk_guard_rejects_move_from_pk_path_hierarchical() {
        // Same as the flat test but exercises one path of a MultiHash PK
        // (`/tenant`, `/region`, `/user`). A move out of `/tenant` would
        // erase a component of the hierarchical partition key.
        let pk_def: PartitionKeyDefinition = serde_json::from_str(
            r#"{"paths":["/tenant","/region","/user"],"kind":"MultiHash","version":2}"#,
        )
        .unwrap();
        let props = ContainerProperties {
            id: "hpkc".into(),
            partition_key: pk_def,
            system_properties: SystemProperties::default(),
        };
        let container = ContainerReference::new(
            test_account(),
            "testdb",
            "testdb_rid",
            "hpkc",
            "hpkc_rid",
            &props,
        );
        let item_ref =
            ItemReference::from_name(&container, PartitionKey::from(("t1", "r1", "u1")), "doc1");

        let ops = vec![PatchOp::move_op("/tenant", "/somewhere_else")];

        let err = validate_partition_key_paths(&ops, &item_ref)
            .expect_err("MoveOp from /tenant on a hierarchical PK must be rejected");
        let msg = format!("{err}").to_ascii_lowercase();
        assert!(
            msg.contains("partition key"),
            "error should mention partition key; got: {err}"
        );
    }
}

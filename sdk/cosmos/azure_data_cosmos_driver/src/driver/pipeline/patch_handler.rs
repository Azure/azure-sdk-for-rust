// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-side handler for [`OperationType::Patch`] operations.
//!
//! See `docs/PATCH_HANDLER_SPEC.md` for the full behavior contract. The
//! short version:
//!
//! 1. Validate the patch spec (no ops that target partition-key paths).
//! 2. Issue an internal [`OperationType::Read`] for the target item.
//! 3. Capture the response ETag and evaluate any caller precondition against it.
//! 4. Refuse to RMW if the Read did not return an ETag.
//! 5. Parse the JSON body into a [`serde_json::Value`], apply the ops locally
//!    using [`apply_patch_ops`], and re-serialize.
//! 6. Issue an internal ETag-guarded [`OperationType::Replace`].
//! 7. On `412 Precondition Failed`, restart from step 3 — up to
//!    `max_attempts` (default 5) total tries. Each Read is a write-region
//!    `LatestCommitted` read without a session token unless write routing is
//!    unavailable, in which case normal read routing uses session consistency.
//! 8. Synthesize a [`CosmosResponse`] from the locally-merged body plus the
//!    transport headers/status of the final Replace and an aggregated
//!    [`DiagnosticsContext`] that concatenates every successful sub-op's
//!    per-request diagnostics — so callers see one PATCH operation = one
//!    [`DiagnosticsContext`].
//!
//! This is the only place in the driver allowed to deserialize a data plane
//! response body. It is gated behind the `Patch` operation type so the
//! schema-agnostic invariant continues to hold for every other code path.
//!
//! [`OperationType::Read`]: crate::models::OperationType::Read
//! [`OperationType::Replace`]: crate::models::OperationType::Replace
//! [`OperationType::Patch`]: crate::models::OperationType::Patch
//! [`apply_patch_ops`]: super::patch_eval::apply_patch_ops
//! [`DiagnosticsContext`]: crate::diagnostics::DiagnosticsContext

use crate::diagnostics::DiagnosticsContext;
use crate::driver::pipeline::from_local_body::from_local_body_and_driver_headers;
use crate::driver::pipeline::patch_eval::apply_patch_ops;
use crate::driver::pipeline::patch_tracking::{
    prepare_tracking_marker, TrackingMarkerOutcome, PATCH_TRACKING_POINTER,
};
use crate::driver::CosmosDriver;
use crate::models::{
    CosmosOperation, CosmosResponse, PartitionKeyKind, PatchInstructions, PatchOperation,
    Precondition,
};
use crate::options::{
    BinaryEncodingOptions, ContentResponseOnWrite, OperationOptions, ReadConsistencyStrategy,
};
use async_trait::async_trait;
use azure_core::http::{Etag, StatusCode};
use std::num::NonZeroU8;
use std::sync::Arc;
use std::time::Instant;

/// Default cap on the number of RMW attempts before surfacing the latest
/// `412 PreconditionFailed` to the caller.
pub const DEFAULT_PATCH_MAX_ATTEMPTS: u8 = 5;

/// Internal abstraction for dispatching sub-operations from inside the
/// PATCH handler's RMW loop.
///
/// Production code uses the `CosmosDriver` impl, which forwards to
/// `CosmosDriver::execute_operation`. Unit tests provide stub impls so the
/// loop body — including the 412 retry path, the exhaustion error, and the
/// PK guard's "no I/O on rejection" contract — can be exercised without a
/// live Cosmos endpoint or in-memory emulator.
///
/// This trait is `pub(crate)` and intentionally has no public re-export: it
/// is a testability seam, not API surface.
#[async_trait]
pub(crate) trait SubOperationDispatcher: Send + Sync {
    /// Executes a single Read or Replace sub-operation. The PATCH handler
    /// invokes this twice per RMW attempt (Read, then Replace) and consumes
    /// the result exactly as it would the driver's own
    /// [`CosmosDriver::execute_operation`].
    async fn execute_operation(
        &self,
        operation: CosmosOperation,
        options: OperationOptions,
    ) -> crate::error::Result<CosmosResponse>;

    async fn canonicalize_operation_container(
        &self,
        _operation: &mut CosmosOperation,
    ) -> crate::error::Result<bool> {
        Ok(false)
    }
}

#[async_trait]
impl SubOperationDispatcher for CosmosDriver {
    async fn execute_operation(
        &self,
        operation: CosmosOperation,
        options: OperationOptions,
    ) -> crate::error::Result<CosmosResponse> {
        CosmosDriver::execute_singleton_operation(self, operation, options).await
    }

    async fn canonicalize_operation_container(
        &self,
        operation: &mut CosmosOperation,
    ) -> crate::error::Result<bool> {
        CosmosDriver::canonicalize_operation_container(self, operation).await
    }
}

/// Executes a PATCH operation by running the Read-Modify-Write loop.
///
/// `max_attempts` is the *total* number of attempts (not retries). `None`
/// uses [`DEFAULT_PATCH_MAX_ATTEMPTS`].
pub(crate) async fn execute(
    driver: &CosmosDriver,
    operation: CosmosOperation,
    options: OperationOptions,
    max_attempts: Option<NonZeroU8>,
    absolute_deadline: Option<Instant>,
    return_response_body: bool,
) -> crate::error::Result<CosmosResponse> {
    execute_with_dispatcher_and_deadline(
        driver,
        operation,
        options,
        max_attempts,
        absolute_deadline,
        return_response_body,
    )
    .await
}

/// Same as [`execute`], but parameterized over the sub-operation dispatcher.
/// Tests provide a stub that returns scripted responses without a live
/// endpoint.
#[cfg(test)]
pub(crate) async fn execute_with_dispatcher<D: SubOperationDispatcher + ?Sized>(
    dispatcher: &D,
    operation: CosmosOperation,
    options: OperationOptions,
    max_attempts: Option<NonZeroU8>,
) -> crate::error::Result<CosmosResponse> {
    let absolute_deadline = options
        .end_to_end_latency_policy
        .as_ref()
        .map(|policy| Instant::now() + policy.timeout());
    let return_response_body = !matches!(
        options.content_response_on_write,
        Some(ContentResponseOnWrite::Disabled)
    );
    execute_with_dispatcher_and_deadline(
        dispatcher,
        operation,
        options,
        max_attempts,
        absolute_deadline,
        return_response_body,
    )
    .await
}

async fn execute_with_dispatcher_and_deadline<D: SubOperationDispatcher + ?Sized>(
    dispatcher: &D,
    mut operation: CosmosOperation,
    mut options: OperationOptions,
    max_attempts: Option<NonZeroU8>,
    absolute_deadline: Option<Instant>,
    return_response_body: bool,
) -> crate::error::Result<CosmosResponse> {
    // PATCH is excluded from binary encoding. Force it off *explicitly*:
    // `None` would inherit a lower layer (e.g. an account/client that enabled
    // binary), which would then flow into the internal Read/Replace sub-ops.
    options.binary_encoding = Some(BinaryEncodingOptions::new().with_enabled(false));
    let mut read_options = options.clone();
    read_options.read_consistency_strategy = Some(ReadConsistencyStrategy::LatestCommitted);
    let mut replace_options = options.clone();
    replace_options.content_response_on_write = Some(if return_response_body {
        ContentResponseOnWrite::Enabled
    } else {
        ContentResponseOnWrite::Disabled
    });

    if operation
        .precondition()
        .is_some_and(Precondition::is_if_none_match)
    {
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::CLIENT_BAD_REQUEST)
            .with_message("PATCH supports If-Match preconditions; If-None-Match is read-only")
            .build());
    }

    // -- 1. Parse and validate the patch spec --
    let body = operation
        .body()
        .ok_or_else(|| missing_body_error("PATCH operation requires a PatchInstructions body"))?;
    let spec: PatchInstructions = serde_json::from_slice(body).map_err(|err| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::SERIALIZATION_REQUEST_BODY_INVALID)
            .with_message("failed to parse PATCH body as PatchInstructions")
            .with_source(err)
            .build()
    })?;

    if spec.operations.is_empty() {
        return Err(crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::new(
                azure_core::http::StatusCode::BadRequest,
            ))
            .with_message("PATCH operation must include at least one PatchOperation")
            .build());
    }

    let mut item_ref = operation
        .partition_key()
        .cloned()
        .and_then(|pk| operation.resource_reference().try_into_item_reference(pk))
        .ok_or_else(|| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(
                    "PATCH dispatch requires an item-level operation with a partition key",
                )
                .build()
        })?;

    validate_partition_key_paths(&spec.operations, &item_ref)?;

    let caller_supplied_tracking_id = operation.patch_tracking_id().is_some();
    let requires_tracking = caller_supplied_tracking_id || !spec.is_retry_safe();
    if requires_tracking {
        validate_tracking_partition_key_paths(&item_ref)?;
    }
    let tracking = requires_tracking.then(|| {
        (
            operation
                .patch_tracking_id()
                .unwrap_or_else(crate::models::PatchTrackingId::new),
            operation
                .patch_tracking_capacity()
                .unwrap_or(crate::models::DEFAULT_PATCH_TRACKING_CAPACITY),
            operation
                .patch_tracking_retention_seconds()
                .unwrap_or_else(default_patch_tracking_retention_seconds),
        )
    });
    let tracking_id = tracking.map(|(id, _, _)| id);

    let attempts = max_attempts
        .map(|n| n.get())
        .unwrap_or(DEFAULT_PATCH_MAX_ATTEMPTS);

    // -- 3..7. RMW loop --
    let mut last_412: Option<crate::error::CosmosError> = None;
    let mut replace_dispatched = false;
    // Aggregated diagnostics across every successful sub-op the loop
    // dispatches. We hand this to `from_local_body_and_driver_headers`
    // when we synthesize the success response so callers see one
    // PATCH operation = one DiagnosticsContext containing every
    // sub-op's per-request diagnostics, instead of just the final
    // Replace's. See `DiagnosticsContext::aggregate_sub_operations`.
    let mut sub_op_diagnostics: Vec<Arc<DiagnosticsContext>> =
        Vec::with_capacity(2 * attempts as usize);

    // The aggregated context concatenates the Read + Replace sub-ops and would
    // otherwise inherit the *last* sub-op's `db.operation.name`. Stamp the
    // virtual PATCH operation's own canonical name (`patch_item`) instead, so
    // the operation level reports what the caller actually invoked. The
    // individual sub-ops keep their own `patch_read_item` / `patch_replace_item`
    // identity on their per-request diagnostics, so the read/modify/write
    // decomposition stays visible underneath the aggregate.
    let operation_name: Option<Arc<str>> = operation.db_operation_name().map(Arc::from);
    let caller_session_token = operation.request_headers().session_token.clone();
    let caller_precondition = operation.precondition().cloned();

    let custom_session_token = options.custom_headers.as_ref().is_some_and(|headers| {
        headers.contains_key(&azure_core::http::headers::HeaderName::from_static(
            crate::models::request_header_names::SESSION_TOKEN,
        ))
    });
    let recreation_allowed = caller_session_token.is_none() && !custom_session_token;
    let mut recreation_retried = false;
    let mut rmw_attempts = 0;

    while rmw_attempts < attempts {
        rmw_attempts += 1;
        // Read the current item from the write endpoint at LatestCommitted.
        // Writer routing strips the caller's token because LatestCommitted is
        // outside the session lane. If routing degrades to a reader, the
        // operation pipeline restores account-default consistency and can use
        // this explicit token even when the local session cache is empty.
        let read_op = build_read_sub_op(item_ref.clone(), caller_session_token.clone())
            .with_absolute_deadline(absolute_deadline);

        // Any non-2xx Read response is mapped by the driver pipeline into
        // `Err(ErrorKind::HttpResponse { .. })` (see retry_evaluation.rs's
        // `build_http_error`). The caller wants that error verbatim, complete
        // with `raw_response`, status, and source — there is nothing useful the
        // PATCH handler can do on a Read failure — but the diagnostics riding on
        // it still describe the *sub-op* (`read_item`). Re-stamp the virtual
        // PATCH operation's identity so the failure reports the same
        // `db.operation.name` as its success and retry-exhaustion counterparts.
        let read_resp = match dispatcher
            .execute_operation(read_op, read_options.clone())
            .await
        {
            Ok(response) => response,
            Err(err)
                if recreation_allowed
                    && !recreation_retried
                    && is_container_recreation_error(&err)
                    && dispatcher
                        .canonicalize_operation_container(&mut operation)
                        .await? =>
            {
                recreation_retried = true;
                rmw_attempts -= 1;
                item_ref = operation
                    .partition_key()
                    .cloned()
                    .and_then(|pk| operation.resource_reference().try_into_item_reference(pk))
                    .expect("retargeted PATCH operation remains an item operation");
                push_unique_diagnostics(&mut sub_op_diagnostics, err.diagnostics());
                continue;
            }
            Err(err) => {
                return Err(stamp_patch_identity(
                    err,
                    operation_name.clone(),
                    tracking_id,
                    &sub_op_diagnostics,
                ));
            }
        };
        let read_headers = read_resp.headers().clone();
        let read_status = read_resp.status();
        let routing_fallback = read_resp.routing_fallback();
        let read_diagnostics = read_resp.diagnostics();
        sub_op_diagnostics.push(read_diagnostics);
        // Locally apply the patch ops. These failures are synthesized here
        // rather than returned by the pipeline, so they carry no diagnostics of
        // their own; hand them the PATCH-identified aggregate of the sub-ops
        // issued so far.
        let read_body_bytes = read_resp
            .into_body()
            .single()
            .map_err(|err| {
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message("PATCH could not extract Read response body")
                    .with_source(err)
                    .build()
            })
            .map_err(|err| {
                stamp_patch_identity(
                    err,
                    operation_name.clone(),
                    tracking_id,
                    &sub_op_diagnostics,
                )
            })?;
        let mut value: serde_json::Value = serde_json::from_slice(&read_body_bytes)
            .map_err(|err| {
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message(format!(
                        "PATCH could not deserialize current item body: {err}"
                    ))
                    .with_source(err)
                    .build()
            })
            .map_err(|err| {
                stamp_patch_identity(
                    err,
                    operation_name.clone(),
                    tracking_id,
                    &sub_op_diagnostics,
                )
            })?;

        if let Some((tracking_id, capacity, retention_seconds)) = tracking {
            let marker_outcome = prepare_tracking_marker(
                &mut value,
                tracking_id.as_uuid(),
                capacity,
                retention_seconds,
                !routing_fallback || (!caller_supplied_tracking_id && !replace_dispatched),
            )
            .map_err(|err| {
                stamp_patch_identity(
                    err,
                    operation_name.clone(),
                    Some(tracking_id),
                    &sub_op_diagnostics,
                )
            })?;

            match marker_outcome {
                TrackingMarkerOutcome::AlreadyApplied => {
                    let diagnostics = aggregate_patch_diagnostics(
                        &sub_op_diagnostics,
                        operation_name.clone(),
                        Some(tracking_id),
                    );
                    let mut response_headers = read_headers;
                    response_headers.request_charge = Some(diagnostics.total_request_charge());
                    return Ok(from_local_body_and_driver_headers(
                        read_body_bytes.to_vec(),
                        response_headers,
                        read_status,
                        diagnostics,
                    ));
                }
                TrackingMarkerOutcome::Missing => {
                    return Err(stamp_patch_identity(
                        inconclusive_tracking_verification_error(tracking_id),
                        operation_name.clone(),
                        Some(tracking_id),
                        &sub_op_diagnostics,
                    ));
                }
                TrackingMarkerOutcome::Added => {}
            }
        }

        let etag = read_headers
            .etag
            .clone()
            .ok_or_else(|| {
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::new(
                        azure_core::http::StatusCode::BadRequest,
                    ))
                    .with_message("PATCH cannot proceed: the Read response did not include an ETag")
                    .build()
            })
            .map_err(|err| {
                stamp_patch_identity(
                    err,
                    operation_name.clone(),
                    tracking_id,
                    &sub_op_diagnostics,
                )
            })?;
        validate_caller_precondition(caller_precondition.as_ref(), &etag).map_err(|err| {
            stamp_patch_identity(
                err,
                operation_name.clone(),
                tracking_id,
                &sub_op_diagnostics,
            )
        })?;
        // R3-DRIVER: forward the session token returned by the Read on the
        // Replace, so the write commits against the same replica view we
        // just read from. This is what mitigates SE-004 (session token
        // TOCTOU across read->write).
        let read_session_token = read_headers.session_token.clone();

        apply_patch_ops(&mut value, &spec.operations).map_err(|err| {
            stamp_patch_identity(
                err.into(),
                operation_name.clone(),
                tracking_id,
                &sub_op_diagnostics,
            )
        })?;
        let merged_bytes = serde_json::to_vec(&value)
            .map_err(|err| {
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message("PATCH could not serialize merged item")
                    .with_source(err)
                    .build()
            })
            .map_err(|err| {
                stamp_patch_identity(
                    err,
                    operation_name.clone(),
                    tracking_id,
                    &sub_op_diagnostics,
                )
            })?;

        // Issue the ETag-guarded Replace, forwarding the Read response's
        // session token (overriding any caller-supplied value).
        let replace_op = build_replace_sub_op(
            item_ref.clone(),
            merged_bytes.clone(),
            etag,
            read_session_token,
        )
        .with_absolute_deadline(absolute_deadline);

        // The driver pipeline returns `Err(ErrorKind::HttpResponse { .. })`
        // for any non-2xx Replace response (412 included — `OperationAction::Abort`
        // is the terminal disposition for 412). So the success / 412 split
        // happens on the `Result` itself, not on a status code we never get
        // to inspect.
        replace_dispatched = true;
        match dispatcher
            .execute_operation(replace_op, replace_options.clone())
            .await
        {
            Ok(replace_resp) => {
                let mut replace_headers = replace_resp.headers().clone();
                let replace_status = replace_resp.status();
                let replace_etag = replace_headers.etag.clone();
                sub_op_diagnostics.push(replace_resp.diagnostics());
                let replace_body = replace_resp.into_body();
                // Replace responses are always single-payload (or empty when
                // `content_response_on_write` is disabled). Collapse the
                // typed body to `Vec<u8>` so the synthesis helper can treat
                // "empty" uniformly across `NoPayload` and `Bytes(empty)`.
                let replace_body_bytes: Vec<u8> = match replace_body {
                    crate::models::ResponseBody::Bytes(b) => b.to_vec(),
                    crate::models::ResponseBody::NoPayload
                    | crate::models::ResponseBody::Items(_) => Vec::new(),
                };
                // Aggregate the per-request diagnostics of every successful
                // sub-op into a single DiagnosticsContext, so the synthesized
                // response surfaces "one operation = one DiagnosticsContext"
                // instead of just the Replace's view. Falls back to the
                // Replace's own diagnostics if aggregation somehow fails
                // (e.g. an empty source slice — which can't happen here, but
                // we keep the safe fallback for forward-compat).
                let diagnostics = aggregate_patch_diagnostics(
                    &sub_op_diagnostics,
                    operation_name.clone(),
                    tracking_id,
                );
                replace_headers.request_charge = Some(diagnostics.total_request_charge());
                // Reconcile the locally-merged body's system properties with
                // the Replace response. The merged document still carries
                // `_etag`/`_ts` from the *Read* (it is the Read body with
                // the patch ops applied), but the post-image's authoritative
                // `_etag` is the one the Replace just minted. Without this
                // reconciliation a caller that deserializes the response
                // body and reads `_etag` from it would see a stale value
                // that no longer matches the Replace's response header,
                // breaking optimistic-concurrency round-tripping.
                //
                // Preference order:
                //   1. The Replace's response body, when present (the
                //      service-authoritative post-image — set when
                //      `content_response_on_write` is true).
                //   2. Otherwise, the locally-merged body with `_etag`
                //      overwritten from `replace_headers.etag`.
                let synthesized_body = synthesize_post_image_body(
                    merged_bytes,
                    replace_body_bytes,
                    replace_etag.as_ref(),
                );
                return Ok(from_local_body_and_driver_headers(
                    synthesized_body,
                    replace_headers,
                    replace_status,
                    diagnostics,
                ));
            }
            Err(err) if is_precondition_failed(&err) => {
                // 412 — someone raced us.
                // Stash the real service error so exhaustion_error can
                // chain it as the underlying cause. Also capture the
                // failed sub-op's diagnostics into the aggregated list so
                // every PATCH attempt (Reads + this failed Replace) is
                // visible on the final exhaustion error, not just the
                // Reads that succeeded. The Replace's error already
                // carries its sub-op's `DiagnosticsContext` (the
                // operation pipeline's abort branch attaches it via
                // `CosmosError::with_diagnostics` before returning) — extract
                // and forward it.
                push_unique_diagnostics(&mut sub_op_diagnostics, err.diagnostics());
                last_412 = Some(err);
                continue;
            }
            Err(err) => {
                if recreation_allowed
                    && !recreation_retried
                    && is_container_recreation_error(&err)
                    && dispatcher
                        .canonicalize_operation_container(&mut operation)
                        .await?
                {
                    recreation_retried = true;
                    rmw_attempts -= 1;
                    item_ref = operation
                        .partition_key()
                        .cloned()
                        .and_then(|pk| operation.resource_reference().try_into_item_reference(pk))
                        .expect("retargeted PATCH operation remains an item operation");
                    push_unique_diagnostics(&mut sub_op_diagnostics, err.diagnostics());
                    continue;
                }
                if terminal_error_requires_verification(&err) {
                    if let Some(tracking) = tracking {
                        push_unique_diagnostics(&mut sub_op_diagnostics, err.diagnostics());
                        match verify_committed_patch(
                            dispatcher,
                            &item_ref,
                            &read_options,
                            VerificationContext {
                                tracking,
                                caller_session_token: caller_session_token.clone(),
                                operation_name: operation_name.clone(),
                                absolute_deadline,
                            },
                            &mut sub_op_diagnostics,
                        )
                        .await
                        {
                            Ok(VerificationOutcome::Applied(response)) => return Ok(*response),
                            Ok(VerificationOutcome::Absent) => {}
                            Ok(VerificationOutcome::ReadFailed(verification_error)) => {
                                push_unique_diagnostics(
                                    &mut sub_op_diagnostics,
                                    verification_error.diagnostics(),
                                );
                            }

                            Err(verification_error) => {
                                return Err(stamp_patch_identity(
                                    verification_error,
                                    operation_name,
                                    tracking_id,
                                    &sub_op_diagnostics,
                                ));
                            }
                        }
                    }
                }
                return Err(stamp_patch_identity(
                    err,
                    operation_name.clone(),
                    tracking_id,
                    &sub_op_diagnostics,
                ));
            }
        }
    }

    if let Some(tracking) = tracking {
        match verify_committed_patch(
            dispatcher,
            &item_ref,
            &read_options,
            VerificationContext {
                tracking,
                caller_session_token,
                operation_name: operation_name.clone(),
                absolute_deadline,
            },
            &mut sub_op_diagnostics,
        )
        .await
        {
            Ok(VerificationOutcome::Applied(response)) => return Ok(*response),
            Ok(VerificationOutcome::Absent) => {}
            Ok(VerificationOutcome::ReadFailed(verification_error)) => {
                push_unique_diagnostics(&mut sub_op_diagnostics, verification_error.diagnostics());
            }
            Err(verification_error) => {
                return Err(stamp_patch_identity(
                    verification_error,
                    operation_name,
                    tracking_id,
                    &sub_op_diagnostics,
                ));
            }
        }
    }

    Err(exhaustion_error(
        attempts,
        last_412,
        &sub_op_diagnostics,
        operation_name,
        tracking_id,
    ))
}

fn is_container_recreation_error(error: &crate::error::CosmosError) -> bool {
    let status = error.status();
    (status.status_code() == StatusCode::BadRequest
        && status.sub_status() == Some(crate::models::SubStatusCode::COLLECTION_RID_MISMATCH))
        || (status.status_code() == StatusCode::Gone
            && status.sub_status() == Some(crate::models::SubStatusCode::NAME_CACHE_STALE))
        || status.is_read_session_not_available()
}

/// Re-stamps the virtual PATCH operation's canonical `db.operation.name` onto
/// the diagnostics attached to a failure escaping the RMW loop.
///
/// The handler executes 2+ real sub-operations (`read_item` + `replace_item`),
/// so a failure surfaced verbatim from a sub-op would report that sub-op's
/// identity while the matching success and retry-exhaustion paths report
/// `patch_item`. This keeps "one PATCH operation = one `DiagnosticsContext`"
/// true on every exit.
///
/// The wire error itself flows through untouched — status, sub-status, raw
/// response, and source are carried forward by
/// [`CosmosErrorBuilder::from_error`] — only the diagnostics are replaced.
/// `prior_sub_ops` are the contexts accumulated before the failure; when there
/// are any, the failing sub-op's context is aggregated with them so the error
/// carries the whole PATCH attempt history. With a single context there is
/// nothing to aggregate, so it is copied verbatim (preserving hedging
/// diagnostics and compaction metadata) with only the name rewritten. Errors
/// with no diagnostics anywhere are returned unchanged; the operation pipeline
/// grafts the operation-level context onto them on the way out.
fn stamp_patch_identity(
    err: crate::error::CosmosError,
    operation_name: Option<Arc<str>>,
    tracking_id: Option<crate::models::PatchTrackingId>,
    prior_sub_ops: &[Arc<DiagnosticsContext>],
) -> crate::error::CosmosError {
    let mut sources: Vec<Arc<DiagnosticsContext>> = prior_sub_ops.to_vec();
    if let Some(failed) = err.diagnostics() {
        if !sources.iter().any(|source| Arc::ptr_eq(source, &failed)) {
            sources.push(failed);
        }
    }
    let stamped = match sources.as_slice() {
        [] => {
            return match tracking_id {
                Some(id) => crate::error::CosmosErrorBuilder::from_error(err)
                    .with_patch_tracking_id(id)
                    .build(),
                None => err,
            }
        }
        [only] => Arc::new(only.clone_with_operation_name(operation_name)),
        many => match DiagnosticsContext::aggregate_sub_operations(many) {
            Some(ctx) => Arc::new(ctx.with_operation_name(operation_name)),
            // Unreachable: `many` is non-empty. Keep the error intact rather
            // than panicking if that ever changes.
            None => return err,
        },
    };
    let stamped = match tracking_id {
        Some(id) => Arc::new(stamped.as_ref().clone().with_patch_tracking_id(id)),
        None => stamped,
    };
    let mut builder = crate::error::CosmosErrorBuilder::from_error(err).with_diagnostics(stamped);
    if let Some(id) = tracking_id {
        builder = builder.with_patch_tracking_id(id);
    }
    builder.build()
}

fn push_unique_diagnostics(
    diagnostics: &mut Vec<Arc<DiagnosticsContext>>,
    candidate: Option<Arc<DiagnosticsContext>>,
) {
    if let Some(candidate) = candidate {
        if !diagnostics
            .iter()
            .any(|existing| Arc::ptr_eq(existing, &candidate))
        {
            diagnostics.push(candidate);
        }
    }
}

fn aggregate_patch_diagnostics(
    sub_operations: &[Arc<DiagnosticsContext>],
    operation_name: Option<Arc<str>>,
    tracking_id: Option<crate::models::PatchTrackingId>,
) -> Arc<DiagnosticsContext> {
    let diagnostics = DiagnosticsContext::aggregate_sub_operations(sub_operations)
        .map(|context| Arc::new(context.with_operation_name(operation_name)))
        .unwrap_or_else(|| {
            sub_operations
                .last()
                .cloned()
                .expect("PATCH diagnostics are non-empty after a successful sub-operation")
        });
    tracking_id.map_or(diagnostics.clone(), |id| {
        Arc::new(diagnostics.as_ref().clone().with_patch_tracking_id(id))
    })
}

enum VerificationOutcome {
    Applied(Box<CosmosResponse>),
    Absent,
    ReadFailed(crate::error::CosmosError),
}

struct VerificationContext {
    tracking: (
        crate::models::PatchTrackingId,
        std::num::NonZeroU16,
        std::num::NonZeroU32,
    ),
    caller_session_token: Option<crate::models::SessionToken>,
    operation_name: Option<Arc<str>>,
    absolute_deadline: Option<Instant>,
}

async fn verify_committed_patch<D: SubOperationDispatcher + ?Sized>(
    dispatcher: &D,
    item_ref: &crate::models::ItemReference,
    read_options: &OperationOptions,
    context: VerificationContext,
    sub_op_diagnostics: &mut Vec<Arc<DiagnosticsContext>>,
) -> crate::error::Result<VerificationOutcome> {
    let response = match dispatcher
        .execute_operation(
            build_read_sub_op(item_ref.clone(), context.caller_session_token)
                .with_absolute_deadline(context.absolute_deadline),
            read_options.clone(),
        )
        .await
    {
        Ok(response) => response,
        Err(error) => return Ok(VerificationOutcome::ReadFailed(error)),
    };
    let mut headers = response.headers().clone();
    let status = response.status();
    let routing_fallback = response.routing_fallback();
    sub_op_diagnostics.push(response.diagnostics());
    let body = response.into_body().single()?;
    let mut value = serde_json::from_slice::<serde_json::Value>(&body).map_err(|error| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
            .with_message("PATCH could not deserialize verification Read response body")
            .with_source(error)
            .build()
    })?;
    let outcome = prepare_tracking_marker(
        &mut value,
        context.tracking.0.as_uuid(),
        context.tracking.1,
        context.tracking.2,
        false,
    )?;
    if outcome != TrackingMarkerOutcome::AlreadyApplied {
        if routing_fallback {
            return Err(inconclusive_tracking_verification_error(context.tracking.0));
        }
        return Ok(VerificationOutcome::Absent);
    }

    let diagnostics = aggregate_patch_diagnostics(
        sub_op_diagnostics,
        context.operation_name,
        Some(context.tracking.0),
    );
    headers.request_charge = Some(diagnostics.total_request_charge());
    Ok(VerificationOutcome::Applied(Box::new(
        from_local_body_and_driver_headers(body.to_vec(), headers, status, diagnostics),
    )))
}

fn default_patch_tracking_retention_seconds() -> std::num::NonZeroU32 {
    let seconds = u32::try_from(crate::models::PATCH_TRACKING_RETENTION.as_secs())
        .expect("default PATCH tracking retention fits in u32 seconds");
    std::num::NonZeroU32::new(seconds).expect("default PATCH tracking retention is non-zero")
}

fn terminal_error_requires_verification(err: &crate::error::CosmosError) -> bool {
    if err.is_from_wire()
        && matches!(
            err.status().status_code(),
            StatusCode::BadRequest
                | StatusCode::Unauthorized
                | StatusCode::Forbidden
                | StatusCode::PayloadTooLarge
        )
    {
        return false;
    }

    err.diagnostics().is_none_or(|diagnostics| {
        diagnostics.requests().is_empty()
            || diagnostics
                .requests()
                .iter()
                .any(|request| !request.request_sent().definitely_not_sent())
    })
}

fn inconclusive_tracking_verification_error(
    tracking_id: crate::models::PatchTrackingId,
) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::new(
            StatusCode::ServiceUnavailable,
        ))
        .with_message(format!(
            "PATCH tracking verification for '{tracking_id}' was routed away from every usable write endpoint and did not observe the marker; refusing to apply because absence is inconclusive"
        ))
        .build()
}

fn missing_body_error(msg: &'static str) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::new(
            azure_core::http::StatusCode::BadRequest,
        ))
        .with_message(msg)
        .build()
}

fn validate_caller_precondition(
    precondition: Option<&Precondition>,
    current_etag: &Etag,
) -> crate::error::Result<()> {
    let Some(precondition) = precondition else {
        return Ok(());
    };
    let satisfied = match precondition {
        Precondition::IfMatch(expected) => expected.as_ref() == "*" || expected == current_etag,
        Precondition::IfNoneMatch(_) => false,
    };
    if satisfied {
        return Ok(());
    }
    Err(crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::new(
            StatusCode::PreconditionFailed,
        ))
        .with_message("One of the specified pre-conditions is not met.")
        .build())
}

/// Returns `true` if `err` is the driver pipeline's representation of a
/// `412 Precondition Failed` HTTP response (i.e. our ETag-guarded Replace
/// lost the race against a concurrent writer).
///
/// The driver pipeline maps every non-2xx response — 412 included — into
/// an `Err(crate::error::CosmosError)` with `CosmosStatus` via
/// `retry_evaluation::build_http_error`, and 412 specifically resolves
/// to `OperationAction::Abort` (it is never retried at the pipeline layer).
/// The patch handler's RMW loop is the *one* place where 412 needs to be
/// recovered into a retry, so we narrow on the response-presence here
/// instead of relying on a status check that the `await?` above would
/// never reach. Requires a wire response so a future internal
/// constructor that happens to use `StatusCode::PreconditionFailed` for a
/// synthetic error cannot accidentally trigger the RMW retry path.
fn is_precondition_failed(err: &crate::error::CosmosError) -> bool {
    // Use `wire_payload()` (true for both `Wire` and the internal
    // `WirePending` staging state) rather than the narrower public
    // `is_from_wire()` predicate. The patch handler's RMW loop sees
    // sub-op errors fresh out of `driver.execute_operation()` — by that
    // point they are normally `Wire`, but we want the test fixtures (and
    // any future in-pipeline call site) to be able to recognize a
    // service 412 without having to fabricate a full finalized
    // diagnostics context. The status check still narrows to 412.
    err.wire_payload().is_some() && err.status().is_precondition_failed()
}

/// Reconciles the locally-merged post-image JSON with the Replace response so
/// the response body the customer deserializes carries the server's
/// authoritative system properties (`_etag` in particular) instead of the
/// Read's stale ones.
///
/// Preference order:
///
/// 1. If `replace_body` is non-empty, return it verbatim — the service
///    returned the full post-image (i.e., the caller did not disable
///    `content_response_on_write`), and that body is the source of truth.
/// 2. Otherwise, parse `merged_bytes` as a JSON object and overwrite its
///    `_etag` member with `replace_etag` (the value the Replace minted).
///    This is a defensive fallback for explicitly bodyless or anomalously
///    empty Replace responses. The outer driver discards it when the caller
///    disabled response content.
/// 3. If `merged_bytes` is not a JSON object, or `replace_etag` is `None`,
///    or any serde step fails, the merged bytes are returned unchanged —
///    the body in that case is no worse than what the previous
///    implementation produced.
fn synthesize_post_image_body(
    merged_bytes: Vec<u8>,
    replace_body: Vec<u8>,
    replace_etag: Option<&Etag>,
) -> Vec<u8> {
    if !replace_body.is_empty() {
        return replace_body;
    }
    let Some(etag) = replace_etag else {
        return merged_bytes;
    };
    let Ok(mut value) = serde_json::from_slice::<serde_json::Value>(&merged_bytes) else {
        return merged_bytes;
    };
    let serde_json::Value::Object(ref mut map) = value else {
        return merged_bytes;
    };
    map.insert(
        "_etag".to_string(),
        serde_json::Value::String(etag.to_string()),
    );
    serde_json::to_vec(&value).unwrap_or(merged_bytes)
}

/// Builds the internal Read sub-operation used by the RMW loop. The operation
/// hint prefers write endpoints and suppresses hedging.
/// Writer routing strips the caller token because `LatestCommitted` is not
/// session-effective; reader fallback retains it to preserve an external
/// session even when the driver's local session cache has not observed it.
fn build_read_sub_op(
    item_ref: crate::models::ItemReference,
    caller_session_token: Option<crate::models::SessionToken>,
) -> CosmosOperation {
    let mut operation = CosmosOperation::read_item(item_ref).as_patch_read_sub_operation();
    if let Some(token) = caller_session_token {
        operation = operation.with_session_token(token);
    }
    operation
}

/// Builds the internal Replace sub-operation used by the RMW loop. The
/// session token comes from the Read response (NOT the caller's options) so
/// the write commits against the same replica view we just read from. This
/// is the SE-004 TOCTOU mitigation.
fn build_replace_sub_op(
    item_ref: crate::models::ItemReference,
    merged_bytes: Vec<u8>,
    etag: Etag,
    read_response_session_token: Option<crate::models::SessionToken>,
) -> CosmosOperation {
    let mut op = CosmosOperation::replace_item(item_ref)
        .as_patch_sub_operation()
        .with_body(merged_bytes)
        .with_precondition(Precondition::if_match(etag));
    if let Some(token) = read_response_session_token {
        op = op.with_session_token(token);
    }
    op
}

/// Builds the final error returned to callers when the RMW loop exhausted
/// `attempts` retries without ever landing a Replace. When an underlying
/// 412 is supplied it is reused as-is (with the attempts-count message
/// prepended via [`CosmosError::with_context`]) so the typed status, sub-status,
/// cosmos response headers, response body, and diagnostics all flow
/// through verbatim. The `None` branch synthesizes a 412-shaped service
/// error for the `attempts = 0` short-circuit path.
///
/// `sub_op_diagnostics` is the per-attempt diagnostics accumulated by the
/// RMW loop (one entry per Read + one entry per failed Replace). It is
/// aggregated into a single `DiagnosticsContext` and attached to the
/// returned error so callers see "one PATCH operation = one
/// `DiagnosticsContext`" on the error path, matching the success-path
/// contract in `aggregate_sub_operations`. Empty only on the
/// `attempts = 0` short-circuit path, where there is genuinely nothing
/// to aggregate; in that case the synthetic 412 is built with no
/// diagnostics attached and the operation pipeline's abort branch will
/// graft the operation-level diagnostics onto the error via
/// [`CosmosError::with_diagnostics`] before it leaves the pipeline.
fn exhaustion_error(
    attempts: u8,
    last_412: Option<crate::error::CosmosError>,
    sub_op_diagnostics: &[Arc<DiagnosticsContext>],
    operation_name: Option<Arc<str>>,
    tracking_id: Option<crate::models::PatchTrackingId>,
) -> crate::error::CosmosError {
    let message = format!("patch_item: ETag conflict after {attempts} attempts");
    let aggregated = DiagnosticsContext::aggregate_sub_operations(sub_op_diagnostics).map(|ctx| {
        let context = ctx.with_operation_name(operation_name);
        Arc::new(match tracking_id {
            Some(id) => context.with_patch_tracking_id(id),
            None => context,
        })
    });
    let error = match last_412 {
        Some(source) => {
            let mut b = crate::error::CosmosErrorBuilder::from_error(source).with_context(message);
            if let Some(diag) = aggregated {
                b = b.with_diagnostics(diag);
            }
            b.build()
        }
        None => {
            // No prior Replace attempted (e.g. `attempts == 0` short-circuit
            // path) → there genuinely are no per-op diagnostics to aggregate.
            // Build the synthetic 412 directly via the builder; the caller
            // (operation pipeline abort branch) will graft real diagnostics
            // onto the error if any exist by the time it leaves the
            // pipeline. Attach `aggregated` here too in case a future caller
            // seeds `sub_op_diagnostics` without a `last_412` source.
            let mut b = crate::error::CosmosError::builder()
                .with_status(crate::models::CosmosStatus::new(
                    StatusCode::PreconditionFailed,
                ))
                .with_message(message);
            if let Some(diag) = aggregated {
                b = b.with_diagnostics(diag);
            }
            b.build()
        }
    };
    match tracking_id {
        Some(id) => crate::error::CosmosErrorBuilder::from_error(error)
            .with_patch_tracking_id(id)
            .build(),
        None => error,
    }
}

/// Rejects patches that try to mutate the partition key.
///
/// A PATCH that crosses the partition key path can't be implemented safely by
/// a client-side RMW loop — mutating the partition key means the item moves
/// partitions, which can't be done atomically through a Replace. Fail fast
/// rather than silently produce an inconsistent state.
pub(crate) fn validate_partition_key_paths(
    ops: &[PatchOperation],
    item_ref: &crate::models::ItemReference,
) -> crate::error::Result<()> {
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
            PatchOperation::Move { from, .. } => Some(from.as_str()),
            _ => None,
        };
        for path in std::iter::once(dest).chain(from) {
            if path_overlaps_partition_key(path, PATCH_TRACKING_POINTER) {
                return Err(crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::new(
                        azure_core::http::StatusCode::BadRequest,
                    ))
                    .with_message(format!(
                        "PATCH op '{path}' overlaps reserved tracking path \
                         '{PATCH_TRACKING_POINTER}'"
                    ))
                    .build());
            }
            for pk_path in &pk_paths {
                if path_overlaps_partition_key(path, pk_path) {
                    return Err(crate::error::CosmosError::builder()
                        .with_status(crate::error::CosmosStatus::new(
                            azure_core::http::StatusCode::BadRequest,
                        ))
                        .with_message(format!(
                            "PATCH op '{path}' overlaps partition key path '{pk_path}'; \
                             cannot mutate partition key with a client-side Read-Modify-Write"
                        ))
                        .build());
                }
            }
        }
    }
    Ok(())
}

/// Rejects marker-backed PATCH when the reserved tracking property overlaps a
/// partition-key path. Adding or pruning markers would otherwise mutate the
/// item's partition key on every unsafe operation.
fn validate_tracking_partition_key_paths(
    item_ref: &crate::models::ItemReference,
) -> crate::error::Result<()> {
    for pk_path in item_ref.container().partition_key_definition().paths() {
        if path_overlaps_partition_key(PATCH_TRACKING_POINTER, pk_path) {
            return Err(crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message(format!(
                    "unsafe PATCH requires reserved tracking path '{PATCH_TRACKING_POINTER}', \
                     which overlaps partition key path '{pk_path}'"
                ))
                .build());
        }
    }
    Ok(())
}

fn path_overlaps_partition_key(op_path: &str, pk_path: &str) -> bool {
    // Normalize both paths so a caller-supplied op path missing the RFC 6901
    // leading '/' (e.g. "pk" instead of "/pk") still matches a PK path
    // ("/pk"). Without this, the byte-prefix comparison below would silently
    // accept the malformed path here, dispatch the Read sub-op, and only
    // fail later in `apply_patch_ops` via `parse_pointer` — wasting an RU on
    // a request that should have been rejected up front.
    //
    // `parse_pointer` itself rejects empty paths and paths without a leading
    // '/' once we reach the local-apply stage; this normalization closes
    // only the *PK-overlap-guard escape* window, not the broader validation
    // (which is still enforced at apply time).
    fn normalize(p: &str) -> String {
        if p.is_empty() || p.starts_with('/') {
            p.to_string()
        } else {
            format!("/{p}")
        }
    }
    let op = normalize(op_path);
    let pk = normalize(pk_path);
    if op == pk {
        return true;
    }
    // Equal paths overlap; an op path that is an ancestor
    // (e.g., '/account' when PK is '/account/tenantId') also overlaps; an op
    // path that descends into a PK subtree
    // (e.g., '/account/tenantId/extra' on PK '/account/tenantId') also
    // overlaps. The check is symmetric on prefixes split at '/'.
    let with_slash = |p: &str| {
        if p.ends_with('/') {
            p.to_string()
        } else {
            format!("{p}/")
        }
    };
    let a = with_slash(&op);
    let b = with_slash(&pk);
    a.starts_with(&b) || b.starts_with(&a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        AccountReference, ContainerProperties, ContainerReference, ItemReference, OperationType,
        PartitionKey, PartitionKeyDefinition, SessionToken, SystemProperties,
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
    fn path_overlap_normalizes_missing_leading_slash() {
        // A caller-supplied op path missing the RFC 6901 leading '/' must
        // still trip the PK guard. Without the normalization in
        // `path_overlaps_partition_key`, a byte-prefix comparison of "pk"
        // against "/pk" returns false, the PK guard silently accepts it,
        // the handler dispatches a Read sub-op, and the call only fails
        // later in `parse_pointer` — wasting an RU. Pin the fast-fail
        // behavior.
        assert!(path_overlaps_partition_key("pk", "/pk"));
        assert!(path_overlaps_partition_key("pk/inner", "/pk"));
        // The malformed direction is symmetric: a PK path missing the
        // leading slash (shouldn't happen in practice, but the comparator
        // is symmetric) still matches a properly-rooted op path.
        assert!(path_overlaps_partition_key("/pk", "pk"));
        // Sibling paths without leading slashes still don't overlap.
        assert!(!path_overlaps_partition_key("other", "/pk"));
    }

    #[test]
    fn read_sub_op_carries_caller_session_token_for_fallback() {
        let caller_token = SessionToken(Cow::Owned("0:1#7".into()));
        let op = build_read_sub_op(test_item_ref(), Some(caller_token.clone()));

        assert_eq!(op.operation_type(), OperationType::Read);
        assert_eq!(op.request_headers().session_token, Some(caller_token));
    }

    #[test]
    fn read_sub_op_omits_session_token_when_caller_has_none() {
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
        let etag = Etag::from("\"abc\"");
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
        // assert the If-Match precondition was applied. A future refactor
        // that silently dropped `.with_precondition(...)` would downgrade the
        // RMW to a non-conditional Replace — precisely the bug R3-DRIVER's
        // ETag guard exists to prevent.
        assert_eq!(op.precondition(), Some(&Precondition::if_match(etag)));
    }

    #[test]
    fn replace_sub_op_omits_token_when_read_response_has_none() {
        let etag = Etag::from("\"abc\"");
        let op = build_replace_sub_op(test_item_ref(), Vec::new(), etag, None);

        assert_eq!(op.operation_type(), OperationType::Replace);
        assert!(op.request_headers().session_token.is_none());
    }

    #[test]
    fn sub_ops_report_patch_scoped_operation_names() {
        // The RMW sub-ops are dispatched exactly like standalone point
        // operations, so without the marker their telemetry would be
        // indistinguishable from a `read_item` / `replace_item` the caller
        // issued directly. The `patch_` prefix keeps them attributable to the
        // PATCH while still naming which half of the read-modify-write they
        // are.
        let read = build_read_sub_op(test_item_ref(), None);
        assert!(read.is_patch_sub_operation());
        assert_eq!(read.db_operation_name(), Some("patch_read_item"));

        let replace = build_replace_sub_op(
            test_item_ref(),
            b"{\"id\":\"doc1\"}".to_vec(),
            Etag::from("\"abc\""),
            None,
        );
        assert!(replace.is_patch_sub_operation());
        assert_eq!(replace.db_operation_name(), Some("patch_replace_item"));

        // The caller-facing operation keeps its own name; that is what the
        // aggregate context, root span, and operation metric report.
        assert_eq!(canonical_patch_op().db_operation_name(), Some("patch_item"));
    }

    #[test]
    fn is_precondition_failed_matches_real_412() {
        // the RMW loop's 412 detection runs on the `Err(_)` produced
        // by the driver pipeline (`build_service_error`). Build the same
        // shape here.
        let err =
            cosmos_service_error(StatusCode::PreconditionFailed, "412 from server", None, &[]);
        assert!(is_precondition_failed(&err));
    }

    #[test]
    fn is_precondition_failed_rejects_other_http_statuses() {
        for status in [
            StatusCode::NotFound,
            StatusCode::Conflict,
            StatusCode::TooManyRequests,
            StatusCode::ServiceUnavailable,
        ] {
            let err = cosmos_service_error(status, "non-412 service error", None, &[]);
            assert!(
                !is_precondition_failed(&err),
                "should not match status {status:?}",
            );
        }
    }

    #[test]
    fn terminal_verification_considers_every_transport_attempt() {
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::new_uuid(),
            Arc::new(DiagnosticsOptions::default()),
        );
        let endpoint = crate::driver::routing::CosmosEndpoint::global(
            Url::parse("https://test.documents.azure.com/").unwrap(),
        );
        for request_sent in [
            crate::diagnostics::RequestSentStatus::Unknown,
            crate::diagnostics::RequestSentStatus::NotSent,
        ] {
            let handle = diagnostics.start_request(
                crate::diagnostics::ExecutionContext::Initial,
                crate::diagnostics::PipelineType::DataPlane,
                crate::diagnostics::TransportSecurity::Secure,
                crate::diagnostics::TransportKind::Gateway,
                crate::diagnostics::TransportHttpVersion::Http11,
                &endpoint,
            );
            diagnostics.fail_transport_request(
                handle,
                "transport failed",
                request_sent,
                CosmosStatus::TRANSPORT_IO_FAILED,
            );
        }
        let error = crate::error::CosmosError::builder()
            .with_status(CosmosStatus::TRANSPORT_IO_FAILED)
            .with_diagnostics(Arc::new(diagnostics.complete()))
            .build();

        assert!(terminal_error_requires_verification(&error));
    }

    #[test]
    fn is_precondition_failed_rejects_non_http_error_kinds() {
        use crate::error::CosmosError;
        let errs = [
            CosmosError::builder()
                .with_status(crate::error::CosmosStatus::new(
                    azure_core::http::StatusCode::BadRequest,
                ))
                .with_message("synthetic")
                .build(),
            CosmosError::builder()
                .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("bad json")
                .with_source(std::io::Error::new(std::io::ErrorKind::InvalidData, "stub"))
                .build(),
        ];
        for err in &errs {
            assert!(
                !is_precondition_failed(err),
                "should not match {:?}",
                err.status()
            );
        }
    }

    #[test]
    fn pk_guard_rejects_move_from_pk_path() {
        // moving FROM a PK path mutates the partition key (the field
        // is removed after being copied to the destination), so the
        // preflight guard must reject it just like a move TO a PK path.
        // Reuses the `/pk` flat PK fixture.
        let item_ref = test_item_ref();
        let ops = vec![PatchOperation::move_value("/pk", "/somewhere_else")];

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
            id: "multi_hash_container".into(),
            partition_key: pk_def,
            system_properties: SystemProperties::default(),
        };
        let container = ContainerReference::new(
            test_account(),
            "testdb",
            "testdb_rid",
            "multi_hash_container",
            "multi_hash_container_rid",
            &props,
        );
        let item_ref =
            ItemReference::from_name(&container, PartitionKey::from(("t1", "r1", "u1")), "doc1");

        let ops = vec![PatchOperation::move_value("/tenant", "/somewhere_else")];

        let err = validate_partition_key_paths(&ops, &item_ref)
            .expect_err("MoveOp from /tenant on a hierarchical PK must be rejected");
        let msg = format!("{err}").to_ascii_lowercase();
        assert!(
            msg.contains("partition key"),
            "error should mention partition key; got: {err}"
        );
    }

    #[test]
    fn tracking_guard_rejects_overlapping_partition_key_paths() {
        for pk_path in ["/_azsdkPatchTracking", "/_azsdkPatchTracking/tenant", "/"] {
            let props = ContainerProperties {
                id: "tracking_pk_container".into(),
                partition_key: test_partition_key_definition(pk_path),
                system_properties: SystemProperties::default(),
            };
            let container = ContainerReference::new(
                test_account(),
                "testdb",
                "testdb_rid",
                "tracking_pk_container",
                "tracking_pk_container_rid",
                &props,
            );
            let item_ref = ItemReference::from_name(&container, PartitionKey::from("pk1"), "doc1");

            let err = validate_tracking_partition_key_paths(&item_ref)
                .expect_err("tracking and partition-key paths must not overlap");

            assert_eq!(err.status().status_code(), StatusCode::BadRequest);
            assert!(err.to_string().contains("reserved tracking path"));
        }
    }

    // ====== exhaustion_error coverage ======

    #[test]
    fn exhaustion_error_with_source_chains_underlying_412() {
        // Closes the loop where the RMW gives up: the final `Err` returned to
        // the caller must (a) be a 412-shaped service error, (b) carry the
        // attempts count in its message, and (c) keep the underlying 412's
        // typed payload (response body, headers) accessible via the cosmos
        // accessors so callers do not need to walk std::error::Error::source
        // to recover them.
        let underlying = cosmos_service_error(
            StatusCode::PreconditionFailed,
            "ETag mismatch from server",
            None,
            b"server-body",
        );
        let err = exhaustion_error(
            7,
            Some(underlying),
            &[],
            Some(Arc::from("patch_item")),
            None,
        );

        // (a) Shape.
        assert_eq!(
            err.status().status_code(),
            StatusCode::PreconditionFailed,
            "exhaustion error must surface as a 412; got {:?}",
            err.status()
        );
        // (b) Message carries the attempts count and the underlying detail
        //     (with_context prefixes the attempts message onto the source).
        let msg = format!("{err}");
        assert!(
            msg.contains("7"),
            "exhaustion message should mention the attempts count: {msg}"
        );
        assert!(
            msg.to_ascii_lowercase().contains("etag")
                || msg.to_ascii_lowercase().contains("conflict"),
            "exhaustion message should mention ETag conflict: {msg}"
        );
        assert!(
            msg.contains("ETag mismatch from server"),
            "exhaustion message should still surface the underlying detail: {msg}"
        );
        // (c) Typed payload from the underlying 412 is preserved verbatim.
        assert_eq!(
            err.wire_payload().and_then(|p| match p.body() {
                crate::models::ResponseBody::Bytes(b) => Some(b.as_ref()),
                _ => None,
            }),
            Some(b"server-body".as_slice())
        );
    }

    #[test]
    fn exhaustion_error_without_source_is_still_412_shaped() {
        // If the loop somehow exits without ever observing a real 412 (e.g.
        // `attempts = 0` short-circuit), we still want the caller to see a
        // 412-shaped error so they can recognize "we gave up" the same way
        // they would for any other PATCH retry exhaustion.
        let id = tracking_id(42);
        let err = exhaustion_error(0, None, &[], Some(Arc::from("patch_item")), Some(id));

        assert_eq!(err.status().status_code(), StatusCode::PreconditionFailed);
        assert_eq!(err.patch_tracking_id(), Some(id));
        assert!(err.diagnostics().is_none());
        // No underlying service error was supplied, so the synthesized
        // error has no further std::error::Error source chain.
        assert!(
            std::error::Error::source(&err).is_none(),
            "exhaustion_error must NOT synthesize a source when none was passed"
        );
        let msg = format!("{err}");
        assert!(
            msg.contains("0"),
            "exhaustion message should still mention the attempts count: {msg}"
        );
    }

    #[test]
    fn exhaustion_error_forwards_underlying_response_body_and_headers() {
        // The top-level exhaustion error must expose the same typed payload
        // as the wrapped 412, so callers reading `err.wire_payload().and_then(|p| match p.body() { crate::models::ResponseBody::Bytes(b) => Some(b.as_ref()), _ => None })` /
        // `err.wire_payload().map(|p| p.headers())` see a consistent shape — exactly like any
        // other 412 path in this SDK.
        let underlying = cosmos_service_error(
            StatusCode::PreconditionFailed,
            "ETag mismatch from server",
            Some("0:1#42"),
            b"{\"code\":\"PreconditionFailed\",\"message\":\"server: stale etag\"}",
        );
        let err = exhaustion_error(
            4,
            Some(underlying),
            &[],
            Some(Arc::from("patch_item")),
            None,
        );

        assert_eq!(err.status().status_code(), StatusCode::PreconditionFailed);
        assert_eq!(
            err.wire_payload().and_then(|p| match p.body() {
                crate::models::ResponseBody::Bytes(b) => Some(b.as_ref()),
                _ => None,
            }),
            Some(
                b"{\"code\":\"PreconditionFailed\",\"message\":\"server: stale etag\"}".as_slice()
            ),
            "exhaustion error must forward the wrapped 412's response body verbatim"
        );
        assert_eq!(
            err.wire_payload()
                .map(|p| p.headers())
                .and_then(|h| h.session_token.as_ref())
                .map(|t| t.0.as_ref()),
            Some("0:1#42"),
            "exhaustion error must forward the wrapped 412's session token"
        );
    }

    #[test]
    fn exhaustion_error_attaches_aggregated_sub_op_diagnostics() {
        // Regression guard: when the RMW loop gives up after multiple
        // attempts, the returned error must carry the aggregated
        // per-attempt `DiagnosticsContext` (Reads + failed Replaces), not
        // a default/empty context or the source-only single-attempt view.
        // Triage tooling reads `err.diagnostics().request_count()` and
        // must see the real per-attempt history.
        let underlying = cosmos_service_error(
            StatusCode::PreconditionFailed,
            "ETag mismatch from server",
            None,
            b"server-body",
        );
        // Four synthetic per-attempt contexts standing in for what the
        // RMW loop accumulates. Each one carries a real (completed)
        // request entry so the aggregation is observably correct — the
        // expected `request_count` is the sum of inputs, not zero.
        let attempt_diags: Vec<Arc<DiagnosticsContext>> = (0..4)
            .map(|_| {
                let mut builder = DiagnosticsContextBuilder::new(
                    crate::models::ActivityId::new_uuid(),
                    Arc::new(crate::options::DiagnosticsOptions::default()),
                );
                let handle = builder.start_request(
                    crate::diagnostics::ExecutionContext::Initial,
                    crate::diagnostics::PipelineType::DataPlane,
                    crate::diagnostics::TransportSecurity::Secure,
                    crate::diagnostics::TransportKind::Gateway,
                    crate::diagnostics::TransportHttpVersion::Http11,
                    &crate::driver::routing::CosmosEndpoint::global(
                        url::Url::parse("https://test.documents.azure.com/").unwrap(),
                    ),
                );
                builder.complete_request(handle, StatusCode::PreconditionFailed, None);
                Arc::new(builder.complete())
            })
            .collect();
        let err = exhaustion_error(
            2,
            Some(underlying),
            &attempt_diags,
            Some(Arc::from("patch_item")),
            None,
        );

        let diag = err
            .diagnostics()
            .expect("exhaustion error must carry an aggregated DiagnosticsContext");
        assert_eq!(
            diag.request_count(),
            4,
            "aggregated diagnostics must concatenate every per-attempt RequestDiagnostics",
        );
        assert_eq!(
            diag.operation_name(),
            Some("patch_item"),
            "aggregated PATCH diagnostics must carry the virtual operation's own name",
        );
        // And critically, the attached diagnostics must be distinct from
        // every input Arc — the aggregator returns a fresh context.
        for input in &attempt_diags {
            assert!(
                !Arc::ptr_eq(&diag, input),
                "exhaustion error must surface the aggregated context, not any input Arc",
            );
        }
    }

    // ====== Dispatcher-driven loop coverage ======
    //
    // These tests close the gap left by the predicate-only `is_precondition_failed`
    // tests: they drive the *real* RMW loop end-to-end through the
    // `SubOperationDispatcher` seam, so a regression that handled 412 in the
    // `Ok(_)` arm (the bug this PR fixes) or that issued the Read AFTER the PK
    // guard (rather than before) will fail loudly here — without needing a
    // live emulator.

    use crate::diagnostics::{DiagnosticsContextBuilder, RequestEventType};
    use crate::models::{ActivityId, CosmosResponseHeaders, CosmosStatus, RequestCharge};
    use crate::options::{BinaryEncodingOptions, DiagnosticsOptions};
    use std::sync::{Arc, Mutex};

    /// A pre-baked response a [`ScriptedDispatcher`] returns for a single
    /// sub-operation. `Ok` becomes a [`CosmosResponse`]; `Err` is returned
    /// verbatim — so tests can inject a service-side 412 just like the
    /// driver pipeline would.
    enum ScriptedReply {
        Ok {
            body: Vec<u8>,
            etag: Option<&'static str>,
            session_token: Option<&'static str>,
            status: StatusCode,
        },
        OkWithRoutingFallback {
            body: Vec<u8>,
            etag: Option<&'static str>,
            status: StatusCode,
        },
        Err(crate::error::CosmosError),
    }

    impl ScriptedReply {
        /// Convenience constructor for an `Ok` reply with no session token —
        /// the most common shape used in the existing tests.
        fn ok(body: Vec<u8>, etag: Option<&'static str>, status: StatusCode) -> Self {
            ScriptedReply::Ok {
                body,
                etag,
                session_token: None,
                status,
            }
        }

        fn fallback(body: Vec<u8>, etag: Option<&'static str>, status: StatusCode) -> Self {
            Self::OkWithRoutingFallback { body, etag, status }
        }
    }

    /// Records every (operation_type, etag-on-precondition, body) the PATCH
    /// loop dispatches, and replays a fixed script of responses.
    struct ScriptedDispatcher {
        script: Mutex<Vec<ScriptedReply>>,
        calls: Mutex<Vec<DispatchedCall>>,
    }

    #[derive(Debug, Clone)]
    struct DispatchedCall {
        op_type: OperationType,
        /// The If-Match precondition's ETag, if one was set. The PATCH handler
        /// MUST set this on the Replace; absence here means the ETag guard was
        /// dropped, which would be a regression.
        if_match_etag: Option<String>,
        /// The session token applied to the dispatched sub-op's request
        /// headers, if any. Captured so tests can pin the cross-attempt
        /// session-token carry-forward behavior.
        session_token: Option<SessionToken>,
        body: Option<Vec<u8>>,
        read_consistency_strategy: Option<ReadConsistencyStrategy>,
        content_response_on_write: Option<ContentResponseOnWrite>,
        prefers_write_endpoints_for_read: bool,
        suppresses_hedging: bool,
        absolute_deadline: Option<Instant>,
    }

    impl ScriptedDispatcher {
        fn new(script: Vec<ScriptedReply>) -> Self {
            Self {
                script: Mutex::new(script),
                calls: Mutex::new(Vec::new()),
            }
        }

        fn calls(&self) -> Vec<DispatchedCall> {
            self.calls.lock().unwrap().clone()
        }
    }

    #[async_trait]
    impl SubOperationDispatcher for ScriptedDispatcher {
        async fn execute_operation(
            &self,
            operation: CosmosOperation,
            options: OperationOptions,
        ) -> crate::error::Result<CosmosResponse> {
            let operation_type = operation.operation_type();
            let if_match = match operation.precondition() {
                Some(Precondition::IfMatch(tag)) => Some(tag.as_ref().to_string()),
                _ => None,
            };
            self.calls.lock().unwrap().push(DispatchedCall {
                op_type: operation_type,
                if_match_etag: if_match,
                session_token: operation.request_headers().session_token.clone(),
                body: operation.body().map(<[u8]>::to_vec),
                read_consistency_strategy: options.read_consistency_strategy,
                content_response_on_write: options.content_response_on_write,
                prefers_write_endpoints_for_read: operation.prefers_write_endpoints_for_read(),
                suppresses_hedging: operation.suppresses_hedging(),
                absolute_deadline: operation.absolute_deadline(),
            });

            let reply =
                self.script.lock().unwrap().drain(..1).next().expect(
                    "ScriptedDispatcher exhausted: PATCH loop made more sub-ops than scripted",
                );

            match reply {
                ScriptedReply::Err(e) => Err(e),
                ScriptedReply::Ok {
                    body,
                    etag,
                    session_token,
                    status,
                } => scripted_response(
                    body,
                    etag,
                    session_token,
                    status,
                    false,
                    operation_type == OperationType::Read,
                ),
                ScriptedReply::OkWithRoutingFallback { body, etag, status } => {
                    scripted_response(body, etag, None, status, true, true)
                }
            }
        }
    }

    fn scripted_response(
        body: Vec<u8>,
        etag: Option<&'static str>,
        session_token: Option<&'static str>,
        status: StatusCode,
        routing_fallback: bool,
        inject_document_timestamp: bool,
    ) -> crate::error::Result<CosmosResponse> {
        let body = match (
            inject_document_timestamp,
            serde_json::from_slice::<serde_json::Value>(&body),
        ) {
            (true, Ok(mut value)) => {
                if let Some(object) = value.as_object_mut() {
                    object.entry("_ts").or_insert_with(|| {
                        serde_json::Value::from(
                            time::OffsetDateTime::now_utc().unix_timestamp().max(0),
                        )
                    });
                    serde_json::to_vec(&value).expect("scripted response body must serialize")
                } else {
                    body
                }
            }
            _ => body,
        };
        let mut headers = CosmosResponseHeaders::new();
        if let Some(tag) = etag {
            headers.etag = Some(Etag::from(tag));
        }
        if let Some(token) = session_token {
            headers.session_token = Some(SessionToken(Cow::Owned(token.into())));
        }
        headers.request_charge = Some(RequestCharge::new(1.0));
        let mut diagnostics = DiagnosticsContextBuilder::new(
            ActivityId::new_uuid(),
            Arc::new(DiagnosticsOptions::default()),
        );
        if routing_fallback {
            let endpoint = crate::driver::routing::CosmosEndpoint::global(
                Url::parse("https://fallback.documents.azure.com/").unwrap(),
            );
            let handle = diagnostics.start_request(
                crate::diagnostics::ExecutionContext::Initial,
                crate::diagnostics::PipelineType::DataPlane,
                crate::diagnostics::TransportSecurity::Secure,
                crate::diagnostics::TransportKind::Gateway,
                crate::diagnostics::TransportHttpVersion::Http11,
                &endpoint,
            );
            diagnostics.add_event(
                handle,
                crate::diagnostics::RequestEvent::new(RequestEventType::RoutingFallback),
            );
            diagnostics.complete_request(handle, status, None);
        }
        Ok(from_local_body_and_driver_headers(
            body,
            headers,
            CosmosStatus::from_parts(status, None),
            Arc::new(diagnostics.complete()),
        )
        .with_routing_fallback(routing_fallback))
    }

    /// Builds a real cosmos `CosmosError::service_from_parts` for a non-2xx HTTP
    /// status, just like the production driver pipeline would (see
    /// `retry_evaluation::build_service_error`). Using the same
    /// constructor as production exercises the same accessors
    /// (`err.wire_payload().map(|p| p.headers())`, `err.wire_payload().and_then(|p| match p.body() { crate::models::ResponseBody::Bytes(b) => Some(b.as_ref()), _ => None })`,
    /// `err.status().sub_status()`) that callers see at runtime.
    fn http_error(status: StatusCode, msg: &'static str) -> crate::error::CosmosError {
        cosmos_service_error(status, msg, None, &[])
    }

    /// Same as [`http_error`], optionally populating response headers and body
    /// for tests that verify service-error payload preservation.
    fn cosmos_service_error(
        status: StatusCode,
        msg: &'static str,
        session_token: Option<&'static str>,
        body: &[u8],
    ) -> crate::error::CosmosError {
        let mut headers = CosmosResponseHeaders::new();
        if let Some(token) = session_token {
            headers.session_token = Some(SessionToken(Cow::Owned(token.into())));
        }
        // Match the production shape: the operation pipeline's abort
        // branch always promotes the per-attempt `WirePending` error
        // into a finalized `Wire` error by attaching the completed
        // operation diagnostics (see `execute_operation_pipeline`'s
        // abort arm). Without this, the test fixture would build a
        // `WirePending` error that does not exercise the same
        // `CosmosErrorBuilder` rules production callers hit when
        // they re-decorate the error (notably `exhaustion_error`,
        // which graft-overrides diagnostics on a Wire base).
        let diagnostics = Arc::new(
            crate::diagnostics::DiagnosticsContextBuilder::new(
                crate::models::ActivityId::new_uuid(),
                Arc::new(crate::options::DiagnosticsOptions::default()),
            )
            .complete(),
        );
        crate::error::CosmosError::builder()
            .with_status(CosmosStatus::new(status))
            .with_message(msg)
            .with_response_parts(crate::models::CosmosResponsePayload::new(
                body.to_vec(),
                headers,
            ))
            .with_diagnostics(diagnostics)
            .build()
    }

    fn patch_op_for(item_ref: ItemReference, ops: Vec<PatchOperation>) -> CosmosOperation {
        let body = serde_json::to_vec(&PatchInstructions::from(ops)).unwrap();
        CosmosOperation::patch_item(item_ref).with_body(body)
    }

    /// Builds the canonical (`/pk`, `pk1`, `doc1`) PATCH operation used by
    /// all of these tests — `+1` on `/visits`.
    fn canonical_patch_op() -> CosmosOperation {
        patch_op_for(
            test_item_ref(),
            vec![PatchOperation::increment("/visits", 1i64)],
        )
    }

    fn tracking_id(value: u128) -> crate::models::PatchTrackingId {
        crate::models::PatchTrackingId::from(uuid::Uuid::from_u128(value))
    }

    fn marker_entry(id: crate::models::PatchTrackingId, attempted_at: i64) -> serde_json::Value {
        serde_json::json!({
            "trackingId": id.to_string(),
            "attemptedAt": attempted_at,
            "retentionSeconds": crate::models::PATCH_TRACKING_RETENTION.as_secs(),
        })
    }

    fn dispatched_body(call: &DispatchedCall) -> serde_json::Value {
        serde_json::from_slice(call.body.as_deref().expect("call must carry a body"))
            .expect("dispatched body must be JSON")
    }

    fn marker_ids(document: &serde_json::Value) -> Vec<&str> {
        document[crate::driver::pipeline::patch_tracking::PATCH_TRACKING_PROPERTY]
            .as_array()
            .expect("tracking property must be an array")
            .iter()
            .map(|entry| {
                entry["trackingId"]
                    .as_str()
                    .expect("trackingId must be a string")
            })
            .collect()
    }

    #[tokio::test]
    async fn committed_replace_with_lost_response_is_applied_once() {
        let id = tracking_id(1);
        let committed = serde_json::json!({
            "id": "doc1",
            "pk": "pk1",
            "visits": 1,
            crate::driver::pipeline::patch_tracking::PATCH_TRACKING_PROPERTY: [
                marker_entry(id, 1)
            ]
        });
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            // Models the retry of a committed Replace returning 412 after the
            // original response was lost.
            ScriptedReply::Err(http_error(StatusCode::PreconditionFailed, "lost response")),
            ScriptedReply::ok(
                serde_json::to_vec(&committed).unwrap(),
                Some("\"v2\""),
                StatusCode::Ok,
            ),
        ]);

        let response = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op()
                .with_patch_tracking_id(id)
                .with_precondition(Precondition::if_match(Etag::from("\"v1\""))),
            OperationOptions::default(),
            None,
        )
        .await
        .expect("marker recognition must precede the now-stale caller If-Match");

        let body: serde_json::Value = response.into_body().into_single().unwrap();
        assert_eq!(body["visits"], 1);
        assert_eq!(dispatcher.calls().len(), 3, "no second Replace is allowed");
        let first_replace = dispatched_body(&dispatcher.calls()[1]);
        assert_eq!(first_replace["visits"], 1);
        assert_eq!(marker_ids(&first_replace), vec![id.to_string()]);
    }

    #[tokio::test]
    async fn genuine_concurrent_writer_reapplies_once_and_preserves_markers() {
        let id = tracking_id(1);
        let other_id = tracking_id(2);
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        let concurrent = serde_json::json!({
            "id": "doc1",
            "pk": "pk1",
            "visits": 10,
            crate::driver::pipeline::patch_tracking::PATCH_TRACKING_PROPERTY: [
                marker_entry(other_id, now)
            ]
        });
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(http_error(
                StatusCode::PreconditionFailed,
                "concurrent writer",
            )),
            ScriptedReply::ok(
                serde_json::to_vec(&concurrent).unwrap(),
                Some("\"v2\""),
                StatusCode::Ok,
            ),
            ScriptedReply::ok(Vec::new(), Some("\"v3\""), StatusCode::Ok),
        ]);

        let response = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op().with_patch_tracking_id(id),
            OperationOptions::default(),
            None,
        )
        .await
        .expect("marker absence on a write-region read proves a genuine race");

        let body: serde_json::Value = response.into_body().into_single().unwrap();
        assert_eq!(body["visits"], 11);
        let second_replace = dispatched_body(&dispatcher.calls()[3]);
        assert_eq!(second_replace["visits"], 11);
        assert_eq!(
            marker_ids(&second_replace),
            vec![other_id.to_string(), id.to_string()]
        );
    }

    #[tokio::test]
    async fn cooperating_writer_on_top_does_not_hide_committed_marker() {
        let id = tracking_id(1);
        let other_id = tracking_id(2);
        let current = serde_json::json!({
            "id": "doc1",
            "pk": "pk1",
            "visits": 1,
            "name": "changed-after-commit",
            crate::driver::pipeline::patch_tracking::PATCH_TRACKING_PROPERTY: [
                marker_entry(id, 1),
                marker_entry(other_id, 2)
            ]
        });
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(http_error(StatusCode::PreconditionFailed, "lost response")),
            ScriptedReply::ok(
                serde_json::to_vec(&current).unwrap(),
                Some("\"v3\""),
                StatusCode::Ok,
            ),
        ]);

        let response = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op().with_patch_tracking_id(id),
            OperationOptions::default(),
            None,
        )
        .await
        .expect("our marker must survive a cooperating writer");

        let body: serde_json::Value = response.into_body().into_single().unwrap();
        assert_eq!(body["visits"], 1);
        assert_eq!(body["name"], "changed-after-commit");
        assert_eq!(dispatcher.calls().len(), 3);
    }

    #[tokio::test]
    async fn generated_tracking_id_is_stable_across_rmw_attempts() {
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(http_error(StatusCode::PreconditionFailed, "race")),
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":10}"#.to_vec(),
                Some("\"v2\""),
                StatusCode::Ok,
            ),
            ScriptedReply::ok(Vec::new(), Some("\"v3\""), StatusCode::Ok),
        ]);

        let response = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op(),
            OperationOptions::default(),
            None,
        )
        .await
        .unwrap();

        let calls = dispatcher.calls();
        let first = marker_ids(&dispatched_body(&calls[1]))[0].to_owned();
        let second = marker_ids(&dispatched_body(&calls[3]))[0].to_owned();
        assert_eq!(first, second);
        let effective_id = response
            .patch_tracking_id()
            .expect("tracked PATCH response exposes its generated ID");
        assert_eq!(effective_id.to_string(), first);
        assert_eq!(
            response.diagnostics().patch_tracking_id(),
            Some(effective_id)
        );
        let diagnostics_json: serde_json::Value = serde_json::from_str(
            response
                .diagnostics()
                .to_json_string(Some(crate::options::DiagnosticsVerbosity::Detailed)),
        )
        .unwrap();
        assert_eq!(diagnostics_json["patch_tracking_id"], first);
    }

    #[tokio::test]
    async fn full_unexpired_marker_list_evicts_oldest_before_replace() {
        let id = tracking_id(3);
        let now = time::OffsetDateTime::now_utc().unix_timestamp();
        let document = serde_json::json!({
            "id": "doc1",
            "pk": "pk1",
            "visits": 0,
            crate::driver::pipeline::patch_tracking::PATCH_TRACKING_PROPERTY: [
                marker_entry(tracking_id(1), now),
                marker_entry(tracking_id(2), now)
            ]
        });
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                serde_json::to_vec(&document).unwrap(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::ok(Vec::new(), Some("\"v2\""), StatusCode::Ok),
        ]);

        execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op()
                .with_patch_tracking_id(id)
                .with_patch_tracking_capacity(std::num::NonZeroU16::new(2).unwrap()),
            OperationOptions::default(),
            None,
        )
        .await
        .expect("capacity pressure must evict the oldest marker");

        let calls = dispatcher.calls();
        assert_eq!(calls.len(), 2);
        assert_eq!(
            marker_ids(&dispatched_body(&calls[1])),
            vec![tracking_id(2).to_string(), id.to_string()]
        );
    }

    #[tokio::test]
    async fn custom_tracking_retention_is_persisted_on_marker() {
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::ok(Vec::new(), Some("\"v2\""), StatusCode::Ok),
        ]);

        execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op()
                .with_patch_tracking_retention_seconds(std::num::NonZeroU32::new(17).unwrap()),
            OperationOptions::default(),
            None,
        )
        .await
        .expect("PATCH with custom marker retention succeeds");

        let replace = dispatched_body(&dispatcher.calls()[1]);
        assert_eq!(
            replace[crate::driver::pipeline::patch_tracking::PATCH_TRACKING_PROPERTY][0]
                ["retentionSeconds"],
            17
        );
    }

    #[tokio::test]
    async fn retry_safe_patch_does_not_write_tracking_property() {
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","name":"before"}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::ok(Vec::new(), Some("\"v2\""), StatusCode::Ok),
        ]);
        let operation = patch_op_for(
            test_item_ref(),
            vec![PatchOperation::set("/name", serde_json::json!("after"))],
        );

        execute_with_dispatcher(&dispatcher, operation, OperationOptions::default(), None)
            .await
            .unwrap();

        let replace = dispatched_body(&dispatcher.calls()[1]);
        assert_eq!(replace["name"], "after");
        assert!(replace
            .get(crate::driver::pipeline::patch_tracking::PATCH_TRACKING_PROPERTY)
            .is_none());
    }

    #[tokio::test]
    async fn caller_supplied_id_tracks_retry_safe_patch() {
        let tracking_id = tracking_id(1);
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","name":"before"}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::ok(Vec::new(), Some("\"v2\""), StatusCode::Ok),
        ]);
        let operation = patch_op_for(
            test_item_ref(),
            vec![PatchOperation::set("/name", serde_json::json!("after"))],
        )
        .with_patch_tracking_id(tracking_id);

        let response =
            execute_with_dispatcher(&dispatcher, operation, OperationOptions::default(), None)
                .await
                .expect("explicit ID opts a retry-safe PATCH into tracking");

        let replace = dispatched_body(&dispatcher.calls()[1]);
        assert_eq!(replace["name"], "after");
        assert_eq!(marker_ids(&replace), vec![tracking_id.to_string()]);
        assert_eq!(response.patch_tracking_id(), Some(tracking_id));
    }

    #[tokio::test]
    async fn caller_supplied_id_deduplicates_retry_safe_patch() {
        let tracking_id = tracking_id(1);
        let document = serde_json::json!({
            "id": "doc1",
            "pk": "pk1",
            "name": "after",
            "_ts": 10_000,
            crate::driver::pipeline::patch_tracking::PATCH_TRACKING_PROPERTY: [
                marker_entry(tracking_id, 10_000)
            ]
        });
        let dispatcher = ScriptedDispatcher::new(vec![ScriptedReply::ok(
            serde_json::to_vec(&document).unwrap(),
            Some("\"v2\""),
            StatusCode::Ok,
        )]);
        let operation = patch_op_for(
            test_item_ref(),
            vec![PatchOperation::set("/name", serde_json::json!("after"))],
        )
        .with_patch_tracking_id(tracking_id);

        let response =
            execute_with_dispatcher(&dispatcher, operation, OperationOptions::default(), None)
                .await
                .expect("existing explicit marker suppresses an application retry");

        assert_eq!(dispatcher.calls().len(), 1);
        assert_eq!(response.patch_tracking_id(), Some(tracking_id));
    }

    #[tokio::test]
    async fn fallback_read_requires_positive_marker_proof() {
        let id = tracking_id(1);
        let missing = ScriptedDispatcher::new(vec![ScriptedReply::fallback(
            br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
            Some("\"v1\""),
            StatusCode::Ok,
        )]);

        let error = execute_with_dispatcher(
            &missing,
            canonical_patch_op().with_patch_tracking_id(id),
            OperationOptions::default(),
            None,
        )
        .await
        .expect_err("marker absence on degraded routing is inconclusive");
        assert_eq!(error.status().status_code(), StatusCode::ServiceUnavailable);
        assert_eq!(missing.calls().len(), 1);

        let present_document = serde_json::json!({
            "id": "doc1",
            "pk": "pk1",
            "visits": 1,
            crate::driver::pipeline::patch_tracking::PATCH_TRACKING_PROPERTY: [
                marker_entry(id, 1)
            ]
        });
        let present = ScriptedDispatcher::new(vec![ScriptedReply::fallback(
            serde_json::to_vec(&present_document).unwrap(),
            Some("\"v2\""),
            StatusCode::Ok,
        )]);

        let response = execute_with_dispatcher(
            &present,
            canonical_patch_op().with_patch_tracking_id(id),
            OperationOptions::default(),
            None,
        )
        .await
        .expect("marker presence remains conclusive on degraded routing");
        let body: serde_json::Value = response.into_body().into_single().unwrap();
        assert_eq!(body["visits"], 1);
        assert_eq!(present.calls().len(), 1);
    }

    #[tokio::test]
    async fn fresh_generated_tracking_id_can_start_from_fallback_read() {
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::fallback(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::ok(Vec::new(), Some("\"v2\""), StatusCode::Ok),
        ]);

        let response = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op(),
            OperationOptions::default(),
            None,
        )
        .await
        .expect("a fresh generated ID cannot have committed before the first read");

        let body: serde_json::Value = response.into_body().into_single().unwrap();
        assert_eq!(body["visits"], 1);
        assert_eq!(dispatcher.calls().len(), 2);
    }

    #[tokio::test]
    async fn fallback_marker_absence_after_replace_dispatch_remains_inconclusive() {
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(http_error(StatusCode::PreconditionFailed, "ambiguous")),
            ScriptedReply::fallback(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v2\""),
                StatusCode::Ok,
            ),
        ]);

        let error = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op(),
            OperationOptions::default(),
            None,
        )
        .await
        .expect_err("fallback absence cannot disprove an earlier ambiguous commit");

        assert_eq!(error.status().status_code(), StatusCode::ServiceUnavailable);
        assert_eq!(dispatcher.calls().len(), 3);
    }

    #[tokio::test]
    async fn rmw_recovers_from_412_on_first_replace() {
        // Gap #1 closure: a service-side 412 on the first Replace must drive
        // the loop back to step 2 (Read again) — not be returned to the
        // caller, and not be silently treated as a success.
        //
        // Script: Read#1 ok -> Replace#1 412 -> Read#2 ok -> Replace#2 ok.
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(http_error(StatusCode::PreconditionFailed, "lost the race")),
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":1}"#.to_vec(),
                Some("\"v2\""),
                StatusCode::Ok,
            ),
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":2}"#.to_vec(),
                Some("\"v3\""),
                StatusCode::Ok,
            ),
        ]);

        let resp = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op(),
            OperationOptions::default(),
            None,
        )
        .await
        .expect("PATCH must succeed after a single 412 retry");

        // The handler synthesizes the final response from the post-image
        // it computed locally on attempt #2 (visits=1 + 1 = 2).
        let body: serde_json::Value = resp.into_body().into_single().unwrap();
        assert_eq!(body["visits"], serde_json::json!(2));

        let calls = dispatcher.calls();
        assert_eq!(
            calls.len(),
            4,
            "expected exactly Read,Replace,Read,Replace; got: {calls:?}"
        );
        assert_eq!(calls[0].op_type, OperationType::Read);
        assert_eq!(
            calls[0].read_consistency_strategy,
            Some(ReadConsistencyStrategy::LatestCommitted)
        );
        assert!(calls[0].prefers_write_endpoints_for_read);
        assert!(calls[0].suppresses_hedging);
        assert_eq!(calls[1].op_type, OperationType::Replace);
        assert_eq!(calls[1].read_consistency_strategy, None);
        assert!(!calls[1].prefers_write_endpoints_for_read);
        assert!(!calls[1].suppresses_hedging);
        // Each Replace MUST be If-Match guarded — the ETag guard is the
        // entire reason the RMW is safe under concurrent writers.
        assert_eq!(calls[1].if_match_etag.as_deref(), Some("\"v1\""));
        assert_eq!(calls[2].op_type, OperationType::Read);
        assert_eq!(
            calls[2].read_consistency_strategy,
            Some(ReadConsistencyStrategy::LatestCommitted)
        );
        assert!(calls[2].prefers_write_endpoints_for_read);
        assert!(calls[2].suppresses_hedging);
        assert_eq!(calls[3].op_type, OperationType::Replace);
        // The second Replace MUST use the *new* ETag returned by the second
        // Read — not stash the old one.
        assert_eq!(calls[3].if_match_etag.as_deref(), Some("\"v2\""));
    }

    #[tokio::test]
    async fn rmw_propagates_412_after_exhausting_max_attempts() {
        // Gap #1 closure (other half): after `max_attempts` failed Replaces
        // we surface the 412 with the chained source — not a synthetic
        // success.
        let dispatcher = ScriptedDispatcher::new(vec![
            // Attempt 1
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(http_error(StatusCode::PreconditionFailed, "412 #1")),
            // Attempt 2
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v2\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(http_error(StatusCode::PreconditionFailed, "412 #2")),
            // Attempt 3
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v3\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(http_error(StatusCode::PreconditionFailed, "412 #3")),
            // Final verification does not find the marker, so the original
            // exhaustion error must still be returned.
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v4\""),
                StatusCode::Ok,
            ),
        ]);

        let err = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op(),
            OperationOptions::default(),
            Some(NonZeroU8::new(3).unwrap()),
        )
        .await
        .expect_err("PATCH must fail after exhausting attempts");

        assert!(
            is_precondition_failed(&err),
            "final error must be 412-shaped; got {:?}",
            err.status()
        );
        assert!(
            format!("{err}").contains("3"),
            "final error must mention attempt count; got {err}"
        );
        // We exhausted all 3 attempts, then issued one verification-only Read.
        let calls = dispatcher.calls();
        assert_eq!(
            calls.len(),
            7,
            "expected 3 RMW attempts + verification: {calls:?}"
        );
    }

    #[tokio::test]
    async fn final_412_returns_success_when_verification_finds_marker() {
        let id = tracking_id(1);
        let committed = serde_json::json!({
            "id": "doc1",
            "pk": "pk1",
            "visits": 1,
            crate::driver::pipeline::patch_tracking::PATCH_TRACKING_PROPERTY: [
                marker_entry(id, 1)
            ]
        });
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(http_error(StatusCode::PreconditionFailed, "lost response")),
            ScriptedReply::ok(
                serde_json::to_vec(&committed).unwrap(),
                Some("\"v2\""),
                StatusCode::Ok,
            ),
        ]);

        let response = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op().with_patch_tracking_id(id),
            OperationOptions::default(),
            Some(NonZeroU8::new(1).unwrap()),
        )
        .await
        .expect("the final verification read proves the Replace committed");

        assert_eq!(
            response.headers().request_charge,
            Some(response.diagnostics().total_request_charge())
        );
        let body: serde_json::Value = response.into_body().into_single().unwrap();
        assert_eq!(body["visits"], 1);
        assert_eq!(dispatcher.calls().len(), 3);
    }

    #[tokio::test]
    async fn rmw_propagates_non_412_replace_error_immediately() {
        // A 500 / 503 / etc. on the Replace must surface verbatim when a
        // verification-only Read does not find the tracking marker.
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(http_error(StatusCode::InternalServerError, "boom")),
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v2\""),
                StatusCode::Ok,
            ),
        ]);

        let err = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op(),
            OperationOptions::default(),
            None,
        )
        .await
        .expect_err("non-412 Replace error must abort the loop");

        assert!(
            err.status().status_code() == StatusCode::InternalServerError,
            "non-412 must propagate verbatim; got {:?}",
            err.status()
        );
        // The wire failure keeps its own status/response, but its diagnostics
        // must be labeled with the virtual PATCH operation rather than the
        // `replace_item` sub-op that actually failed.
        assert_eq!(
            err.diagnostics()
                .as_deref()
                .and_then(DiagnosticsContext::operation_name),
            Some("patch_item"),
            "non-412 Replace failure must carry the PATCH operation identity"
        );
        // Single Read + single Replace + verification-only Read — no retry.
        assert_eq!(dispatcher.calls().len(), 3);
        assert!(err.patch_tracking_id().is_some());
        assert_eq!(
            err.diagnostics()
                .and_then(|diagnostics| diagnostics.patch_tracking_id()),
            err.patch_tracking_id()
        );
    }

    #[tokio::test]
    async fn definitive_replace_rejection_skips_verification() {
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(http_error(StatusCode::BadRequest, "rejected")),
        ]);

        let error = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op(),
            OperationOptions::default(),
            None,
        )
        .await
        .expect_err("definitive rejection must remain an error");

        assert_eq!(error.status().status_code(), StatusCode::BadRequest);
        assert_eq!(dispatcher.calls().len(), 2);
    }

    #[tokio::test]
    async fn malformed_terminal_verification_state_returns_bad_request() {
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(http_error(StatusCode::InternalServerError, "ambiguous")),
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","_azsdkPatchTracking":{}}"#.to_vec(),
                Some("\"v2\""),
                StatusCode::Ok,
            ),
        ]);

        let error = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op(),
            OperationOptions::default(),
            None,
        )
        .await
        .expect_err("malformed verification state must not be hidden");

        assert_eq!(error.status().status_code(), StatusCode::BadRequest);
        assert_eq!(dispatcher.calls().len(), 3);
    }

    #[tokio::test]
    async fn timeout_before_verification_exposes_id_for_safe_application_retry() {
        let timeout_error = || {
            crate::error::CosmosError::builder()
                .with_status(CosmosStatus::from_parts(
                    StatusCode::RequestTimeout,
                    Some(crate::models::SubStatusCode::CLIENT_OPERATION_TIMEOUT),
                ))
                .with_message("end-to-end operation timeout exceeded")
                .build()
        };
        let first_dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(timeout_error()),
            ScriptedReply::Err(timeout_error()),
        ]);

        let error = execute_with_dispatcher(
            &first_dispatcher,
            canonical_patch_op(),
            OperationOptions::default(),
            None,
        )
        .await
        .expect_err("timeout before verification leaves an ambiguous result");

        assert_eq!(error.status().status_code(), StatusCode::RequestTimeout);
        assert_eq!(
            error.status().sub_status(),
            Some(crate::models::SubStatusCode::CLIENT_OPERATION_TIMEOUT)
        );
        let effective_id = error
            .patch_tracking_id()
            .expect("ambiguous timeout must expose the generated tracking ID");
        assert_eq!(first_dispatcher.calls().len(), 3);

        let committed = serde_json::json!({
            "id": "doc1",
            "pk": "pk1",
            "visits": 1,
            crate::driver::pipeline::patch_tracking::PATCH_TRACKING_PROPERTY: [
                marker_entry(effective_id, 1)
            ]
        });
        let retry_dispatcher = ScriptedDispatcher::new(vec![ScriptedReply::ok(
            serde_json::to_vec(&committed).unwrap(),
            Some("\"v2\""),
            StatusCode::Ok,
        )]);

        let response = execute_with_dispatcher(
            &retry_dispatcher,
            canonical_patch_op().with_patch_tracking_id(effective_id),
            OperationOptions::default(),
            None,
        )
        .await
        .expect("retry with the timeout error's ID must recognize the commit");

        let body: serde_json::Value = response.into_body().into_single().unwrap();
        assert_eq!(body["visits"], 1);
        assert_eq!(retry_dispatcher.calls().len(), 1, "retry must not Replace");
    }

    #[tokio::test]
    async fn terminal_replace_error_returns_success_when_verification_finds_marker() {
        let id = tracking_id(1);
        let committed = serde_json::json!({
            "id": "doc1",
            "pk": "pk1",
            "visits": 1,
            crate::driver::pipeline::patch_tracking::PATCH_TRACKING_PROPERTY: [
                marker_entry(id, 1)
            ]
        });
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(http_error(StatusCode::InternalServerError, "ambiguous")),
            ScriptedReply::ok(
                serde_json::to_vec(&committed).unwrap(),
                Some("\"v2\""),
                StatusCode::Ok,
            ),
        ]);

        let response = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op().with_patch_tracking_id(id),
            OperationOptions::default(),
            None,
        )
        .await
        .expect("positive marker proof must recover an ambiguous Replace error");

        let body: serde_json::Value = response.into_body().into_single().unwrap();
        assert_eq!(body["visits"], 1);
        assert_eq!(dispatcher.calls().len(), 3);
    }

    #[tokio::test]
    async fn rmw_propagates_read_error_immediately() {
        // Gap #3 closure (handler-level): a non-2xx Read response (here a 404
        // for a non-existent item) propagates to the caller without ever
        // issuing a Replace. The emulator-level analog lives in
        // tests/emulator_tests/driver_patch.rs.
        let dispatcher = ScriptedDispatcher::new(vec![ScriptedReply::Err(http_error(
            StatusCode::NotFound,
            "no such item",
        ))]);

        let err = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op(),
            OperationOptions::default(),
            None,
        )
        .await
        .expect_err("PATCH on a missing item must fail on the Read");

        assert!(
            err.status().status_code() == StatusCode::NotFound,
            "PATCH on missing item must surface the Read's 404 verbatim; got {:?}",
            err.status()
        );
        // The Read's own diagnostics ride along on the error, but they must be
        // re-labeled with the virtual PATCH operation's name so a failed PATCH
        // is never reported as a `read_item`.
        assert_eq!(
            err.diagnostics()
                .as_deref()
                .and_then(DiagnosticsContext::operation_name),
            Some("patch_item"),
            "Read failure must carry the PATCH operation identity"
        );
        // Exactly one sub-op was issued: the Read. No Replace.
        let calls = dispatcher.calls();
        assert_eq!(calls.len(), 1, "no Replace must be issued on Read failure");
        assert_eq!(calls[0].op_type, OperationType::Read);
    }

    #[tokio::test]
    async fn rmw_fails_without_etag_before_replacing() {
        // The Read response without an ETag is unrecoverable — we cannot
        // construct an If-Match precondition. Verify the handler aborts
        // *before* issuing a Replace.
        let dispatcher = ScriptedDispatcher::new(vec![ScriptedReply::ok(
            br#"{"id":"doc1","pk":"pk1"}"#.to_vec(),
            None,
            StatusCode::Ok,
        )]);

        let _err = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op(),
            OperationOptions::default(),
            None,
        )
        .await
        .expect_err("missing ETag on Read must fail PATCH");
        let calls = dispatcher.calls();
        assert_eq!(calls.len(), 1, "no Replace must be issued without an ETag");
        assert_eq!(calls[0].op_type, OperationType::Read);
    }

    #[tokio::test]
    async fn rmw_read_error_on_retry_aggregates_prior_attempts() {
        // A Read failure on attempt 2 must still be labeled `patch_item` and
        // must fold in attempt 1's sub-op diagnostics, so the error reports the
        // whole PATCH — not just the sub-op that happened to fail.
        let read_failure = http_error(StatusCode::ServiceUnavailable, "read down");
        let failure_diagnostics = read_failure
            .diagnostics()
            .expect("fixture error carries diagnostics");
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(http_error(StatusCode::PreconditionFailed, "etag conflict")),
            ScriptedReply::Err(read_failure),
        ]);

        let err = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op(),
            OperationOptions::default(),
            NonZeroU8::new(3),
        )
        .await
        .expect_err("Read failure on retry must abort the loop");

        assert_eq!(err.status().status_code(), StatusCode::ServiceUnavailable);
        let diagnostics = err.diagnostics().expect("error must carry diagnostics");
        assert_eq!(diagnostics.operation_name(), Some("patch_item"));
        assert!(
            !Arc::ptr_eq(&diagnostics, &failure_diagnostics),
            "with prior sub-ops in flight the error's diagnostics must be an aggregate, \
             not the failing sub-op's context verbatim"
        );
    }

    #[tokio::test]
    async fn pk_guard_rejection_issues_no_sub_operations() {
        // Gap #4 closure: when the PK guard fires, the handler MUST return
        // before issuing a Read. A regression that re-ordered the guard
        // after the first dispatch would do a wasted I/O AND would expose
        // a window where a partition-key-mutating PATCH partially executed.
        let dispatcher = ScriptedDispatcher::new(vec![]); // any sub-op call panics

        // SET on `/pk` directly — this is a PK mutation; guard must reject.
        let op = patch_op_for(
            test_item_ref(),
            vec![PatchOperation::set("/pk", serde_json::json!("evicted"))],
        );

        let err = execute_with_dispatcher(&dispatcher, op, OperationOptions::default(), None)
            .await
            .expect_err("PK-mutating PATCH must be rejected by the guard");

        assert!(
            format!("{err}")
                .to_ascii_lowercase()
                .contains("partition key"),
            "error must mention the partition key; got: {err}"
        );
        // The script was empty: any sub-op dispatch would have panicked
        // with "ScriptedDispatcher exhausted". The fact that we got here
        // means zero sub-ops were issued.
        assert!(
            dispatcher.calls().is_empty(),
            "PK guard rejection must issue zero sub-operations; got: {:?}",
            dispatcher.calls()
        );
    }

    #[tokio::test]
    async fn empty_patch_spec_issues_no_sub_operations() {
        // A PATCH with zero ops short-circuits before any I/O — same
        // structural guarantee as the PK guard.
        let dispatcher = ScriptedDispatcher::new(vec![]);
        let op = patch_op_for(test_item_ref(), vec![]);

        let err = execute_with_dispatcher(&dispatcher, op, OperationOptions::default(), None)
            .await
            .expect_err("PATCH with no ops must be rejected");

        let msg = format!("{err}").to_ascii_lowercase();
        assert!(
            msg.contains("at least one"),
            "error should mention the empty-ops constraint: {err}"
        );
        assert!(dispatcher.calls().is_empty());
    }

    #[tokio::test]
    async fn malformed_patch_body_is_a_request_serialization_error() {
        let dispatcher = ScriptedDispatcher::new(vec![]);
        let operation = CosmosOperation::patch_item(test_item_ref()).with_body(b"{".to_vec());

        let error =
            execute_with_dispatcher(&dispatcher, operation, OperationOptions::default(), None)
                .await
                .expect_err("malformed PATCH input must fail before I/O");

        assert_eq!(
            error.status(),
            CosmosStatus::SERIALIZATION_REQUEST_BODY_INVALID
        );
        assert!(dispatcher.calls().is_empty());
    }

    #[tokio::test]
    async fn matching_caller_precondition_uses_read_etag_for_replace() {
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":1}"#.to_vec(),
                Some("\"v2\""),
                StatusCode::Ok,
            ),
        ]);
        let op = patch_op_for(
            test_item_ref(),
            vec![PatchOperation::increment("/visits", 1i64)],
        )
        .with_precondition(Precondition::if_match(Etag::from("\"v1\"")));

        execute_with_dispatcher(&dispatcher, op, OperationOptions::default(), None)
            .await
            .expect("matching caller If-Match must allow the PATCH");

        let calls = dispatcher.calls();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].op_type, OperationType::Read);
        assert_eq!(calls[1].op_type, OperationType::Replace);
        assert_eq!(calls[1].if_match_etag.as_deref(), Some("\"v1\""));
    }

    #[tokio::test]
    async fn mismatching_caller_if_match_fails_after_read() {
        let dispatcher = ScriptedDispatcher::new(vec![ScriptedReply::ok(
            br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
            Some("\"v1\""),
            StatusCode::Ok,
        )]);
        let op = patch_op_for(
            test_item_ref(),
            vec![PatchOperation::increment("/visits", 1i64)],
        )
        .with_precondition(Precondition::if_match(Etag::from("\"other\"")));

        let error = execute_with_dispatcher(&dispatcher, op, OperationOptions::default(), None)
            .await
            .expect_err("a failed caller If-Match must stop before Replace");

        assert_eq!(error.status().status_code(), StatusCode::PreconditionFailed);
        assert_eq!(dispatcher.calls().len(), 1);
        assert_eq!(dispatcher.calls()[0].op_type, OperationType::Read);
    }

    #[tokio::test]
    async fn caller_if_none_match_is_rejected_before_read() {
        let dispatcher = ScriptedDispatcher::new(vec![]);
        let op = patch_op_for(
            test_item_ref(),
            vec![PatchOperation::increment("/visits", 1i64)],
        )
        .with_precondition(Precondition::if_none_match(Etag::from("\"v1\"")));

        let error = execute_with_dispatcher(&dispatcher, op, OperationOptions::default(), None)
            .await
            .expect_err("PATCH If-None-Match must be rejected before I/O");

        assert_eq!(
            error.status(),
            crate::error::CosmosStatus::CLIENT_BAD_REQUEST
        );
        assert!(dispatcher.calls().is_empty());
    }

    #[tokio::test]
    async fn caller_if_match_is_reevaluated_after_replace_race() {
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(http_error(StatusCode::PreconditionFailed, "race")),
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":4}"#.to_vec(),
                Some("\"v2\""),
                StatusCode::Ok,
            ),
        ]);
        let op = patch_op_for(
            test_item_ref(),
            vec![PatchOperation::increment("/visits", 1i64)],
        );
        let op = op.with_precondition(Precondition::if_match(Etag::from("\"v1\"")));

        let error = execute_with_dispatcher(&dispatcher, op, OperationOptions::default(), None)
            .await
            .expect_err("the caller If-Match must fail after a concurrent update");

        assert_eq!(error.status().status_code(), StatusCode::PreconditionFailed);
        assert_eq!(
            dispatcher
                .calls()
                .iter()
                .map(|call| call.op_type)
                .collect::<Vec<_>>(),
            vec![
                OperationType::Read,
                OperationType::Replace,
                OperationType::Read
            ],
            "the changed ETag must stop the retry before a second Replace"
        );
    }

    #[tokio::test]
    async fn rmw_loop_dispatches_read_then_etag_guarded_replace() {
        // Structural pin: the loop issues exactly Read → Replace in order,
        // the Replace inherits the ETag captured from the Read, and the
        // post-image is produced from the locally-merged document.
        //
        // LatestCommitted Read token behavior and Read-response→Replace
        // token wiring are covered by the per-builder and multi-attempt tests.
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":1}"#.to_vec(),
                Some("\"v2\""),
                StatusCode::Ok,
            ),
        ]);

        let caller_token = SessionToken(Cow::Owned("0:1#7".into()));
        let op = canonical_patch_op().with_session_token(caller_token.clone());

        let mut options = OperationOptions::default();
        options.end_to_end_latency_policy = Some(
            crate::options::EndToEndOperationLatencyPolicy::new(std::time::Duration::from_secs(2)),
        );
        let _resp = execute_with_dispatcher(&dispatcher, op, options, None)
            .await
            .expect("PATCH should succeed");

        let calls = dispatcher.calls();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].op_type, OperationType::Read);
        assert_eq!(calls[0].if_match_etag, None);
        assert_eq!(calls[0].session_token.as_ref(), Some(&caller_token));
        assert_eq!(calls[1].op_type, OperationType::Replace);
        assert_eq!(calls[1].if_match_etag.as_deref(), Some("\"v1\""));
        assert!(calls[0].absolute_deadline.is_some());
        assert_eq!(calls[0].absolute_deadline, calls[1].absolute_deadline);
    }

    #[tokio::test]
    async fn rmw_reads_preserve_caller_token_across_412_retries() {
        // The caller token stays on each Read so reader fallback can preserve
        // an external session. Production writer routing strips it before
        // transport because LatestCommitted is not session-effective. Each
        // Replace still uses the token from its own Read response.
        let dispatcher = ScriptedDispatcher::new(vec![
            // Attempt 1
            ScriptedReply::Ok {
                body: br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                etag: Some("\"v1\""),
                session_token: Some("0:1#100"),
                status: StatusCode::Ok,
            },
            ScriptedReply::Err(http_error(StatusCode::PreconditionFailed, "lost the race")),
            // Attempt 2
            ScriptedReply::Ok {
                body: br#"{"id":"doc1","pk":"pk1","visits":1}"#.to_vec(),
                etag: Some("\"v2\""),
                session_token: Some("0:1#200"),
                status: StatusCode::Ok,
            },
            ScriptedReply::Ok {
                body: br#"{"id":"doc1","pk":"pk1","visits":2}"#.to_vec(),
                etag: Some("\"v3\""),
                session_token: Some("0:1#201"),
                status: StatusCode::Ok,
            },
        ]);

        let caller_token = SessionToken(Cow::Owned("0:1#1".into()));
        let op = canonical_patch_op().with_session_token(caller_token.clone());

        let _resp = execute_with_dispatcher(&dispatcher, op, OperationOptions::default(), None)
            .await
            .expect("PATCH should succeed after one 412 retry");

        let calls = dispatcher.calls();
        assert_eq!(calls.len(), 4);

        // Attempt 1 Read carries the caller's token for a possible fallback.
        assert_eq!(calls[0].op_type, OperationType::Read);
        assert_eq!(calls[0].session_token.as_ref(), Some(&caller_token));

        // Attempt 1, Replace: uses Attempt 1 Read's response token (TOCTOU
        // mitigation, unchanged behavior).
        assert_eq!(calls[1].op_type, OperationType::Replace);
        assert_eq!(
            calls[1].session_token.as_ref().map(|t| t.0.as_ref()),
            Some("0:1#100")
        );

        // Attempt 2 Read carries the same caller token, not the prior Read's
        // response token.
        assert_eq!(calls[2].op_type, OperationType::Read);
        assert_eq!(calls[2].session_token.as_ref(), Some(&caller_token));

        // Attempt 2, Replace: uses Attempt 2 Read's response token.
        assert_eq!(calls[3].op_type, OperationType::Replace);
        assert_eq!(
            calls[3].session_token.as_ref().map(|t| t.0.as_ref()),
            Some("0:1#200")
        );
    }

    #[tokio::test]
    async fn synthesized_response_body_reflects_replace_etag_not_read_etag() {
        // The locally-merged body the handler synthesizes is the Read body
        // with patch ops applied — but the Read body's `_etag` is the Read's
        // value, NOT the post-image's. The Replace just minted a fresh
        // `_etag` (the value in `replace_resp.headers().etag`), and that is
        // what the caller will see in the response header. The body MUST
        // carry the same `_etag` so callers that round-trip the body
        // (`.into_model::<MyTypeWithEtag>()`) see the authoritative value,
        // not a stale Read-time tag that would be rejected as a future
        // If-Match precondition.
        //
        // Script: Read returns body with _etag=\"v1\" + etag header \"v1\";
        // Replace returns an explicitly requested empty body + etag header
        // \"v2\".
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0,"_etag":"\"v1\""}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::ok(Vec::new(), Some("\"v2\""), StatusCode::Ok),
        ]);

        let op = canonical_patch_op();
        let options = OperationOptions {
            content_response_on_write: Some(ContentResponseOnWrite::Disabled),
            ..OperationOptions::default()
        };
        let resp = execute_with_dispatcher(&dispatcher, op, options, None)
            .await
            .expect("PATCH should succeed");

        assert_eq!(
            dispatcher.calls()[1].content_response_on_write,
            Some(ContentResponseOnWrite::Disabled),
            "bodyless client-side PATCH must keep the inner Replace bodyless"
        );

        // Header carries the Replace's new etag (existing behavior).
        assert_eq!(
            resp.headers().etag.as_ref().map(|t| -> &str { t.as_ref() }),
            Some("\"v2\""),
            "response header etag must be the Replace's etag"
        );

        // Body's `_etag` MUST match the header — the Read's `_etag` (`\"v1\"`)
        // must have been overwritten with the Replace's (`\"v2\"`).
        let body: serde_json::Value = resp
            .into_body()
            .into_single()
            .expect("body must be valid JSON");
        assert_eq!(
            body.get("_etag").and_then(|v| v.as_str()),
            Some("\"v2\""),
            "synthesized body's _etag must be the Replace's, not the Read's"
        );
        // Other patched fields are preserved.
        assert_eq!(body.get("visits").and_then(|v| v.as_i64()), Some(1));
    }

    #[tokio::test]
    async fn synthesized_response_body_prefers_replace_response_body_when_present() {
        // When `content_response_on_write` is true on the inner Replace, the
        // service returns the full post-image — that body is the source of
        // truth for the whole document (including `_etag`, `_ts`, and any
        // server-applied transforms). The handler must surface it verbatim
        // rather than its locally-merged version.
        let server_post_image =
            br#"{"id":"doc1","pk":"pk1","visits":1,"_etag":"\"v2\"","_ts":1234567890}"#.to_vec();
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0,"_etag":"\"v1\"","_ts":1234567000}"#
                    .to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::ok(server_post_image.clone(), Some("\"v2\""), StatusCode::Ok),
        ]);

        let op = canonical_patch_op();
        let resp = execute_with_dispatcher(&dispatcher, op, OperationOptions::default(), None)
            .await
            .expect("PATCH should succeed");

        assert_eq!(
            dispatcher.calls()[1].content_response_on_write,
            Some(ContentResponseOnWrite::Enabled),
            "default client-side PATCH must request the authoritative Replace post-image"
        );

        let body_bytes = resp
            .into_body()
            .single()
            .expect("body should be a single payload");
        assert_eq!(
            body_bytes.as_ref(),
            server_post_image.as_slice(),
            "when the Replace returned a body, the handler must surface it \
             verbatim (it's the service-authoritative post-image)"
        );
    }

    #[tokio::test]
    async fn rmw_aggregates_diagnostics_across_sub_operations() {
        // The synthesized PATCH response's DiagnosticsContext must be the
        // *aggregate* of every successful sub-op's DiagnosticsContext, not
        // just the final Replace's. Pre-aggregation the handler returned
        // `replace_resp.diagnostics()` directly, so callers saw activity
        // for one HTTP request even though the loop made N sub-ops. We
        // pin both shape (Arc identity differs from any single sub-op
        // context) and behavior (aggregated activity_id matches the LAST
        // sub-op's activity_id; status comes from the LAST sub-op).
        use crate::models::CosmosResponseHeaders;

        // We need to peek at the Arc identity of each sub-op's
        // DiagnosticsContext, so use a bespoke dispatcher that captures
        // the diagnostics it hands out instead of `ScriptedDispatcher`.
        struct CapturingDispatcher {
            handed_out: Mutex<Vec<Arc<DiagnosticsContext>>>,
        }

        #[async_trait]
        impl SubOperationDispatcher for CapturingDispatcher {
            async fn execute_operation(
                &self,
                operation: CosmosOperation,
                _options: OperationOptions,
            ) -> crate::error::Result<CosmosResponse> {
                let body = match operation.operation_type() {
                    OperationType::Read => {
                        br#"{"id":"doc1","pk":"pk1","visits":0,"_ts":1}"#.to_vec()
                    }
                    OperationType::Replace => br#"{"id":"doc1","pk":"pk1","visits":1}"#.to_vec(),
                    other => panic!("unexpected sub-op {other:?}"),
                };
                let mut headers = CosmosResponseHeaders::new();
                headers.etag = Some(Etag::from("\"v1\""));
                headers.request_charge = Some(RequestCharge::new(1.0));
                let mut diagnostics = DiagnosticsContextBuilder::new(
                    ActivityId::new_uuid(),
                    Arc::new(DiagnosticsOptions::default()),
                );
                let endpoint = crate::driver::routing::CosmosEndpoint::global(
                    Url::parse("https://test.documents.azure.com/").unwrap(),
                );
                let handle = diagnostics.start_request(
                    crate::diagnostics::ExecutionContext::Initial,
                    crate::diagnostics::PipelineType::DataPlane,
                    crate::diagnostics::TransportSecurity::Secure,
                    crate::diagnostics::TransportKind::Gateway,
                    crate::diagnostics::TransportHttpVersion::Http11,
                    &endpoint,
                );
                diagnostics.record_response(handle, StatusCode::Ok, &headers);
                let diagnostics = Arc::new(diagnostics.complete());
                self.handed_out
                    .lock()
                    .unwrap()
                    .push(Arc::clone(&diagnostics));
                Ok(from_local_body_and_driver_headers(
                    body,
                    headers,
                    CosmosStatus::from_parts(StatusCode::Ok, None),
                    diagnostics,
                ))
            }
        }

        let dispatcher = CapturingDispatcher {
            handed_out: Mutex::new(Vec::new()),
        };

        let resp = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op(),
            OperationOptions::default(),
            None,
        )
        .await
        .expect("PATCH should succeed");

        let handed_out = dispatcher.handed_out.lock().unwrap().clone();
        assert_eq!(
            handed_out.len(),
            2,
            "expected one Read + one Replace sub-op"
        );

        let returned = resp.diagnostics();

        // Aggregation produces a fresh Arc that is identity-distinct from
        // each individual sub-op context; a regression that returned
        // `replace_resp.diagnostics()` directly would fail this check.
        assert!(
            !Arc::ptr_eq(&returned, &handed_out[0]),
            "returned diagnostics must not be identity-equal to the Read sub-op's context"
        );
        assert!(
            !Arc::ptr_eq(&returned, &handed_out[1]),
            "returned diagnostics must not be identity-equal to the Replace sub-op's context \
             (regression: handler used to return the Replace's context verbatim)"
        );

        // The aggregated context inherits its activity_id from the LAST
        // source (the Replace), per `aggregate_sub_operations`'s contract.
        assert_eq!(returned.activity_id(), handed_out[1].activity_id());

        // ...but its `db.operation.name` is the virtual PATCH operation's own
        // name, not the Replace sub-op's, so telemetry labels the operation
        // correctly.
        assert_eq!(returned.operation_name(), Some("patch_item"));
        assert_eq!(resp.headers().request_charge, Some(RequestCharge::new(2.0)));
    }

    #[tokio::test]
    async fn rmw_forces_binary_encoding_off_on_forwarded_sub_ops() {
        // A caller may set `binary_encoding` on a patch; the handler must force
        // it OFF explicitly (not `None`, which would inherit an account/client
        // default) so the forwarded Read/Replace sub-ops stay text.
        struct OptionsCapturingDispatcher {
            binary_encodings: Mutex<Vec<Option<BinaryEncodingOptions>>>,
        }

        #[async_trait]
        impl SubOperationDispatcher for OptionsCapturingDispatcher {
            async fn execute_operation(
                &self,
                operation: CosmosOperation,
                options: OperationOptions,
            ) -> crate::error::Result<CosmosResponse> {
                self.binary_encodings
                    .lock()
                    .unwrap()
                    .push(options.binary_encoding.clone());
                let body = match operation.operation_type() {
                    OperationType::Read => {
                        br#"{"id":"doc1","pk":"pk1","visits":0,"_ts":1}"#.to_vec()
                    }
                    OperationType::Replace => br#"{"id":"doc1","pk":"pk1","visits":1}"#.to_vec(),
                    other => panic!("unexpected sub-op {other:?}"),
                };
                let mut headers = CosmosResponseHeaders::new();
                headers.etag = Some(Etag::from("\"v1\""));
                let diagnostics = Arc::new(
                    DiagnosticsContextBuilder::new(
                        ActivityId::new_uuid(),
                        Arc::new(DiagnosticsOptions::default()),
                    )
                    .complete(),
                );
                Ok(from_local_body_and_driver_headers(
                    body,
                    headers,
                    CosmosStatus::from_parts(StatusCode::Ok, None),
                    diagnostics,
                ))
            }
        }

        let dispatcher = OptionsCapturingDispatcher {
            binary_encodings: Mutex::new(Vec::new()),
        };

        // Caller opts a patch into binary encoding + text response.
        let mut options = OperationOptions::default();
        options.binary_encoding = Some(
            BinaryEncodingOptions::new()
                .with_enabled(true)
                .with_request_text_response(true),
        );

        execute_with_dispatcher(&dispatcher, canonical_patch_op(), options, None)
            .await
            .expect("PATCH should succeed");

        let captured = dispatcher.binary_encodings.lock().unwrap().clone();
        assert_eq!(captured.len(), 2, "expected one Read + one Replace sub-op");
        let disabled = Some(BinaryEncodingOptions::new().with_enabled(false));
        assert!(
            captured.iter().all(|be| *be == disabled),
            "patch must force binary_encoding OFF (explicit disabled, not inherit) \
             on every forwarded sub-op, got {captured:?}",
        );
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Driver-side handler for [`OperationType::Patch`] operations.
//!
//! See `docs/PATCH_HANDLER_SPEC.md` for the full behavior contract. The
//! short version:
//!
//! 1. Reject any caller-set [`Precondition`] on the outer PATCH operation —
//!    the handler manages `If-Match` internally, so honoring a caller's
//!    value would silently break the RMW guarantee.
//! 2. Validate the patch spec (no ops that target partition-key paths).
//! 3. Issue an internal [`OperationType::Read`] for the target item.
//! 4. Capture the response ETag (refuse to RMW if there isn't one).
//! 5. Parse the JSON body into a [`serde_json::Value`], apply the ops locally
//!    using [`apply_patch_ops`], and re-serialize.
//! 6. Issue an internal ETag-guarded [`OperationType::Replace`].
//! 7. On `412 Precondition Failed`, restart from step 3 — up to
//!    `max_attempts` (default 5) total tries. Across attempts the loop
//!    monotonically advances the session token it threads into the next
//!    Read so attempt N never observes a strictly older session view than
//!    attempt N-1.
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
use crate::driver::CosmosDriver;
use crate::models::{
    CosmosOperation, CosmosResponse, PartitionKeyKind, PatchOp, PatchSpec, Precondition,
    SessionToken,
};
use crate::options::OperationOptions;
use async_trait::async_trait;
use azure_core::http::StatusCode;
use std::num::NonZeroU8;
use std::sync::Arc;

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
) -> crate::error::Result<CosmosResponse> {
    execute_with_dispatcher(driver, operation, options, max_attempts).await
}

/// Same as [`execute`], but parameterized over the sub-operation dispatcher.
/// Tests provide a stub that returns scripted responses without a live
/// endpoint.
pub(crate) async fn execute_with_dispatcher<D: SubOperationDispatcher + ?Sized>(
    dispatcher: &D,
    operation: CosmosOperation,
    options: OperationOptions,
    max_attempts: Option<NonZeroU8>,
) -> crate::error::Result<CosmosResponse> {
    // -- 1. Reject caller-set preconditions --
    //
    // PATCH manages its own `If-Match` precondition internally — the handler
    // captures the current item's ETag on the internal Read and threads it
    // into the internal Replace. Honoring a caller-set `Precondition` would
    // either shadow that ETag (silently breaking the RMW guarantees) or
    // require resolving it against the handler's own ETag (no sensible
    // merge). The SDK's `ContainerClient::patch_item` already drops any
    // precondition before reaching this layer; this guard fail-fasts on any
    // driver-level user that constructed
    // `CosmosOperation::patch_item(..).with_precondition(..)` directly,
    // instead of silently ignoring it.
    if operation.precondition().is_some() {
        return Err(
            crate::error::CosmosError::builder(crate::error::CosmosStatusKind::Client)
                .with_message(
                    "PATCH does not support caller-set preconditions; \
             the handler manages If-Match internally",
                )
                .build(),
        );
    }

    // -- 2. Parse and validate the patch spec --
    let body = operation
        .body()
        .ok_or_else(|| missing_body_error("PATCH operation requires a PatchSpec body"))?;
    let spec: PatchSpec = serde_json::from_slice(body).map_err(|err| {
        crate::error::CosmosError::builder(crate::error::CosmosStatusKind::Serialization)
            .with_message(format!("failed to parse PATCH body as PatchSpec: {err}"))
            .with_source(err)
            .build()
    })?;

    if spec.operations.is_empty() {
        return Err(
            crate::error::CosmosError::builder(crate::error::CosmosStatusKind::Client)
                .with_message("PATCH operation must include at least one PatchOp")
                .build(),
        );
    }

    let item_ref = operation
        .partition_key()
        .cloned()
        .and_then(|pk| operation.resource_reference().try_into_item_reference(pk))
        .ok_or_else(|| {
            crate::error::CosmosError::builder(crate::error::CosmosStatusKind::Client)
                .with_message(
                    "PATCH dispatch requires an item-level operation with a partition key",
                )
                .build()
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
    //
    // Across RMW attempts we monotonically advance `effective_session_token`
    // to the freshest one we observe (Read response on every attempt;
    // Replace response on the final successful attempt). That way attempt
    // N's Read does not regress to a strictly older session view than
    // attempt N-1 already saw.
    let mut effective_session_token = operation.request_headers().session_token.clone();

    // -- 3..7. RMW loop --
    let mut last_412: Option<crate::error::CosmosError> = None;
    // Aggregated diagnostics across every successful sub-op the loop
    // dispatches. We hand this to `from_local_body_and_driver_headers`
    // when we synthesize the success response so callers see one
    // PATCH operation = one DiagnosticsContext containing every
    // sub-op's per-request diagnostics, instead of just the final
    // Replace's. See `DiagnosticsContext::aggregate_sub_operations`.
    let mut sub_op_diagnostics: Vec<Arc<DiagnosticsContext>> =
        Vec::with_capacity(2 * attempts as usize);
    for _ in 0..attempts {
        // Read the current item, propagating the freshest session token we
        // have observed so far (caller's on attempt 1; carried-forward on
        // subsequent attempts).
        let read_op = build_read_sub_op(item_ref.clone(), effective_session_token.clone());

        // Any non-2xx Read response is mapped by the driver pipeline into
        // `Err(ErrorKind::HttpResponse { .. })` (see retry_evaluation.rs's
        // `build_http_error`). Propagating with `?` is sufficient — the
        // caller wants the original error verbatim, complete with
        // `raw_response` and diagnostics — and there is nothing useful the
        // PATCH handler can do on a Read failure.
        let read_resp = dispatcher
            .execute_operation(read_op, options.clone())
            .await?;
        sub_op_diagnostics.push(read_resp.diagnostics());
        let etag = read_resp.headers().etag.clone().ok_or_else(|| {
            crate::error::CosmosError::builder(crate::error::CosmosStatusKind::Client)
                .with_message("PATCH cannot proceed: the Read response did not include an ETag")
                .build()
        })?;
        // R3-DRIVER: forward the session token returned by the Read on the
        // Replace, so the write commits against the same replica view we
        // just read from. This is what mitigates SE-004 (session token
        // TOCTOU across read->write).
        let read_session_token = read_resp.headers().session_token.clone();
        // Carry the Read response's session token into the next attempt's
        // Read so a subsequent retry never regresses to a strictly older
        // session view.
        if let Some(token) = read_session_token.clone() {
            effective_session_token = Some(token);
        }

        // Locally apply the patch ops.
        let read_body_bytes = read_resp.into_body().single().map_err(|err| {
            crate::error::CosmosError::builder(crate::error::CosmosStatusKind::Serialization)
                .with_message(format!("PATCH could not extract Read response body: {err}"))
                .with_source(err)
                .build()
        })?;
        let mut value: serde_json::Value =
            serde_json::from_slice(&read_body_bytes).map_err(|err| {
                crate::error::CosmosError::builder(crate::error::CosmosStatusKind::Serialization)
                    .with_message(format!(
                        "PATCH could not deserialize current item body: {err}"
                    ))
                    .with_source(err)
                    .build()
            })?;
        apply_patch_ops(&mut value, &spec.operations)?;
        let merged_bytes = serde_json::to_vec(&value).map_err(|err| {
            crate::error::CosmosError::builder(crate::error::CosmosStatusKind::Serialization)
                .with_message(format!("PATCH could not serialize merged item: {err}"))
                .with_source(err)
                .build()
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
        match dispatcher
            .execute_operation(replace_op, options.clone())
            .await
        {
            Ok(replace_resp) => {
                let replace_headers = replace_resp.headers().clone();
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
                let diagnostics = DiagnosticsContext::aggregate_sub_operations(&sub_op_diagnostics)
                    .map(Arc::new)
                    .unwrap_or_else(|| {
                        sub_op_diagnostics
                            .last()
                            .cloned()
                            .expect("sub_op_diagnostics is non-empty after a successful Replace")
                    });
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
                //
                // A 412 response carries a session token that is strictly
                // fresher than the Read we just performed (it was produced
                // by a replica that already saw the conflicting writer's
                // commit). Fold it into `effective_session_token` — using
                // `merge` to preserve segments from previously observed
                // tokens for other partition-key ranges — so the next
                // attempt's Read can't regress to an older session view.
                // Falls back to the carry-forward from the Read response
                // we already advanced above when the 412 carries no
                // session token header (e.g. unit-test errors built
                // without a populated response).
                if let Some(token_412) = session_token_from_error(&err) {
                    effective_session_token = Some(
                        effective_session_token
                            .as_ref()
                            .and_then(|prev| prev.merge(&token_412).ok())
                            .unwrap_or(token_412),
                    );
                }
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
                if let Some(diag) = err.diagnostics() {
                    sub_op_diagnostics.push(Arc::clone(diag));
                }
                last_412 = Some(err);
                continue;
            }
            Err(err) => return Err(err),
        }
    }

    Err(exhaustion_error(attempts, last_412, &sub_op_diagnostics))
}

fn missing_body_error(msg: &'static str) -> crate::error::CosmosError {
    crate::error::CosmosError::builder(crate::error::CosmosStatusKind::Client)
        .with_message(msg)
        .build()
}

/// Returns `true` if `err` is the driver pipeline's representation of a
/// `412 Precondition Failed` HTTP response (i.e. our ETag-guarded Replace
/// lost the race against a concurrent writer).
///
/// The driver pipeline maps every non-2xx response — 412 included — into
/// an `Err(crate::error::CosmosError)` with `CosmosStatusKind::Service` via
/// `retry_evaluation::build_http_error`, and 412 specifically resolves
/// to `OperationAction::Abort` (it is never retried at the pipeline layer).
/// The patch handler's RMW loop is the *one* place where 412 needs to be
/// recovered into a retry, so we narrow on the kind here instead of relying
/// on a status check that the `await?` above would never reach. Requires
/// `CosmosStatusKind::Service` so a future internal constructor that happens to use
/// `StatusCode::PreconditionFailed` cannot accidentally trigger the RMW
/// retry path.
fn is_precondition_failed(err: &crate::error::CosmosError) -> bool {
    err.status().is_service_error() && err.status().is_precondition_failed()
}

/// Extracts the `x-ms-session-token` from a service-built cosmos error's
/// parsed response headers, if present.
///
/// The driver pipeline mints every non-2xx response into a typed
/// service error with the wire-level [`CosmosResponsePayload`] (body +
/// parsed [`CosmosResponseHeaders`]) attached, so the session-token
/// header on a 412 is already accessible via the [`CosmosResponse`] returned
/// by [`CosmosError::response`].
/// Returns `None` for non-service errors or service errors whose response
/// carried no session-token header (e.g. accounts not configured for
/// Session consistency).
fn session_token_from_error(err: &crate::error::CosmosError) -> Option<SessionToken> {
    err.wire_payload()
        .and_then(|p| p.headers().session_token.clone())
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
///    Other system properties (`_rid`, `_self`, `_attachments`) are stable
///    across edits of the same item, so the Read's values remain correct.
///    `_ts` is not exposed on the Replace response header path, so the
///    Read's `_ts` is left intact; it may lag the true post-image by the
///    Read→Replace round-trip but never goes backwards.
/// 3. If `merged_bytes` is not a JSON object, or `replace_etag` is `None`,
///    or any serde step fails, the merged bytes are returned unchanged —
///    the body in that case is no worse than what the previous
///    implementation produced.
fn synthesize_post_image_body(
    merged_bytes: Vec<u8>,
    replace_body: Vec<u8>,
    replace_etag: Option<&crate::models::ETag>,
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
        serde_json::Value::String(etag.as_str().to_owned()),
    );
    serde_json::to_vec(&value).unwrap_or(merged_bytes)
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
) -> crate::error::CosmosError {
    let message = format!("patch_item: ETag conflict after {attempts} attempts");
    let aggregated = DiagnosticsContext::aggregate_sub_operations(sub_op_diagnostics).map(Arc::new);
    match last_412 {
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
            let mut b = crate::error::CosmosError::builder(crate::error::CosmosStatusKind::Service)
                .with_status(crate::models::CosmosStatus::new(
                    StatusCode::PreconditionFailed,
                ))
                .with_message(message);
            if let Some(diag) = aggregated {
                b = b.with_diagnostics(diag);
            }
            b.build()
        }
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
            PatchOp::MoveOp { from, .. } => Some(from.as_str()),
            _ => None,
        };
        for path in std::iter::once(dest).chain(from) {
            for pk_path in &pk_paths {
                if path_overlaps_partition_key(path, pk_path) {
                    return Err(crate::error::CosmosError::builder(
                        crate::error::CosmosStatusKind::Client,
                    )
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
        // assert the If-Match precondition was applied. A future refactor
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
    fn is_precondition_failed_rejects_non_http_error_kinds() {
        use crate::error::{CosmosError, CosmosStatusKind};
        let errs = [
            CosmosError::builder(CosmosStatusKind::Client)
                .with_message("synthetic")
                .build(),
            CosmosError::builder(CosmosStatusKind::Serialization)
                .with_message("bad json")
                .with_source(std::io::Error::new(std::io::ErrorKind::InvalidData, "stub"))
                .build(),
        ];
        for err in &errs {
            assert!(
                !is_precondition_failed(err),
                "should not match {:?}",
                err.kind()
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

        let ops = vec![PatchOp::move_op("/tenant", "/somewhere_else")];

        let err = validate_partition_key_paths(&ops, &item_ref)
            .expect_err("MoveOp from /tenant on a hierarchical PK must be rejected");
        let msg = format!("{err}").to_ascii_lowercase();
        assert!(
            msg.contains("partition key"),
            "error should mention partition key; got: {err}"
        );
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
        let err = exhaustion_error(7, Some(underlying), &[]);

        // (a) Shape.
        assert_eq!(
            err.status().status_code(),
            StatusCode::PreconditionFailed,
            "exhaustion error must surface as a 412; got {:?}",
            err.kind()
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
        let err = exhaustion_error(0, None, &[]);

        assert_eq!(err.status().status_code(), StatusCode::PreconditionFailed);
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
        let err = exhaustion_error(4, Some(underlying), &[]);

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
        let err = exhaustion_error(2, Some(underlying), &attempt_diags);

        let diag = err
            .diagnostics()
            .expect("exhaustion error must carry an aggregated DiagnosticsContext");
        assert_eq!(
            diag.request_count(),
            4,
            "aggregated diagnostics must concatenate every per-attempt RequestDiagnostics",
        );
        // And critically, the attached diagnostics must be distinct from
        // every input Arc — the aggregator returns a fresh context.
        for input in &attempt_diags {
            assert!(
                !Arc::ptr_eq(diag, input),
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

    use crate::diagnostics::DiagnosticsContextBuilder;
    use crate::models::{ActivityId, CosmosResponseHeaders, CosmosStatus, RequestCharge};
    use crate::options::DiagnosticsOptions;
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
            _options: OperationOptions,
        ) -> crate::error::Result<CosmosResponse> {
            let if_match = match operation.precondition() {
                Some(Precondition::IfMatch(tag)) => Some(tag.as_ref().to_string()),
                _ => None,
            };
            self.calls.lock().unwrap().push(DispatchedCall {
                op_type: operation.operation_type(),
                if_match_etag: if_match,
                session_token: operation.request_headers().session_token.clone(),
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
                } => {
                    let mut headers = CosmosResponseHeaders::new();
                    if let Some(tag) = etag {
                        headers.etag = Some(ETag::from(tag));
                    }
                    if let Some(token) = session_token {
                        headers.session_token = Some(SessionToken(Cow::Owned(token.into())));
                    }
                    headers.request_charge = Some(RequestCharge::new(1.0));
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
                        CosmosStatus::from_parts(status, None),
                        diagnostics,
                    ))
                }
            }
        }
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

    /// Same as [`http_error`], but populates the cosmos response headers
    /// with the given session token so the patch handler can recover it
    /// via `session_token_from_error`.
    fn http_error_with_session_token(
        status: StatusCode,
        msg: &'static str,
        session_token: &'static str,
    ) -> crate::error::CosmosError {
        cosmos_service_error(status, msg, Some(session_token), &[])
    }

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
        crate::error::CosmosError::builder(crate::error::CosmosStatusKind::Service)
            .with_status(CosmosStatus::new(status))
            .with_message(msg)
            .with_response_parts(crate::models::CosmosResponsePayload::new(
                body.to_vec(),
                headers,
            ))
            .build()
    }

    fn patch_op_for(item_ref: ItemReference, ops: Vec<PatchOp>) -> CosmosOperation {
        let body = serde_json::to_vec(&PatchSpec::new(ops)).unwrap();
        CosmosOperation::patch_item(item_ref).with_body(body)
    }

    /// Builds the canonical (`/pk`, `pk1`, `doc1`) PATCH operation used by
    /// all of these tests — `+1` on `/visits`.
    fn canonical_patch_op() -> CosmosOperation {
        patch_op_for(test_item_ref(), vec![PatchOp::increment("/visits", 1i64)])
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
        assert_eq!(calls[1].op_type, OperationType::Replace);
        // Each Replace MUST be If-Match guarded — the ETag guard is the
        // entire reason the RMW is safe under concurrent writers.
        assert_eq!(calls[1].if_match_etag.as_deref(), Some("\"v1\""));
        assert_eq!(calls[2].op_type, OperationType::Read);
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
            err.kind()
        );
        assert!(
            format!("{err}").contains("3"),
            "final error must mention attempt count; got {err}"
        );
        // We exhausted all 3 attempts: that's exactly 3 Reads + 3 Replaces.
        let calls = dispatcher.calls();
        assert_eq!(calls.len(), 6, "expected 3 RMW attempts: {calls:?}");
    }

    #[tokio::test]
    async fn rmw_propagates_non_412_replace_error_immediately() {
        // A 500 / 503 / etc. on the Replace must surface verbatim — no
        // retry, no remapping. The retry loop is ONLY for 412.
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::Err(http_error(StatusCode::InternalServerError, "boom")),
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
            err.kind()
        );
        // Single Read + single Replace — no retry.
        assert_eq!(dispatcher.calls().len(), 2);
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
            err.kind()
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

        let err = execute_with_dispatcher(
            &dispatcher,
            canonical_patch_op(),
            OperationOptions::default(),
            None,
        )
        .await
        .expect_err("missing ETag on Read must fail PATCH");

        assert!(err.kind() == crate::error::CosmosStatusKind::Client);
        let calls = dispatcher.calls();
        assert_eq!(calls.len(), 1, "no Replace must be issued without an ETag");
        assert_eq!(calls[0].op_type, OperationType::Read);
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
            vec![PatchOp::set("/pk", serde_json::json!("evicted"))],
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
    async fn caller_set_precondition_is_rejected_before_any_sub_op() {
        // PATCH manages its own If-Match internally — letting a caller-set
        // precondition through would either shadow the handler's ETag
        // (silently breaking RMW) or require resolving against it (no
        // sensible merge). The guard must fail fast before issuing any
        // sub-operation so a misuse never makes it onto the wire.
        let dispatcher = ScriptedDispatcher::new(vec![]);
        let op = patch_op_for(
            test_item_ref(),
            vec![PatchOp::set("/x", serde_json::json!(1))],
        )
        .with_precondition(Precondition::if_match(ETag::from("\"abc\"")));

        let err = execute_with_dispatcher(&dispatcher, op, OperationOptions::default(), None)
            .await
            .expect_err("PATCH with caller-set precondition must be rejected");

        let msg = format!("{err}").to_ascii_lowercase();
        assert!(
            msg.contains("precondition"),
            "error should mention the precondition rejection: {err}"
        );
        assert!(
            dispatcher.calls().is_empty(),
            "precondition rejection must issue zero sub-operations; got: {:?}",
            dispatcher.calls()
        );
    }

    #[tokio::test]
    async fn rmw_loop_dispatches_read_then_etag_guarded_replace() {
        // Structural pin: the loop issues exactly Read → Replace in order,
        // the Replace inherits the ETag captured from the Read, and the
        // post-image is produced from the locally-merged document.
        //
        // Cross-attempt session-token carry-forward is covered by
        // `rmw_carries_session_token_forward_across_412_retries`; the
        // single-attempt caller→Read / Read-response→Replace wire-up is
        // covered by the per-builder unit tests
        // `read_sub_op_propagates_caller_session_token` and
        // `replace_sub_op_uses_read_response_session_token`.
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

        let _resp = execute_with_dispatcher(&dispatcher, op, OperationOptions::default(), None)
            .await
            .expect("PATCH should succeed");

        let calls = dispatcher.calls();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].op_type, OperationType::Read);
        assert_eq!(calls[0].if_match_etag, None);
        assert_eq!(calls[1].op_type, OperationType::Replace);
        assert_eq!(calls[1].if_match_etag.as_deref(), Some("\"v1\""));
    }

    #[tokio::test]
    async fn rmw_carries_session_token_forward_across_412_retries() {
        // The loop must monotonically advance the session token it threads
        // into the next attempt's Read: attempt 2's Read should observe
        // attempt 1's Read response token, not regress to the caller's
        // (potentially older) token. This guards against a future
        // regression that resets `effective_session_token` to the caller's
        // value at the top of every iteration — which would silently
        // weaken the session-consistency guarantees the PATCH handler
        // promises after the first 412.
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

        // Attempt 1, Read: uses caller's session token.
        assert_eq!(calls[0].op_type, OperationType::Read);
        assert_eq!(calls[0].session_token.as_ref(), Some(&caller_token));

        // Attempt 1, Replace: uses Attempt 1 Read's response token (TOCTOU
        // mitigation, unchanged behavior).
        assert_eq!(calls[1].op_type, OperationType::Replace);
        assert_eq!(
            calls[1].session_token.as_ref().map(|t| t.0.as_ref()),
            Some("0:1#100")
        );

        // Attempt 2, Read: MUST use the freshest observed token (Attempt 1
        // Read's `0:1#100`), NOT the caller's stale `0:1#1`. This is the
        // cross-attempt carry-forward.
        assert_eq!(calls[2].op_type, OperationType::Read);
        assert_eq!(
            calls[2].session_token.as_ref().map(|t| t.0.as_ref()),
            Some("0:1#100"),
            "attempt 2 Read must use the carried-forward session token \
             from attempt 1's Read response, not the caller's stale token"
        );

        // Attempt 2, Replace: uses Attempt 2 Read's response token.
        assert_eq!(calls[3].op_type, OperationType::Replace);
        assert_eq!(
            calls[3].session_token.as_ref().map(|t| t.0.as_ref()),
            Some("0:1#200")
        );
    }

    #[tokio::test]
    async fn rmw_folds_412_response_session_token_into_carry_forward() {
        // When a 412 carries a session-token header (the replica that
        // rejected our Replace had already seen the conflicting writer's
        // commit), it is strictly fresher than the Read response we just
        // observed. The PATCH handler must fold it into
        // `effective_session_token` so attempt 2's Read uses the freshest
        // possible view — matching .NET's behavior and minimizing the
        // chance of an avoidable second 412.
        //
        // Script: Read#1 token=0:1#100 -> Replace#1 412 with token=0:1#300
        // -> Read#2 token=0:1#301 -> Replace#2 ok.
        let dispatcher = ScriptedDispatcher::new(vec![
            // Attempt 1
            ScriptedReply::Ok {
                body: br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                etag: Some("\"v1\""),
                session_token: Some("0:1#100"),
                status: StatusCode::Ok,
            },
            ScriptedReply::Err(http_error_with_session_token(
                StatusCode::PreconditionFailed,
                "lost the race",
                "0:1#300",
            )),
            // Attempt 2
            ScriptedReply::Ok {
                body: br#"{"id":"doc1","pk":"pk1","visits":1}"#.to_vec(),
                etag: Some("\"v2\""),
                session_token: Some("0:1#301"),
                status: StatusCode::Ok,
            },
            ScriptedReply::Ok {
                body: br#"{"id":"doc1","pk":"pk1","visits":2}"#.to_vec(),
                etag: Some("\"v3\""),
                session_token: Some("0:1#302"),
                status: StatusCode::Ok,
            },
        ]);

        let op = canonical_patch_op();
        let _resp = execute_with_dispatcher(&dispatcher, op, OperationOptions::default(), None)
            .await
            .expect("PATCH should succeed");

        let calls = dispatcher.calls();
        assert_eq!(calls.len(), 4);

        // Attempt 2 Read MUST use the 412's session token (0:1#300), not
        // the Read#1's older 0:1#100. SessionToken::merge picks the
        // higher version per partition-key range, so the carry-forward
        // strictly advances.
        assert_eq!(calls[2].op_type, OperationType::Read);
        assert_eq!(
            calls[2].session_token.as_ref().map(|t| t.0.as_ref()),
            Some("0:1#300"),
            "attempt 2 Read must use the 412's session token (freshest), \
             not Read#1's older token"
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
        // Replace returns empty body (content_response_on_write=false
        // semantics) + etag header \"v2\".
        let dispatcher = ScriptedDispatcher::new(vec![
            ScriptedReply::ok(
                br#"{"id":"doc1","pk":"pk1","visits":0,"_etag":"\"v1\""}"#.to_vec(),
                Some("\"v1\""),
                StatusCode::Ok,
            ),
            ScriptedReply::ok(Vec::new(), Some("\"v2\""), StatusCode::Ok),
        ]);

        let op = canonical_patch_op();
        let resp = execute_with_dispatcher(&dispatcher, op, OperationOptions::default(), None)
            .await
            .expect("PATCH should succeed");

        // Header carries the Replace's new etag (existing behavior).
        assert_eq!(
            resp.headers().etag.as_ref().map(|t| t.as_str()),
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
                    OperationType::Read => br#"{"id":"doc1","pk":"pk1","visits":0}"#.to_vec(),
                    OperationType::Replace => br#"{"id":"doc1","pk":"pk1","visits":1}"#.to_vec(),
                    other => panic!("unexpected sub-op {other:?}"),
                };
                let mut headers = CosmosResponseHeaders::new();
                headers.etag = Some(ETag::from("\"v1\""));
                let diagnostics = Arc::new(
                    DiagnosticsContextBuilder::new(
                        ActivityId::new_uuid(),
                        Arc::new(DiagnosticsOptions::default()),
                    )
                    .complete(),
                );
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
    }
}

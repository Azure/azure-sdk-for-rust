// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Preview Distributed Transaction wire models.
//!
//! **Preview / work in progress.** Gated behind the disabled-by-default
//! `preview_dtx` feature; the DTX service feature is not yet generally
//! available. These types may change or be removed without notice and are
//! **not supported for production use**.

use std::{borrow::Cow, sync::Arc};

use azure_core::fmt::SafeDebug;
use azure_core::http::headers::AsHeaders;
use azure_core::Bytes;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::diagnostics::DiagnosticsContext;
use crate::models::{
    ActivityId, ContainerReference, CosmosResponseHeaders, PartitionKey, Precondition,
    RequestCharge, SessionToken,
};

/// The coordinator behavior requested for a distributed transaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum DistributedTransactionType {
    /// Atomic write transaction.
    Write,
    /// Snapshot read transaction.
    Read,
}

impl DistributedTransactionType {
    /// Returns the string representation of this transaction type.
    ///
    /// This is a diagnostic/display label that mirrors the variant name. It is
    /// **not** the coordinator wire value: the `x-ms-cosmos-operation-type`
    /// header sends `CommitDistributedTransaction` for [`Self::Write`] and
    /// `Read` for [`Self::Read`] (built in `execute_distributed_transaction`).
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Write => "Write",
            Self::Read => "Read",
        }
    }
}

impl std::fmt::Display for DistributedTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl AsRef<str> for DistributedTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// The per-item operation kind in a distributed transaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum DistributedTransactionOperationKind {
    /// Create an item.
    Create,
    /// Read an item.
    Read,
    /// Replace an item.
    Replace,
    /// Upsert an item.
    Upsert,
    /// Delete an item.
    Delete,
    /// Patch an item.
    Patch,
}

impl DistributedTransactionOperationKind {
    pub(crate) fn as_wire_str(self) -> &'static str {
        match self {
            Self::Create => "Create",
            Self::Read => "Read",
            Self::Replace => "Replace",
            Self::Upsert => "Upsert",
            Self::Delete => "Delete",
            Self::Patch => "Patch",
        }
    }
}

/// A resolved item target for a distributed transaction operation.
#[derive(Clone, SafeDebug)]
#[safe(true)]
#[non_exhaustive]
pub struct DistributedTransactionTarget {
    /// Resolved target container.
    pub container: ContainerReference,
    /// Item partition key.
    pub partition_key: PartitionKey,
    /// Item id.
    pub id: Cow<'static, str>,
}

impl DistributedTransactionTarget {
    /// Creates a target from resolved container metadata, partition key, and item id.
    pub fn new(
        container: ContainerReference,
        partition_key: impl Into<PartitionKey>,
        id: impl Into<Cow<'static, str>>,
    ) -> Self {
        Self {
            container,
            partition_key: partition_key.into(),
            id: id.into(),
        }
    }
}

/// A single operation buffered into a distributed transaction request.
#[derive(Clone, SafeDebug)]
#[safe(true)]
#[non_exhaustive]
pub struct DistributedTransactionOperation {
    /// Operation type.
    pub kind: DistributedTransactionOperationKind,
    /// Operation target.
    pub target: DistributedTransactionTarget,
    /// Optional raw JSON resource body.
    pub resource_body: Option<Bytes>,
    /// Optional per-operation session token.
    pub session_token: Option<SessionToken>,
    /// Optional ETag precondition.
    pub precondition: Option<Precondition>,
    /// Optional SQL predicate for server-side conditional PATCH.
    pub patch_filter_predicate: Option<Cow<'static, str>>,
}

impl DistributedTransactionOperation {
    /// Creates an operation for the given kind and target.
    pub fn new(
        kind: DistributedTransactionOperationKind,
        target: DistributedTransactionTarget,
    ) -> Self {
        Self {
            kind,
            target,
            resource_body: None,
            session_token: None,
            precondition: None,
            patch_filter_predicate: None,
        }
    }

    /// Sets the raw JSON resource body.
    pub fn with_resource_body(mut self, body: impl Into<Bytes>) -> Self {
        self.resource_body = Some(body.into());
        self
    }

    /// Sets the per-operation session token.
    pub fn with_session_token(mut self, session_token: impl Into<SessionToken>) -> Self {
        self.session_token = Some(session_token.into());
        self
    }

    /// Sets the ETag precondition.
    pub fn with_precondition(mut self, precondition: Precondition) -> Self {
        self.precondition = Some(precondition);
        self
    }

    /// Sets a SQL predicate for conditional PATCH operations.
    pub fn with_patch_filter_predicate(mut self, predicate: impl Into<Cow<'static, str>>) -> Self {
        self.patch_filter_predicate = Some(predicate.into());
        self
    }
}

/// Immutable distributed transaction request data.
#[derive(Clone, SafeDebug)]
#[safe(true)]
#[non_exhaustive]
pub struct DistributedTransactionRequest {
    /// Transaction type.
    pub transaction_type: DistributedTransactionType,
    /// Operations to send to the coordinator.
    pub operations: Vec<DistributedTransactionOperation>,
    /// Idempotency token generated once and reused across driver retries.
    pub idempotency_token: Uuid,
}

impl DistributedTransactionRequest {
    /// Creates a request with a fresh idempotency token.
    pub fn new(
        transaction_type: DistributedTransactionType,
        operations: Vec<DistributedTransactionOperation>,
    ) -> Self {
        Self {
            transaction_type,
            operations,
            idempotency_token: Uuid::new_v4(),
        }
    }

    /// Serializes the request body expected by the current .NET-compatible DTX endpoint.
    pub fn serialize_body(&self) -> crate::error::Result<Vec<u8>> {
        self.validate()?;

        let operations = self
            .operations
            .iter()
            .enumerate()
            .map(serialize_operation)
            .collect::<crate::error::Result<Vec<_>>>()?;

        serde_json::to_vec(&serde_json::json!({ "operations": operations })).map_err(|error| {
            crate::error::CosmosError::builder()
                .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                .with_message("failed to serialize distributed transaction request body")
                .with_source(error)
                .build()
        })
    }

    fn validate(&self) -> crate::error::Result<()> {
        for operation in &self.operations {
            match (self.transaction_type, operation.kind) {
                (DistributedTransactionType::Read, DistributedTransactionOperationKind::Read) => {}
                (DistributedTransactionType::Read, other) => {
                    return Err(invalid_dtx_request(format!(
                        "distributed read transaction cannot contain {} operations",
                        other.as_wire_str()
                    )));
                }
                (DistributedTransactionType::Write, DistributedTransactionOperationKind::Read) => {
                    return Err(invalid_dtx_request(
                        "distributed write transaction cannot contain Read operations",
                    ));
                }
                (DistributedTransactionType::Write, _) => {}
            }
            if operation.kind.requires_resource_body() && operation.resource_body.is_none() {
                return Err(invalid_dtx_request(format!(
                    "distributed transaction {} operations require resourceBody",
                    operation.kind.as_wire_str()
                )));
            }
        }
        Ok(())
    }
}

impl DistributedTransactionOperationKind {
    fn requires_resource_body(self) -> bool {
        matches!(
            self,
            Self::Create | Self::Replace | Self::Upsert | Self::Patch
        )
    }
}

fn invalid_dtx_request(message: impl Into<String>) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::new(
            azure_core::http::StatusCode::BadRequest,
        ))
        .with_message(message.into())
        .build()
}

fn serialize_operation(
    (index, operation): (usize, &DistributedTransactionOperation),
) -> crate::error::Result<serde_json::Value> {
    let mut object = serde_json::Map::new();
    let target = &operation.target;

    object.insert(
        "databaseName".to_owned(),
        serde_json::Value::String(target.container.database_name().to_owned()),
    );
    object.insert(
        "collectionName".to_owned(),
        serde_json::Value::String(target.container.name().to_owned()),
    );
    object.insert(
        "id".to_owned(),
        serde_json::Value::String(target.id.to_string()),
    );
    object.insert(
        "collectionResourceId".to_owned(),
        serde_json::Value::String(target.container.rid().to_owned()),
    );
    object.insert(
        "databaseResourceId".to_owned(),
        serde_json::Value::String(target.container.database_rid().to_owned()),
    );
    object.insert("partitionKey".to_owned(), partition_key_json(target)?);
    object.insert("index".to_owned(), serde_json::json!(index));

    if let Some(body) = operation.resource_body.as_ref() {
        let mut resource_body =
            serde_json::from_slice::<serde_json::Value>(body).map_err(|error| {
                crate::error::CosmosError::builder()
                    .with_status(crate::error::CosmosStatus::SERIALIZATION_RESPONSE_BODY_INVALID)
                    .with_message(
                        "distributed transaction operation resource body must be valid JSON",
                    )
                    .with_source(error)
                    .build()
            })?;
        if let Some(predicate) = operation.patch_filter_predicate.as_ref() {
            match &mut resource_body {
                serde_json::Value::Object(map) => {
                    map.insert(
                        "condition".to_owned(),
                        serde_json::Value::String(predicate.to_string()),
                    );
                }
                _ => {
                    return Err(crate::error::CosmosError::builder()
                        .with_status(crate::error::CosmosStatus::new(
                            azure_core::http::StatusCode::BadRequest,
                        ))
                        .with_message(
                            "distributed transaction patch resource body must be a JSON object when a filter predicate is set",
                        )
                        .build());
                }
            }
        }
        validate_resource_body_id(operation, &resource_body)?;
        object.insert("resourceBody".to_owned(), resource_body);
    }

    if let Some(session_token) = operation.session_token.as_ref() {
        if !session_token.as_str().trim().is_empty() {
            object.insert(
                "sessionToken".to_owned(),
                serde_json::Value::String(session_token.as_str().to_owned()),
            );
        }
    }

    if let Some(precondition) = operation.precondition.as_ref() {
        match precondition {
            Precondition::IfMatch(etag) => {
                object.insert(
                    "ifMatch".to_owned(),
                    serde_json::Value::String(etag.to_string()),
                );
            }
            Precondition::IfNoneMatch(etag) => {
                object.insert(
                    "ifNoneMatch".to_owned(),
                    serde_json::Value::String(etag.to_string()),
                );
            }
        }
    }

    object.insert(
        "operationType".to_owned(),
        serde_json::Value::String(operation.kind.as_wire_str().to_owned()),
    );
    object.insert(
        "resourceType".to_owned(),
        serde_json::Value::String("Document".to_owned()),
    );

    Ok(serde_json::Value::Object(object))
}

fn validate_resource_body_id(
    operation: &DistributedTransactionOperation,
    resource_body: &serde_json::Value,
) -> crate::error::Result<()> {
    if !matches!(
        operation.kind,
        DistributedTransactionOperationKind::Create
            | DistributedTransactionOperationKind::Replace
            | DistributedTransactionOperationKind::Upsert
    ) {
        return Ok(());
    }

    match resource_body.get("id").and_then(|value| value.as_str()) {
        Some(body_id) if body_id == operation.target.id.as_ref() => Ok(()),
        Some(body_id) => Err(invalid_dtx_request(format!(
            "distributed transaction operation resourceBody.id ('{body_id}') must match operation id ('{}')",
            operation.target.id
        ))),
        None => Err(invalid_dtx_request(
            "distributed transaction create, replace, and upsert operations require resourceBody.id",
        )),
    }
}

fn partition_key_json(
    target: &DistributedTransactionTarget,
) -> crate::error::Result<serde_json::Value> {
    let (_, value) = target
        .partition_key
        .as_headers()?
        .next()
        .ok_or_else(|| invalid_partition_key("partition key did not produce a header value"))?;
    let text = value.as_str();
    serde_json::from_str(text).map_err(|error| {
        crate::error::CosmosError::builder()
            .with_status(crate::error::CosmosStatus::new(
                azure_core::http::StatusCode::BadRequest,
            ))
            .with_message("distributed transaction operations require a non-empty partition key")
            .with_source(error)
            .build()
    })
}

fn invalid_partition_key(message: impl Into<String>) -> crate::error::CosmosError {
    let message = message.into();
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::new(
            azure_core::http::StatusCode::BadRequest,
        ))
        .with_message(message)
        .build()
}

/// The resource body returned for one distributed transaction operation.
#[derive(Clone, SafeDebug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub enum DistributedTransactionResultBody {
    /// No resource body was returned.
    #[default]
    None,
    /// Raw JSON payload returned by the coordinator.
    Bytes(Bytes),
}

/// Result of one operation in a distributed transaction response.
#[derive(Clone, SafeDebug)]
#[safe(true)]
#[non_exhaustive]
pub struct DistributedTransactionOperationResult {
    /// Raw operation response object returned by the coordinator.
    #[safe(false)]
    pub raw_response: serde_json::Map<String, serde_json::Value>,
    /// Zero-based request operation index.
    pub index: usize,
    /// HTTP status code for this operation.
    pub status_code: azure_core::http::StatusCode,
    /// Cosmos sub-status code, when present.
    pub sub_status_code: Option<crate::models::SubStatusCode>,
    /// ETag returned for this operation, when present.
    pub etag: Option<azure_core::http::Etag>,
    /// Per-operation session token returned by the coordinator, when present.
    pub session_token: Option<SessionToken>,
    /// Partition key range id returned by the coordinator, when present.
    pub partition_key_range_id: Option<String>,
    /// Request charge for this operation.
    pub request_charge: Option<RequestCharge>,
    /// Operation resource body.
    #[safe(false)]
    pub resource_body: DistributedTransactionResultBody,
}

impl DistributedTransactionOperationResult {
    /// Returns `true` when the operation status is in the 2xx range.
    pub fn is_success_status_code(&self) -> bool {
        self.status_code.is_success()
    }

    /// Returns `true` when the operation is a completed DTX success outcome.
    ///
    /// Read transactions treat `304 NotModified` as a completed success code
    /// (no body), even though it is not in the HTTP 2xx success range.
    pub fn is_completed_status_code(&self) -> bool {
        is_dtx_completed_status_code(self.status_code)
    }
}

/// Response returned by the distributed transaction coordinator.
#[derive(Clone, SafeDebug)]
#[safe(true)]
#[non_exhaustive]
pub struct DistributedTransactionResponse {
    /// Overall transaction status.
    pub status_code: azure_core::http::StatusCode,
    /// Overall transaction sub-status, when present.
    pub sub_status_code: Option<crate::models::SubStatusCode>,
    /// Operation results ordered by request index.
    pub operation_results: Vec<DistributedTransactionOperationResult>,
    /// Idempotency token used for write retries.
    pub idempotency_token: Uuid,
    /// Parsed Cosmos response headers returned by the coordinator.
    pub headers: CosmosResponseHeaders,
    /// Activity ID returned by the service, when present.
    pub activity_id: Option<ActivityId>,
    /// Total request charge returned by the service, when present.
    pub request_charge: Option<RequestCharge>,
    /// Retry-after hint in milliseconds, when present.
    pub retry_after_ms: Option<u64>,
    /// Diagnostics captured while executing the transaction.
    pub diagnostics: Option<Arc<DiagnosticsContext>>,
    /// Whether the coordinator says the same idempotency token can be retried.
    pub is_retriable: bool,
    /// Coordinator diagnostic string, when present.
    pub diagnostic_string: Option<String>,
    /// Error message synthesized by the driver for malformed coordinator responses.
    pub error_message: Option<String>,
}

impl DistributedTransactionResponse {
    /// Parses a coordinator response body into a distributed transaction response.
    pub fn from_body(
        status_code: azure_core::http::StatusCode,
        sub_status_code: Option<crate::models::SubStatusCode>,
        body: &[u8],
        operation_count: usize,
        idempotency_token: Uuid,
    ) -> Self {
        let is_success_envelope = is_dtx_completed_status_code(status_code);
        let mut is_retriable = false;
        let mut diagnostic_string = None;

        if body.is_empty() {
            return Self::fallback(
                is_success_envelope,
                status_code,
                sub_status_code,
                operation_count,
                idempotency_token,
                is_retriable,
                diagnostic_string,
                None,
                "server response deserialization failure",
            );
        }

        let root: serde_json::Value = match serde_json::from_slice(body) {
            Ok(root) => root,
            Err(_) => {
                return Self::fallback(
                    is_success_envelope,
                    status_code,
                    sub_status_code,
                    operation_count,
                    idempotency_token,
                    is_retriable,
                    diagnostic_string,
                    None,
                    "server response deserialization failure",
                );
            }
        };

        is_retriable = get_property(&root, "isRetriable")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        diagnostic_string = get_property(&root, "diagnosticString")
            .and_then(|v| v.as_str())
            .map(ToOwned::to_owned);
        // `get_property` already matches keys case-insensitively, so a single
        // lookup per name covers both `message`/`Message` and `code`/`Code`.
        let service_error_message = get_property(&root, "message")
            .and_then(|v| v.as_str())
            .map(ToOwned::to_owned)
            .or_else(|| {
                get_property(&root, "code")
                    .and_then(|v| v.as_str())
                    .map(ToOwned::to_owned)
            });

        let operation_responses =
            get_property(&root, "operationResponses").and_then(|v| v.as_array());
        let Some(operation_responses) = operation_responses else {
            return Self::fallback(
                is_success_envelope,
                status_code,
                sub_status_code,
                operation_count,
                idempotency_token,
                is_retriable,
                diagnostic_string,
                service_error_message,
                "invalid server response",
            );
        };

        let mut parsed_results = Vec::with_capacity(operation_responses.len());
        for value in operation_responses {
            match parse_operation_result(value) {
                Ok(result) => parsed_results.push(result),
                Err(()) => {
                    return Self::fallback(
                        is_success_envelope,
                        status_code,
                        sub_status_code,
                        operation_count,
                        idempotency_token,
                        is_retriable,
                        diagnostic_string,
                        service_error_message,
                        "server response deserialization failure",
                    );
                }
            }
        }

        if parsed_results.len() != operation_count {
            return Self::fallback(
                is_success_envelope,
                status_code,
                sub_status_code,
                operation_count,
                idempotency_token,
                is_retriable,
                diagnostic_string,
                service_error_message,
                "invalid server response",
            );
        }

        let Some(mut ordered) = reorder_results(parsed_results, operation_count) else {
            return Self::fallback(
                is_success_envelope,
                status_code,
                sub_status_code,
                operation_count,
                idempotency_token,
                is_retriable,
                diagnostic_string,
                service_error_message,
                "server response deserialization failure",
            );
        };

        // Promote the MultiStatus envelope AFTER reordering so the promoted
        // status reflects the first failing operation in *request* order,
        // matching .NET's DistributedTransactionResponse (PR #5974). Promoting
        // before reorder would select the first failure in wire order, which
        // differs when the coordinator returns results out of order.
        let (status_code, sub_status_code) =
            promote_multistatus(status_code, sub_status_code, &ordered);

        Self {
            status_code,
            sub_status_code,
            operation_results: std::mem::take(&mut ordered),
            idempotency_token,
            headers: CosmosResponseHeaders::default(),
            activity_id: None,
            request_charge: None,
            retry_after_ms: None,
            diagnostics: None,
            is_retriable,
            diagnostic_string,
            error_message: None,
        }
    }

    /// Returns `true` when the overall status is in the 2xx range.
    pub fn is_success_status_code(&self) -> bool {
        self.status_code.is_success()
    }

    /// Returns `true` when the overall transaction completed successfully.
    ///
    /// This differs from [`Self::is_success_status_code`] for read transactions:
    /// an all-`304 NotModified` snapshot is complete and terminal, but `304` is
    /// not an HTTP 2xx success status.
    pub fn is_completed_status_code(&self) -> bool {
        is_dtx_completed_status_code(self.status_code)
    }

    /// Number of operation results.
    pub fn len(&self) -> usize {
        self.operation_results.len()
    }

    /// Returns `true` when there are no operation results.
    pub fn is_empty(&self) -> bool {
        self.operation_results.is_empty()
    }

    pub(crate) fn with_response_headers(mut self, headers: &CosmosResponseHeaders) -> Self {
        self.headers = headers.clone();
        if let Some(token) = headers.distributed_transaction_idempotency_token {
            self.idempotency_token = token;
        }
        self.activity_id = headers.activity_id.clone();
        self.request_charge = headers.request_charge;
        self.retry_after_ms = headers.retry_after_ms;
        self
    }

    pub(crate) fn with_diagnostics(mut self, diagnostics: Arc<DiagnosticsContext>) -> Self {
        self.diagnostics = Some(diagnostics);
        self
    }

    /// Builds a fail-closed / padded response for an uninterpretable coordinator body.
    ///
    /// On a completed-status envelope (`is_success_envelope`) an unparseable body
    /// is treated as a hard failure and synthesized as `500 InternalServerError`
    /// (`fail_closed`); on a non-success envelope the results are padded with the
    /// envelope status so the caller still sees one result per operation.
    #[allow(clippy::too_many_arguments)]
    fn fallback(
        is_success_envelope: bool,
        status_code: azure_core::http::StatusCode,
        sub_status_code: Option<crate::models::SubStatusCode>,
        operation_count: usize,
        idempotency_token: Uuid,
        is_retriable: bool,
        diagnostic_string: Option<String>,
        service_error_message: Option<String>,
        reason: &'static str,
    ) -> Self {
        if is_success_envelope {
            Self::fail_closed(
                operation_count,
                idempotency_token,
                is_retriable,
                diagnostic_string,
                reason,
            )
        } else {
            Self::padded(
                status_code,
                sub_status_code,
                operation_count,
                idempotency_token,
                is_retriable,
                diagnostic_string,
                service_error_message,
            )
        }
    }

    fn fail_closed(
        operation_count: usize,
        idempotency_token: Uuid,
        is_retriable: bool,
        diagnostic_string: Option<String>,
        error_message: impl Into<String>,
    ) -> Self {
        Self::padded(
            azure_core::http::StatusCode::InternalServerError,
            None,
            operation_count,
            idempotency_token,
            is_retriable,
            diagnostic_string,
            Some(error_message.into()),
        )
    }

    fn padded(
        status_code: azure_core::http::StatusCode,
        sub_status_code: Option<crate::models::SubStatusCode>,
        operation_count: usize,
        idempotency_token: Uuid,
        is_retriable: bool,
        diagnostic_string: Option<String>,
        error_message: Option<String>,
    ) -> Self {
        let operation_results = (0..operation_count)
            .map(|index| DistributedTransactionOperationResult {
                raw_response: raw_operation_response(index, status_code, sub_status_code),
                index,
                status_code,
                sub_status_code,
                etag: None,
                session_token: None,
                partition_key_range_id: None,
                request_charge: None,
                resource_body: DistributedTransactionResultBody::None,
            })
            .collect();

        Self {
            status_code,
            sub_status_code,
            operation_results,
            idempotency_token,
            headers: CosmosResponseHeaders::default(),
            activity_id: None,
            request_charge: None,
            retry_after_ms: None,
            diagnostics: None,
            is_retriable,
            diagnostic_string,
            error_message,
        }
    }
}

fn get_property<'a>(value: &'a serde_json::Value, name: &str) -> Option<&'a serde_json::Value> {
    value.as_object()?.iter().find_map(|(key, value)| {
        if key.eq_ignore_ascii_case(name) {
            Some(value)
        } else {
            None
        }
    })
}

fn is_dtx_completed_status_code(status_code: azure_core::http::StatusCode) -> bool {
    status_code.is_success() || status_code == azure_core::http::StatusCode::NotModified
}

fn parse_operation_result(
    value: &serde_json::Value,
) -> Result<DistributedTransactionOperationResult, ()> {
    let raw_response = value.as_object().cloned().ok_or(())?;
    let index = get_property(value, "index")
        .and_then(|v| v.as_u64())
        .and_then(|v| usize::try_from(v).ok())
        .ok_or(())?;
    let status_code = get_property(value, "statusCode")
        .and_then(|v| v.as_u64())
        .and_then(|v| u16::try_from(v).ok())
        .map(azure_core::http::StatusCode::from)
        .ok_or(())?;
    let sub_status_code = get_property(value, "subStatusCode")
        .and_then(|v| v.as_u64())
        .and_then(|v| u16::try_from(v).ok())
        .map(crate::models::SubStatusCode::new);
    let etag = get_property(value, "etag")
        .and_then(|v| v.as_str())
        .map(|s| azure_core::http::Etag::from(s.to_owned()));
    let session_token = get_property(value, "sessionToken")
        .and_then(|v| v.as_str())
        .map(|s| SessionToken::new(s.to_owned()));
    let partition_key_range_id = get_property(value, "partitionKeyRangeId")
        .and_then(|v| v.as_str())
        .map(ToOwned::to_owned);
    let request_charge = get_property(value, "requestCharge")
        .and_then(|v| v.as_f64())
        .map(RequestCharge::new);
    let resource_body = match get_property(value, "resourceBody") {
        Some(serde_json::Value::Null) | None => DistributedTransactionResultBody::None,
        Some(body) => {
            if !body.is_object() {
                return Err(());
            }
            let bytes = serde_json::to_vec(body).map_err(|_| ())?;
            DistributedTransactionResultBody::Bytes(Bytes::from(bytes))
        }
    };

    Ok(DistributedTransactionOperationResult {
        raw_response,
        index,
        status_code,
        sub_status_code,
        etag,
        session_token,
        partition_key_range_id,
        request_charge,
        resource_body,
    })
}

fn raw_operation_response(
    index: usize,
    status_code: azure_core::http::StatusCode,
    sub_status_code: Option<crate::models::SubStatusCode>,
) -> serde_json::Map<String, serde_json::Value> {
    let mut raw_response = serde_json::Map::new();
    raw_response.insert("index".to_owned(), serde_json::json!(index));
    raw_response.insert(
        "statusCode".to_owned(),
        serde_json::json!(u16::from(status_code)),
    );
    if let Some(sub_status_code) = sub_status_code {
        raw_response.insert(
            "subStatusCode".to_owned(),
            serde_json::json!(sub_status_code.value()),
        );
    }
    raw_response
}

fn reorder_results(
    results: Vec<DistributedTransactionOperationResult>,
    operation_count: usize,
) -> Option<Vec<DistributedTransactionOperationResult>> {
    let mut ordered = vec![None; operation_count];
    for result in results {
        let index = result.index;
        if index >= operation_count || ordered[index].is_some() {
            return None;
        }
        ordered[index] = Some(result);
    }
    ordered.into_iter().collect()
}

/// Promotes a `207 MultiStatus` envelope to the status of the first failing
/// operation (status `>= 400`, excluding `424 FailedDependency`).
///
/// Callers must pass results already reordered by request index so promotion
/// selects the first failure in *request* order, matching .NET (PR #5974).
/// Non-`207` envelopes are returned unchanged.
fn promote_multistatus(
    status_code: azure_core::http::StatusCode,
    sub_status_code: Option<crate::models::SubStatusCode>,
    results: &[DistributedTransactionOperationResult],
) -> (
    azure_core::http::StatusCode,
    Option<crate::models::SubStatusCode>,
) {
    if u16::from(status_code) != 207 {
        return (status_code, sub_status_code);
    }

    for result in results {
        let code = u16::from(result.status_code);
        if code != 424 && code >= 400 {
            return (result.status_code, result.sub_status_code);
        }
    }

    (status_code, sub_status_code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        AccountReference, ContainerProperties, PartitionKeyDefinition, SystemProperties,
    };
    use azure_core::http::Etag;
    use url::Url;

    fn container() -> ContainerReference {
        let account = AccountReference::with_master_key(
            Url::parse("https://example.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        );
        let pk_def: PartitionKeyDefinition = serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap();
        let properties = ContainerProperties {
            id: "coll".into(),
            partition_key: pk_def,
            system_properties: SystemProperties::default(),
        };
        ContainerReference::new(account, "db", "db_rid", "coll", "coll_rid", &properties)
    }

    fn target(id: &str) -> DistributedTransactionTarget {
        DistributedTransactionTarget::new(container(), PartitionKey::from("pk1"), id.to_owned())
    }

    #[test]
    fn serialize_create_operation() {
        let operation = DistributedTransactionOperation::new(
            DistributedTransactionOperationKind::Create,
            target("item1"),
        )
        .with_resource_body(Bytes::from_static(br#"{"id":"item1","pk":"pk1"}"#))
        .with_session_token("0:1#9#4=8")
        .with_precondition(Precondition::if_match(Etag::from("\"etag\"")));
        let request =
            DistributedTransactionRequest::new(DistributedTransactionType::Write, vec![operation]);

        let body = request.serialize_body().unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let op = &json["operations"][0];

        assert_eq!(op["databaseName"], "db");
        assert_eq!(op["collectionName"], "coll");
        assert_eq!(op["collectionResourceId"], "coll_rid");
        assert_eq!(op["databaseResourceId"], "db_rid");
        assert_eq!(op["partitionKey"], serde_json::json!(["pk1"]));
        assert_eq!(op["index"], 0);
        assert_eq!(op["resourceBody"]["id"], "item1");
        assert_eq!(op["sessionToken"], "0:1#9#4=8");
        assert_eq!(op["ifMatch"], "\"etag\"");
        assert_eq!(op["operationType"], "Create");
        assert_eq!(op["resourceType"], "Document");
        assert!(json.get("operationType").is_none());
    }

    #[test]
    fn serialize_read_omits_body_and_empty_session_token() {
        let operation = DistributedTransactionOperation::new(
            DistributedTransactionOperationKind::Read,
            target("item1"),
        )
        .with_session_token("   ")
        .with_precondition(Precondition::if_none_match(Etag::from("\"etag\"")));
        let request =
            DistributedTransactionRequest::new(DistributedTransactionType::Read, vec![operation]);

        let body = request.serialize_body().unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        let op = &json["operations"][0];

        assert!(op.get("resourceBody").is_none());
        assert!(op.get("sessionToken").is_none());
        assert_eq!(op["ifNoneMatch"], "\"etag\"");
        assert_eq!(op["operationType"], "Read");
    }

    #[test]
    fn serialize_patch_adds_condition() {
        let operation = DistributedTransactionOperation::new(
            DistributedTransactionOperationKind::Patch,
            target("item1"),
        )
        .with_resource_body(Bytes::from_static(br#"{"operations":[]}"#))
        .with_patch_filter_predicate("from c where c.status = 'pending'");
        let request =
            DistributedTransactionRequest::new(DistributedTransactionType::Write, vec![operation]);

        let body = request.serialize_body().unwrap();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(
            json["operations"][0]["resourceBody"]["condition"],
            "from c where c.status = 'pending'"
        );
    }

    #[test]
    fn parse_out_of_order_results_reorders_by_index() {
        let body = br#"{"operationResponses":[{"index":2,"statusCode":204},{"index":0,"statusCode":201,"Etag":"e0"},{"index":1,"statusCode":200}]}"#;
        let response = DistributedTransactionResponse::from_body(
            azure_core::http::StatusCode::Ok,
            None,
            body,
            3,
            Uuid::nil(),
        );

        assert_eq!(response.status_code, azure_core::http::StatusCode::Ok);
        assert_eq!(response.operation_results[0].index, 0);
        assert_eq!(response.operation_results[1].index, 1);
        assert_eq!(response.operation_results[2].index, 2);
        assert_eq!(
            response.operation_results[0].status_code,
            azure_core::http::StatusCode::Created
        );
    }

    #[test]
    fn parse_duplicate_index_success_fails_closed() {
        let body = br#"{"isRetriable":true,"diagnosticString":"diag","operationResponses":[{"index":0,"statusCode":201},{"index":1,"statusCode":201},{"index":1,"statusCode":201}]}"#;
        let response = DistributedTransactionResponse::from_body(
            azure_core::http::StatusCode::Ok,
            None,
            body,
            3,
            Uuid::nil(),
        );

        assert_eq!(
            response.status_code,
            azure_core::http::StatusCode::InternalServerError
        );
        assert!(response.is_retriable);
        assert_eq!(response.diagnostic_string.as_deref(), Some("diag"));
        assert_eq!(response.len(), 3);
        assert!(response
            .operation_results
            .iter()
            .all(|result| result.status_code == azure_core::http::StatusCode::InternalServerError));
    }

    #[test]
    fn parse_duplicate_index_error_pads_with_envelope_status() {
        let body = br#"{"operationResponses":[{"index":0,"statusCode":201},{"index":1,"statusCode":201},{"index":1,"statusCode":201}]}"#;
        let response = DistributedTransactionResponse::from_body(
            azure_core::http::StatusCode::Conflict,
            None,
            body,
            3,
            Uuid::nil(),
        );

        assert_eq!(response.status_code, azure_core::http::StatusCode::Conflict);
        assert_eq!(response.len(), 3);
        assert!(response
            .operation_results
            .iter()
            .all(|result| result.status_code == azure_core::http::StatusCode::Conflict));
    }

    #[test]
    fn parse_fewer_results_error_pads_with_envelope_status() {
        // The coordinator returned fewer per-op results than operations on a
        // non-success envelope: the payload is uninterpretable, so it is padded
        // to one result per operation, each carrying the envelope status.
        let body = br#"{"operationResponses":[{"index":0,"statusCode":201},{"index":1,"statusCode":201}]}"#;
        let response = DistributedTransactionResponse::from_body(
            azure_core::http::StatusCode::Conflict,
            None,
            body,
            3,
            Uuid::nil(),
        );

        assert_eq!(response.status_code, azure_core::http::StatusCode::Conflict);
        assert_eq!(response.len(), 3);
        assert!(response
            .operation_results
            .iter()
            .all(|result| result.status_code == azure_core::http::StatusCode::Conflict));
    }

    #[test]
    fn parse_fewer_results_success_fails_closed() {
        // The same short payload on a success envelope must fail closed to `500`
        // rather than return a success with unverifiable per-operation data.
        let body = br#"{"operationResponses":[{"index":0,"statusCode":201},{"index":1,"statusCode":201}]}"#;
        let response = DistributedTransactionResponse::from_body(
            azure_core::http::StatusCode::Ok,
            None,
            body,
            3,
            Uuid::nil(),
        );

        assert_eq!(
            response.status_code,
            azure_core::http::StatusCode::InternalServerError
        );
        assert_eq!(response.len(), 3);
    }

    #[test]
    fn parse_multistatus_promotes_first_failure_in_request_order() {
        // Wire order is [idx2=503, idx0=201, idx1=409]. After reordering by
        // index the request order is [201, 409, 503], so the first failing
        // operation in request order is index 1 (409 Conflict) — not the 503
        // that happens to appear first on the wire. Matches .NET PR #5974.
        let body = br#"{"operationResponses":[{"index":2,"statusCode":503},{"index":0,"statusCode":201},{"index":1,"statusCode":409}]}"#;
        let response = DistributedTransactionResponse::from_body(
            azure_core::http::StatusCode::from(207_u16),
            None,
            body,
            3,
            Uuid::nil(),
        );

        assert_eq!(response.status_code, azure_core::http::StatusCode::Conflict);
        assert_eq!(
            response.operation_results[1].status_code,
            azure_core::http::StatusCode::Conflict
        );
        assert_eq!(
            response.operation_results[2].status_code,
            azure_core::http::StatusCode::ServiceUnavailable
        );
    }

    #[test]
    fn parse_write_abort_keeps_failure_and_rolled_back_siblings() {
        // 452 abort: op0 was prepared then rolled back (453 / 5415); op1 is the
        // real failure (409). 452 is not a 207, so the envelope is preserved and
        // each operation keeps its own status.
        let body = br#"{"isRetriable":false,"operationResponses":[{"index":0,"statusCode":453,"subStatusCode":5415},{"index":1,"statusCode":409}]}"#;
        let response = DistributedTransactionResponse::from_body(
            azure_core::http::StatusCode::from(452_u16),
            None,
            body,
            2,
            Uuid::nil(),
        );

        assert_eq!(u16::from(response.status_code), 452);
        assert_eq!(u16::from(response.operation_results[0].status_code), 453);
        assert_eq!(
            response.operation_results[0].sub_status_code,
            Some(crate::models::SubStatusCode::new(5415))
        );
        assert_eq!(
            response.operation_results[1].status_code,
            azure_core::http::StatusCode::Conflict
        );
    }

    #[test]
    fn parse_read_multistatus_skips_failed_dependency_and_promotes_real_failure() {
        // 207 read snapshot failure: op0 was rewritten to 424 FailedDependency,
        // op1 is the real 404. Promotion skips 424 and surfaces the 404.
        let body = br#"{"operationResponses":[{"index":0,"statusCode":424},{"index":1,"statusCode":404}]}"#;
        let response = DistributedTransactionResponse::from_body(
            azure_core::http::StatusCode::from(207_u16),
            None,
            body,
            2,
            Uuid::nil(),
        );

        assert_eq!(response.status_code, azure_core::http::StatusCode::NotFound);
        assert_eq!(u16::from(response.operation_results[0].status_code), 424);
        assert_eq!(
            response.operation_results[1].status_code,
            azure_core::http::StatusCode::NotFound
        );
    }

    #[test]
    fn serialize_patch_with_non_object_body_and_predicate_fails() {
        let operation = DistributedTransactionOperation::new(
            DistributedTransactionOperationKind::Patch,
            target("item1"),
        )
        .with_resource_body(Bytes::from_static(br#"[1,2,3]"#))
        .with_patch_filter_predicate("from c where c.status = 'pending'");
        let request =
            DistributedTransactionRequest::new(DistributedTransactionType::Write, vec![operation]);

        let error = request.serialize_body().unwrap_err();
        assert_eq!(
            error.status().status_code(),
            azure_core::http::StatusCode::BadRequest
        );
    }

    #[test]
    fn serialize_rejects_invalid_transaction_type_operation_kind_combinations() {
        let write_in_read = DistributedTransactionOperation::new(
            DistributedTransactionOperationKind::Create,
            target("item1"),
        )
        .with_resource_body(Bytes::from_static(br#"{"id":"item1","pk":"pk1"}"#));
        let request = DistributedTransactionRequest::new(
            DistributedTransactionType::Read,
            vec![write_in_read],
        );
        assert_eq!(
            request.serialize_body().unwrap_err().status().status_code(),
            azure_core::http::StatusCode::BadRequest
        );

        let read_in_write = DistributedTransactionOperation::new(
            DistributedTransactionOperationKind::Read,
            target("item1"),
        );
        let request = DistributedTransactionRequest::new(
            DistributedTransactionType::Write,
            vec![read_in_write],
        );
        assert_eq!(
            request.serialize_body().unwrap_err().status().status_code(),
            azure_core::http::StatusCode::BadRequest
        );
    }

    #[test]
    fn serialize_rejects_create_replace_upsert_body_id_mismatch() {
        for kind in [
            DistributedTransactionOperationKind::Create,
            DistributedTransactionOperationKind::Replace,
            DistributedTransactionOperationKind::Upsert,
        ] {
            let operation = DistributedTransactionOperation::new(kind, target("item1"))
                .with_resource_body(Bytes::from_static(br#"{"id":"other","pk":"pk1"}"#));
            let request = DistributedTransactionRequest::new(
                DistributedTransactionType::Write,
                vec![operation],
            );
            assert_eq!(
                request.serialize_body().unwrap_err().status().status_code(),
                azure_core::http::StatusCode::BadRequest
            );
        }
    }

    #[test]
    fn serialize_rejects_missing_resource_body_for_body_required_write_operations() {
        for kind in [
            DistributedTransactionOperationKind::Create,
            DistributedTransactionOperationKind::Replace,
            DistributedTransactionOperationKind::Upsert,
            DistributedTransactionOperationKind::Patch,
        ] {
            let operation = DistributedTransactionOperation::new(kind, target("item1"));
            let request = DistributedTransactionRequest::new(
                DistributedTransactionType::Write,
                vec![operation],
            );
            let error = request.serialize_body().unwrap_err();
            assert_eq!(
                error.status().status_code(),
                azure_core::http::StatusCode::BadRequest,
                "{kind:?} should reject missing resourceBody"
            );
            assert!(
                error.to_string().contains("require resourceBody"),
                "{kind:?} produced unexpected error: {error}"
            );
        }
    }

    #[test]
    fn parse_all_not_modified_read_transaction_is_completed() {
        let body = br#"{"operationResponses":[{"index":0,"statusCode":304},{"index":1,"statusCode":304}]}"#;
        let response = DistributedTransactionResponse::from_body(
            azure_core::http::StatusCode::NotModified,
            None,
            body,
            2,
            Uuid::nil(),
        );

        assert!(!response.is_success_status_code());
        assert!(response.is_completed_status_code());
        assert!(response
            .operation_results
            .iter()
            .all(DistributedTransactionOperationResult::is_completed_status_code));
    }

    #[test]
    fn request_is_a_stable_replay_across_retries() {
        // The outer retry loop re-sends the request with the same idempotency
        // token and re-serializes the body on each attempt, so both must be
        // deterministic: a retry then replays byte-for-byte and the server
        // treats it as the same commit rather than a duplicate.
        let operation = DistributedTransactionOperation::new(
            DistributedTransactionOperationKind::Create,
            target("item1"),
        )
        .with_resource_body(Bytes::from_static(br#"{"id":"item1","pk":"pk1"}"#));
        let request =
            DistributedTransactionRequest::new(DistributedTransactionType::Write, vec![operation]);

        let token = request.idempotency_token;
        let first = request.serialize_body().unwrap();
        let second = request.serialize_body().unwrap();

        assert_eq!(
            first, second,
            "serialized body must be identical across retries"
        );
        assert_eq!(
            request.idempotency_token, token,
            "idempotency token must be stable across retries"
        );
    }

    #[test]
    fn response_headers_prefer_valid_idempotency_token_header() {
        let request_token = Uuid::new_v4();
        let response_token = Uuid::new_v4();
        let mut raw_headers = azure_core::http::headers::Headers::new();
        raw_headers.insert("x-ms-cosmos-idempotency-token", response_token.to_string());
        let headers = CosmosResponseHeaders::from_headers(&raw_headers);

        let response = DistributedTransactionResponse::from_body(
            azure_core::http::StatusCode::Ok,
            None,
            br#"{"operationResponses":[]}"#,
            0,
            request_token,
        )
        .with_response_headers(&headers);

        assert_eq!(response.idempotency_token, response_token);
    }

    #[test]
    fn response_headers_fall_back_to_request_idempotency_token() {
        for header_value in [None, Some("not-a-uuid")] {
            let request_token = Uuid::new_v4();
            let mut raw_headers = azure_core::http::headers::Headers::new();
            if let Some(header_value) = header_value {
                raw_headers.insert("x-ms-cosmos-idempotency-token", header_value);
            }
            let headers = CosmosResponseHeaders::from_headers(&raw_headers);

            let response = DistributedTransactionResponse::from_body(
                azure_core::http::StatusCode::Ok,
                None,
                br#"{"operationResponses":[]}"#,
                0,
                request_token,
            )
            .with_response_headers(&headers);

            assert_eq!(response.idempotency_token, request_token);
        }
    }
}

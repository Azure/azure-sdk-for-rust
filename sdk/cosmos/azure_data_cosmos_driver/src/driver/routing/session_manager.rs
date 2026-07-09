// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! High-level session token management for the operation pipeline.
//!
//! [`SessionManager`] wraps [`SessionContainer`] and provides consistency-gated
//! resolve / capture operations that the pipeline calls directly.

#[cfg(feature = "preview_dtx")]
use crate::models::partition_key_range::PartitionKeyRange;
use crate::models::{
    CosmosOperation, CosmosResponseHeaders, OperationType, ResourceType, SessionToken,
};

use super::session_container::SessionContainer;

/// Determines whether a given resource type + operation type combination targets
/// the master partition (metadata), meaning session tokens should NOT be
/// captured from its response.
///
/// This mirrors Java's `ReplicatedResourceClientUtils.isReadingFromMaster()`.
/// Most metadata resources always target master, but `DocumentCollection` is
/// special: only ReadFeed/Query/SqlQuery go to master. CRUD operations like
/// Create/Replace/Delete/Read target data partitions and should capture session
/// tokens.
fn is_reading_from_master(resource_type: ResourceType, operation_type: OperationType) -> bool {
    match resource_type {
        ResourceType::DatabaseAccount | ResourceType::Database | ResourceType::Offer => true,

        ResourceType::PartitionKeyRange => true,

        ResourceType::DocumentCollection => matches!(
            operation_type,
            OperationType::ReadFeed | OperationType::Query | OperationType::SqlQuery
        ),

        // Data-plane resources: Document, StoredProcedure, Trigger, UDF
        _ => false,
    }
}

/// Manages session token resolution and capture for the operation pipeline.
///
/// This type sits between the pipeline loop and the underlying
/// [`SessionContainer`] cache, adding metadata gating (container reference
/// checks, user-provided token precedence) on top of raw cache operations.
#[derive(Debug)]
pub(crate) struct SessionManager {
    container: SessionContainer,
}

impl SessionManager {
    /// Creates a new session manager with an empty cache.
    pub(crate) fn new() -> Self {
        Self {
            container: SessionContainer::new(),
        }
    }

    /// Resolves the session token that should be sent on the next request.
    ///
    /// Resolution order:
    /// 1. If the user explicitly provided a session token via
    ///    [`CosmosOperation::with_session_token`](crate::models::CosmosOperation::with_session_token), use that.
    /// 2. If the request targets a specific partition-key range (`pk_range_id`
    ///    is `Some`), return only that range's cached token. The RNTBD/thin-client
    ///    backend rejects a multi-range composite token on a partition-scoped
    ///    request with `"Session token specified is invalid."`, so a scoped
    ///    request must carry only its own range's token (matching direct-mode
    ///    semantics). Returns `None` when that range has no cached token yet.
    /// 3. Otherwise (no resolved range — e.g. a not-yet-routed or non-partitioned
    ///    request), fall back to the full composite cached by container.
    ///
    /// Returns `None` if no token is available or the operation has no container.
    pub(crate) fn resolve_session_token(
        &self,
        operation: &CosmosOperation,
        user_token: Option<&SessionToken>,
        pk_range_id: Option<&str>,
    ) -> Option<SessionToken> {
        // User-provided token takes precedence
        if let Some(token) = user_token {
            return Some(token.clone());
        }

        // TODO(partition-key-range-parents): When a PKRange cache is available,
        // use it to resolve parent range IDs during splits/merges. Currently
        // only the direct RID is looked up. Java uses PartitionKeyRangeCache to
        // map child ranges back to their parent session tokens.

        let container = operation.container()?;
        match pk_range_id {
            Some(pk_range_id) => self
                .container
                .resolve_session_token_for_range(container, pk_range_id),
            None => self.container.resolve_session_token(container),
        }
    }

    /// Captures the session token from a response into the cache.
    ///
    /// Only captures if:
    /// - The operation is NOT a master/metadata resource operation.
    /// - The operation targets a container (has a [`ContainerReference`]).
    /// - The response headers contain a session token.
    pub(crate) fn capture_session_token(
        &self,
        operation: &CosmosOperation,
        headers: &CosmosResponseHeaders,
    ) {
        // Skip capture for master/metadata resource operations. Session tokens
        // from metadata partition replicas should not be used for data reads.
        // For DocumentCollection, only ReadFeed/Query/SqlQuery target master;
        // CRUD operations (Create/Replace/Delete/Read) should still capture.
        if is_reading_from_master(operation.resource_type(), operation.operation_type()) {
            return;
        }

        let session_token = match &headers.session_token {
            Some(t) => t.as_str(),
            None => return,
        };

        // Require a resolved ContainerReference for capture. Operations without
        // a container (database-level, account-level) are already filtered by
        // the is_reading_from_master check above.
        let container = match operation.container() {
            Some(c) => c,
            None => return,
        };

        self.container.set_session_token(container, session_token);
    }

    /// Merges per-operation DTX session tokens into the shared session cache.
    #[cfg(feature = "preview_dtx")]
    pub(crate) fn merge_distributed_transaction_session_tokens(
        &self,
        response: &crate::models::DistributedTransactionResponse,
        operations: &[crate::models::DistributedTransactionOperation],
        is_session_consistency: bool,
    ) -> crate::error::Result<()> {
        // Only a 2xx-success committed response under Session consistency turns a
        // malformed per-operation token into a hard error. A `304 NotModified`
        // or any non-success envelope must not fail on token bookkeeping.
        let throw_on_malformed = response.is_success_status_code() && is_session_consistency;

        for result in &response.operation_results {
            let Some(operation) = operations.get(result.index) else {
                continue;
            };
            // Skip non-success sub-operations. They may carry stale or malformed
            // tokens that must not be merged or trigger the throw.
            if !result.is_success_status_code() {
                continue;
            }
            let Some(session_token) = result.session_token.as_ref() else {
                continue;
            };
            let token = session_token.as_str();
            if token.trim().is_empty() {
                continue;
            }

            // The coordinator must send a complete `<pkRangeId>:<token>` segment.
            // The SDK validates and merges that value as-is; it does not construct
            // missing partition key range prefixes from side-channel fields.
            let token = token.trim();

            if let Err(error) = self
                .container
                .set_session_token_checked(&operation.target.container, token)
            {
                if throw_on_malformed {
                    return Err(dtx_malformed_session_token_error(format!(
                        "distributed transaction committed but session token for operation {} was rejected: {}",
                        result.index, error
                    )));
                }
            }
        }

        Ok(())
    }

    /// Resolves the session token for one distributed-transaction operation.
    ///
    /// If a resolved partition key range is supplied, this first tries the
    /// exact range token and then parent tokens (for fresh split children). If
    /// that cannot produce a token, it falls back to the compound
    /// collection-level token so the coordinator can select the relevant
    /// segment.
    #[cfg(feature = "preview_dtx")]
    pub(crate) fn resolve_distributed_transaction_session_token(
        &self,
        operation: &crate::models::DistributedTransactionOperation,
        partition_key_range: Option<&PartitionKeyRange>,
    ) -> Option<SessionToken> {
        if let Some(range) = partition_key_range {
            let parents = range.parents.as_deref().unwrap_or(&[]);
            if let Some(token) = self
                .container
                .resolve_session_token_for_partition_key_range(
                    &operation.target.container,
                    &range.id,
                    parents,
                )
            {
                return Some(token);
            }
        }

        self.container
            .resolve_session_token(&operation.target.container)
    }
}

#[cfg(feature = "preview_dtx")]
fn dtx_malformed_session_token_error(message: String) -> crate::error::CosmosError {
    crate::error::CosmosError::builder()
        .with_status(crate::error::CosmosStatus::new(
            azure_core::http::StatusCode::InternalServerError,
        ))
        .with_message(message)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        AccountReference, ContainerProperties, ContainerReference, CosmosOperation,
        CosmosResponseHeaders, DatabaseReference, ItemReference, OperationType, PartitionKey,
        PartitionKeyDefinition, ResourceType, SessionToken, SystemProperties,
    };
    use url::Url;

    fn test_container() -> ContainerReference {
        let account = AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        );
        let pk_def: PartitionKeyDefinition = serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap();
        let props = ContainerProperties {
            id: "coll1".into(),
            partition_key: pk_def,
            system_properties: SystemProperties::default(),
        };
        ContainerReference::new(account, "db1", "db_rid1", "coll1", "coll_rid1", &props)
    }

    fn make_response_headers(
        session_token: Option<&str>,
        owner_id: Option<&str>,
        owner_full_name: Option<&str>,
    ) -> CosmosResponseHeaders {
        CosmosResponseHeaders {
            session_token: session_token.map(|s| SessionToken::new(s.to_owned())),
            owner_id: owner_id.map(|s| s.to_owned()),
            owner_full_name: owner_full_name.map(|s| s.to_owned()),
            ..Default::default()
        }
    }

    #[test]
    fn resolve_returns_none_when_empty() {
        let mgr = SessionManager::new();
        let container = test_container();
        let op = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));
        assert!(mgr.resolve_session_token(&op, None, None).is_none());
    }

    #[test]
    fn user_token_takes_precedence() {
        let mgr = SessionManager::new();
        let container = test_container();
        let op = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));
        let user_token = SessionToken::new("user-provided");
        let result = mgr.resolve_session_token(&op, Some(&user_token), None);
        assert_eq!(result.unwrap().as_str(), "user-provided");
    }

    #[test]
    fn capture_and_resolve() {
        let mgr = SessionManager::new();
        let container = test_container();
        let op = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));

        let headers = make_response_headers(
            Some("0:1#100#1=10"),
            Some("coll_rid1"),
            Some("dbs/db1/colls/coll1"),
        );
        mgr.capture_session_token(&op, &headers);

        let token = mgr.resolve_session_token(&op, None, None).unwrap();
        assert_eq!(token.as_str(), "0:1#100#1=10");
    }

    #[test]
    fn capture_skips_missing_session_token() {
        let mgr = SessionManager::new();
        let container = test_container();
        let op = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));

        let headers = make_response_headers(None, Some("coll_rid1"), None);
        mgr.capture_session_token(&op, &headers);
        assert!(mgr.resolve_session_token(&op, None, None).is_none());
    }

    #[test]
    fn merge_on_capture() {
        let mgr = SessionManager::new();
        let container = test_container();
        let op = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));

        let h1 = make_response_headers(
            Some("0:1#100#1=10"),
            Some("coll_rid1"),
            Some("dbs/db1/colls/coll1"),
        );
        mgr.capture_session_token(&op, &h1);

        let h2 = make_response_headers(
            Some("0:1#200#1=20"),
            Some("coll_rid1"),
            Some("dbs/db1/colls/coll1"),
        );
        mgr.capture_session_token(&op, &h2);

        let token = mgr.resolve_session_token(&op, None, None).unwrap();
        assert!(token.as_str().contains("200"));
    }

    #[test]
    fn resolve_via_name_fallback() {
        let mgr = SessionManager::new();

        // Capture a token for a container with a specific RID
        let account = AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        );
        let pk_def: PartitionKeyDefinition = serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap();
        let props = ContainerProperties {
            id: "coll1".into(),
            partition_key: pk_def.clone(),
            system_properties: SystemProperties::default(),
        };
        let c_capture = ContainerReference::new(
            account.clone(),
            "db1",
            "db_rid1",
            "coll1",
            "original_rid",
            &props,
        );
        let op_capture = CosmosOperation::read_item(ItemReference::from_name(
            &c_capture,
            PartitionKey::from("pk1"),
            "doc1",
        ));

        let headers = make_response_headers(
            Some("0:1#100"),
            Some("original_rid"),
            Some("dbs/db1/colls/coll1"),
        );
        mgr.capture_session_token(&op_capture, &headers);

        // Resolve with a different container reference that has the same
        // name but a different RID — should fall back via name→RID index
        let props2 = ContainerProperties {
            id: "coll1".into(),
            partition_key: pk_def,
            system_properties: SystemProperties::default(),
        };
        let c_resolve =
            ContainerReference::new(account, "db1", "db_rid1", "coll1", "different_rid", &props2);
        let op_resolve = CosmosOperation::read_item(ItemReference::from_name(
            &c_resolve,
            PartitionKey::from("pk1"),
            "doc1",
        ));

        let token = mgr.resolve_session_token(&op_resolve, None, None).unwrap();
        assert_eq!(token.as_str(), "0:1#100");
    }

    #[test]
    fn capture_uses_container_reference_rid() {
        let mgr = SessionManager::new();
        let container = test_container();
        let op = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));

        let headers = make_response_headers(
            Some("0:1#100"),
            Some("some_other_rid"),
            Some("dbs/db1/colls/coll1"),
        );
        mgr.capture_session_token(&op, &headers);

        // Token is stored under the ContainerReference's RID (coll_rid1),
        // not the owner_id header value.
        let token = mgr.resolve_session_token(&op, None, None).unwrap();
        assert_eq!(token.as_str(), "0:1#100");
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_merge_valid_token_updates_session_cache() {
        use crate::models::{
            DistributedTransactionOperation, DistributedTransactionOperationKind,
            DistributedTransactionOperationResult, DistributedTransactionResponse,
            DistributedTransactionResultBody, DistributedTransactionTarget,
        };

        let mgr = SessionManager::new();
        let container = test_container();
        let operation = DistributedTransactionOperation::new(
            DistributedTransactionOperationKind::Create,
            DistributedTransactionTarget::new(container.clone(), PartitionKey::from("pk1"), "doc1"),
        );
        let response = DistributedTransactionResponse {
            status_code: azure_core::http::StatusCode::Ok,
            sub_status_code: None,
            operation_results: vec![DistributedTransactionOperationResult {
                raw_response: Default::default(),
                index: 0,
                status_code: azure_core::http::StatusCode::Created,
                sub_status_code: None,
                etag: None,
                session_token: Some(SessionToken::new("0:1#100#1=10")),
                partition_key_range_id: None,
                request_charge: None,
                resource_body: DistributedTransactionResultBody::None,
            }],
            idempotency_token: uuid::Uuid::nil(),
            headers: Default::default(),
            activity_id: None,
            request_charge: None,
            retry_after_ms: None,
            diagnostics: None,
            is_retriable: false,
            diagnostic_string: None,
            error_message: None,
        };

        mgr.merge_distributed_transaction_session_tokens(&response, &[operation], true)
            .unwrap();

        let read_op = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));
        let token = mgr.resolve_session_token(&read_op, None, None).unwrap();
        assert_eq!(token.as_str(), "0:1#100#1=10");
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_split_session_token_without_pk_range_prefix_errors_under_session_consistency() {
        use crate::models::{
            DistributedTransactionOperation, DistributedTransactionOperationKind,
            DistributedTransactionOperationResult, DistributedTransactionResponse,
            DistributedTransactionResultBody, DistributedTransactionTarget,
        };

        let mgr = SessionManager::new();
        let container = test_container();
        let operation = DistributedTransactionOperation::new(
            DistributedTransactionOperationKind::Create,
            DistributedTransactionTarget::new(container.clone(), PartitionKey::from("pk1"), "doc1"),
        );
        let response = DistributedTransactionResponse {
            status_code: azure_core::http::StatusCode::Ok,
            sub_status_code: None,
            operation_results: vec![DistributedTransactionOperationResult {
                raw_response: Default::default(),
                index: 0,
                status_code: azure_core::http::StatusCode::Created,
                sub_status_code: None,
                etag: None,
                session_token: Some(SessionToken::new("0#3#12=-1")),
                partition_key_range_id: Some("0".to_owned()),
                request_charge: None,
                resource_body: DistributedTransactionResultBody::None,
            }],
            idempotency_token: uuid::Uuid::nil(),
            headers: Default::default(),
            activity_id: None,
            request_charge: None,
            retry_after_ms: None,
            diagnostics: None,
            is_retriable: false,
            diagnostic_string: None,
            error_message: None,
        };

        let error = mgr
            .merge_distributed_transaction_session_tokens(&response, &[operation], true)
            .unwrap_err();
        assert_eq!(
            error.status().status_code(),
            azure_core::http::StatusCode::InternalServerError
        );
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_malformed_success_token_errors_under_session_consistency() {
        use crate::models::{
            DistributedTransactionOperation, DistributedTransactionOperationKind,
            DistributedTransactionOperationResult, DistributedTransactionResponse,
            DistributedTransactionResultBody, DistributedTransactionTarget,
        };

        let mgr = SessionManager::new();
        let container = test_container();
        let operations = vec![DistributedTransactionOperation::new(
            DistributedTransactionOperationKind::Create,
            DistributedTransactionTarget::new(container, PartitionKey::from("pk1"), "doc1"),
        )];
        let response = DistributedTransactionResponse {
            status_code: azure_core::http::StatusCode::Ok,
            sub_status_code: None,
            operation_results: vec![DistributedTransactionOperationResult {
                raw_response: Default::default(),
                index: 0,
                status_code: azure_core::http::StatusCode::Created,
                sub_status_code: None,
                etag: None,
                session_token: Some(SessionToken::new("0:not-a-token")),
                partition_key_range_id: None,
                request_charge: None,
                resource_body: DistributedTransactionResultBody::None,
            }],
            idempotency_token: uuid::Uuid::nil(),
            headers: Default::default(),
            activity_id: None,
            request_charge: None,
            retry_after_ms: None,
            diagnostics: None,
            is_retriable: false,
            diagnostic_string: None,
            error_message: None,
        };

        let error = mgr
            .merge_distributed_transaction_session_tokens(&response, &operations, true)
            .unwrap_err();
        assert_eq!(
            error.status().status_code(),
            azure_core::http::StatusCode::InternalServerError
        );
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_malformed_success_token_is_lenient_without_session_consistency() {
        use crate::models::{
            DistributedTransactionOperation, DistributedTransactionOperationKind,
            DistributedTransactionOperationResult, DistributedTransactionResponse,
            DistributedTransactionResultBody, DistributedTransactionTarget,
        };

        let mgr = SessionManager::new();
        let container = test_container();
        let operations = vec![DistributedTransactionOperation::new(
            DistributedTransactionOperationKind::Create,
            DistributedTransactionTarget::new(container, PartitionKey::from("pk1"), "doc1"),
        )];
        let response = DistributedTransactionResponse {
            status_code: azure_core::http::StatusCode::Ok,
            sub_status_code: None,
            operation_results: vec![DistributedTransactionOperationResult {
                raw_response: Default::default(),
                index: 0,
                status_code: azure_core::http::StatusCode::Created,
                sub_status_code: None,
                etag: None,
                session_token: Some(SessionToken::new("0:not-a-token")),
                partition_key_range_id: None,
                request_charge: None,
                resource_body: DistributedTransactionResultBody::None,
            }],
            idempotency_token: uuid::Uuid::nil(),
            headers: Default::default(),
            activity_id: None,
            request_charge: None,
            retry_after_ms: None,
            diagnostics: None,
            is_retriable: false,
            diagnostic_string: None,
            error_message: None,
        };

        // Outside Session consistency a malformed token is skipped best-effort,
        // never surfaced as an error.
        mgr.merge_distributed_transaction_session_tokens(&response, &operations, false)
            .unwrap();
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_token_without_colon_errors_even_with_pk_range_under_session_consistency() {
        use crate::models::{
            DistributedTransactionOperation, DistributedTransactionOperationKind,
            DistributedTransactionOperationResult, DistributedTransactionResponse,
            DistributedTransactionResultBody, DistributedTransactionTarget,
        };

        let mgr = SessionManager::new();
        let container = test_container();
        let operations = vec![DistributedTransactionOperation::new(
            DistributedTransactionOperationKind::Create,
            DistributedTransactionTarget::new(container, PartitionKey::from("pk1"), "doc1"),
        )];
        // A bare token with no interior colon is not a complete pkRange-prefixed
        // session token segment. The SDK must reject it rather than constructing
        // a token from the side-channel partition_key_range_id field.
        let response = DistributedTransactionResponse {
            status_code: azure_core::http::StatusCode::Ok,
            sub_status_code: None,
            operation_results: vec![DistributedTransactionOperationResult {
                raw_response: Default::default(),
                index: 0,
                status_code: azure_core::http::StatusCode::Created,
                sub_status_code: None,
                etag: None,
                session_token: Some(SessionToken::new("1#100#1=10")),
                partition_key_range_id: Some("0".to_owned()),
                request_charge: None,
                resource_body: DistributedTransactionResultBody::None,
            }],
            idempotency_token: uuid::Uuid::nil(),
            headers: Default::default(),
            activity_id: None,
            request_charge: None,
            retry_after_ms: None,
            diagnostics: None,
            is_retriable: false,
            diagnostic_string: None,
            error_message: None,
        };

        let error = mgr
            .merge_distributed_transaction_session_tokens(&response, &operations, true)
            .unwrap_err();
        assert_eq!(
            error.status().status_code(),
            azure_core::http::StatusCode::InternalServerError
        );

        // Outside Session consistency the same unroutable token is skipped
        // best-effort rather than surfaced as an error.
        mgr.merge_distributed_transaction_session_tokens(&response, &operations, false)
            .unwrap();
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_not_modified_sub_operation_skipped_under_session_consistency() {
        use crate::models::{
            DistributedTransactionOperation, DistributedTransactionOperationKind,
            DistributedTransactionOperationResult, DistributedTransactionResponse,
            DistributedTransactionResultBody, DistributedTransactionTarget,
        };

        let mgr = SessionManager::new();
        let container = test_container();
        let operations = vec![DistributedTransactionOperation::new(
            DistributedTransactionOperationKind::Read,
            DistributedTransactionTarget::new(container.clone(), PartitionKey::from("pk1"), "doc1"),
        )];
        // An all-`304 NotModified` read snapshot whose 304 sub-op carries a
        // malformed token must NOT fail the completed read under Session
        // consistency. The 304 sub-op is non-success, so it is skipped (no merge,
        // no throw) exactly like .NET #5958.
        let response = DistributedTransactionResponse {
            status_code: azure_core::http::StatusCode::NotModified,
            sub_status_code: None,
            operation_results: vec![DistributedTransactionOperationResult {
                raw_response: Default::default(),
                index: 0,
                status_code: azure_core::http::StatusCode::NotModified,
                sub_status_code: None,
                etag: None,
                session_token: Some(SessionToken::new("0:not-a-token")),
                partition_key_range_id: None,
                request_charge: None,
                resource_body: DistributedTransactionResultBody::None,
            }],
            idempotency_token: uuid::Uuid::nil(),
            headers: Default::default(),
            activity_id: None,
            request_charge: None,
            retry_after_ms: None,
            diagnostics: None,
            is_retriable: false,
            diagnostic_string: None,
            error_message: None,
        };

        // No error despite the malformed token on the 304 sub-op.
        mgr.merge_distributed_transaction_session_tokens(&response, &operations, true)
            .unwrap();

        // And nothing was merged for the skipped 304 sub-op.
        let read_op = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));
        assert!(mgr.resolve_session_token(&read_op, None, None).is_none());
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_merges_successful_sub_operation_token_on_non_success_response() {
        use crate::models::{
            DistributedTransactionOperation, DistributedTransactionOperationKind,
            DistributedTransactionOperationResult, DistributedTransactionResponse,
            DistributedTransactionResultBody, DistributedTransactionTarget,
        };

        let mgr = SessionManager::new();
        let container = test_container();
        let operations = vec![DistributedTransactionOperation::new(
            DistributedTransactionOperationKind::Read,
            DistributedTransactionTarget::new(container.clone(), PartitionKey::from("pk1"), "doc1"),
        )];
        // A non-2xx overall status (e.g. a `409` promoted from a MultiStatus
        // partial failure) never throws on token bookkeeping, but a valid token
        // on a 2xx sub-op is still merged best-effort.
        let response = DistributedTransactionResponse {
            status_code: azure_core::http::StatusCode::Conflict,
            sub_status_code: None,
            operation_results: vec![DistributedTransactionOperationResult {
                raw_response: Default::default(),
                index: 0,
                status_code: azure_core::http::StatusCode::Ok,
                sub_status_code: None,
                etag: None,
                session_token: Some(SessionToken::new("0:1#100#1=10")),
                partition_key_range_id: None,
                request_charge: None,
                resource_body: DistributedTransactionResultBody::None,
            }],
            idempotency_token: uuid::Uuid::nil(),
            headers: Default::default(),
            activity_id: None,
            request_charge: None,
            retry_after_ms: None,
            diagnostics: None,
            is_retriable: false,
            diagnostic_string: None,
            error_message: None,
        };

        mgr.merge_distributed_transaction_session_tokens(&response, &operations, true)
            .unwrap();

        let read_op = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));
        let token = mgr.resolve_session_token(&read_op, None, None).unwrap();
        assert_eq!(token.as_str(), "0:1#100#1=10");
    }

    #[cfg(feature = "preview_dtx")]
    #[test]
    fn dtx_resolve_stamps_cached_token_and_preserves_user_token() {
        use crate::models::{
            DistributedTransactionOperation, DistributedTransactionOperationKind,
            DistributedTransactionOperationResult, DistributedTransactionResponse,
            DistributedTransactionResultBody, DistributedTransactionTarget,
        };

        let mgr = SessionManager::new();
        let container = test_container();

        // Prime the cache with a committed token for this container.
        let prime = DistributedTransactionResponse {
            status_code: azure_core::http::StatusCode::Ok,
            sub_status_code: None,
            operation_results: vec![DistributedTransactionOperationResult {
                raw_response: Default::default(),
                index: 0,
                status_code: azure_core::http::StatusCode::Created,
                sub_status_code: None,
                etag: None,
                session_token: Some(SessionToken::new("0:1#100#1=10")),
                partition_key_range_id: None,
                request_charge: None,
                resource_body: DistributedTransactionResultBody::None,
            }],
            idempotency_token: uuid::Uuid::nil(),
            headers: Default::default(),
            activity_id: None,
            request_charge: None,
            retry_after_ms: None,
            diagnostics: None,
            is_retriable: false,
            diagnostic_string: None,
            error_message: None,
        };
        let prime_op = DistributedTransactionOperation::new(
            DistributedTransactionOperationKind::Create,
            DistributedTransactionTarget::new(container.clone(), PartitionKey::from("pk1"), "doc1"),
        );
        mgr.merge_distributed_transaction_session_tokens(&prime, &[prime_op], true)
            .unwrap();

        // op0 has no token (should be stamped from the cache); op1 has a
        // user-supplied token (must be preserved verbatim).
        let mut operations = [
            DistributedTransactionOperation::new(
                DistributedTransactionOperationKind::Read,
                DistributedTransactionTarget::new(
                    container.clone(),
                    PartitionKey::from("pk1"),
                    "doc1",
                ),
            ),
            DistributedTransactionOperation::new(
                DistributedTransactionOperationKind::Read,
                DistributedTransactionTarget::new(
                    container.clone(),
                    PartitionKey::from("pk1"),
                    "doc2",
                ),
            )
            .with_session_token(SessionToken::new("9:9#9")),
        ];

        if let Some(token) = mgr.resolve_distributed_transaction_session_token(&operations[0], None)
        {
            operations[0].session_token = Some(token);
        }
        if operations[1].session_token.is_none() {
            operations[1].session_token =
                mgr.resolve_distributed_transaction_session_token(&operations[1], None);
        }

        assert_eq!(
            operations[0]
                .session_token
                .as_ref()
                .map(|t| t.as_str().to_owned()),
            Some("0:1#100#1=10".to_owned())
        );
        assert_eq!(
            operations[1]
                .session_token
                .as_ref()
                .map(|t| t.as_str().to_owned()),
            Some("9:9#9".to_owned())
        );
    }

    #[test]
    fn capture_succeeds_without_owner_id() {
        let mgr = SessionManager::new();
        let container = test_container();
        let op = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));

        // No owner_id header — capture still works because the RID comes
        // from the ContainerReference, not from response headers.
        let headers = make_response_headers(Some("0:1#100"), None, None);
        mgr.capture_session_token(&op, &headers);
        assert!(mgr.resolve_session_token(&op, None, None).is_some());
    }

    #[test]
    fn multiple_containers_isolated() {
        let mgr = SessionManager::new();

        let account = AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        );
        let pk_def: PartitionKeyDefinition = serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap();
        let props1 = ContainerProperties {
            id: "coll1".into(),
            partition_key: pk_def.clone(),
            system_properties: SystemProperties::default(),
        };
        let c1 = ContainerReference::new(
            account.clone(),
            "db1",
            "db_rid1",
            "coll1",
            "coll_rid1",
            &props1,
        );
        let props2 = ContainerProperties {
            id: "coll2".into(),
            partition_key: pk_def,
            system_properties: SystemProperties::default(),
        };
        let c2 = ContainerReference::new(account, "db1", "db_rid1", "coll2", "coll_rid2", &props2);

        let op1 = CosmosOperation::read_item(ItemReference::from_name(
            &c1,
            PartitionKey::from("pk1"),
            "doc1",
        ));
        let op2 = CosmosOperation::read_item(ItemReference::from_name(
            &c2,
            PartitionKey::from("pk1"),
            "doc1",
        ));

        let h1 = make_response_headers(
            Some("0:1#100"),
            Some("coll_rid1"),
            Some("dbs/db1/colls/coll1"),
        );
        mgr.capture_session_token(&op1, &h1);

        let h2 = make_response_headers(
            Some("0:1#999"),
            Some("coll_rid2"),
            Some("dbs/db1/colls/coll2"),
        );
        mgr.capture_session_token(&op2, &h2);

        let t1 = mgr.resolve_session_token(&op1, None, None).unwrap();
        let t2 = mgr.resolve_session_token(&op2, None, None).unwrap();
        assert!(t1.as_str().contains("100"));
        assert!(t2.as_str().contains("999"));
    }

    #[test]
    fn capture_compound_token() {
        let mgr = SessionManager::new();
        let container = test_container();
        let op = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));

        let headers = make_response_headers(
            Some("0:1#100#1=10,1:1#200#1=20"),
            Some("coll_rid1"),
            Some("dbs/db1/colls/coll1"),
        );
        mgr.capture_session_token(&op, &headers);

        let token = mgr.resolve_session_token(&op, None, None).unwrap();
        assert!(token.as_str().contains("0:") && token.as_str().contains("1:"));
    }

    // ── is_reading_from_master unit tests ──

    #[test]
    fn master_resources_always_reading_from_master() {
        // DatabaseAccount, Database, Offer always read from master
        for rt in [
            ResourceType::DatabaseAccount,
            ResourceType::Database,
            ResourceType::Offer,
        ] {
            for ot in [
                OperationType::Read,
                OperationType::Create,
                OperationType::Delete,
                OperationType::ReadFeed,
                OperationType::Query,
            ] {
                assert!(
                    is_reading_from_master(rt, ot),
                    "{rt:?}/{ot:?} should be master"
                );
            }
        }
    }

    #[test]
    fn partition_key_range_always_reading_from_master() {
        assert!(is_reading_from_master(
            ResourceType::PartitionKeyRange,
            OperationType::ReadFeed,
        ));
        assert!(is_reading_from_master(
            ResourceType::PartitionKeyRange,
            OperationType::Read,
        ));
    }

    #[test]
    fn document_collection_read_feed_query_is_master() {
        for ot in [
            OperationType::ReadFeed,
            OperationType::Query,
            OperationType::SqlQuery,
        ] {
            assert!(
                is_reading_from_master(ResourceType::DocumentCollection, ot),
                "DocumentCollection/{ot:?} should be master"
            );
        }
    }

    #[test]
    fn document_collection_crud_is_not_master() {
        for ot in [
            OperationType::Create,
            OperationType::Read,
            OperationType::Replace,
            OperationType::Delete,
        ] {
            assert!(
                !is_reading_from_master(ResourceType::DocumentCollection, ot),
                "DocumentCollection/{ot:?} should NOT be master"
            );
        }
    }

    #[test]
    fn data_plane_resources_never_master() {
        for rt in [
            ResourceType::Document,
            ResourceType::StoredProcedure,
            ResourceType::Trigger,
            ResourceType::UserDefinedFunction,
        ] {
            assert!(
                !is_reading_from_master(rt, OperationType::Read),
                "{rt:?} should not be master"
            );
        }
    }

    #[test]
    fn capture_skips_container_create_without_reference() {
        // Container Create targets data partitions (NOT master), but the
        // operation has no ContainerReference so capture is skipped.
        let mgr = SessionManager::new();
        let account = AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        );
        let db = DatabaseReference::from_name(account.clone(), "db1");
        let op = CosmosOperation::create_container(db);

        // create_container has resource_type=DocumentCollection, operation_type=Create
        assert!(!is_reading_from_master(
            op.resource_type(),
            op.operation_type()
        ));

        let headers = make_response_headers(
            Some("0:1#100"),
            Some("coll_rid_new"),
            Some("dbs/db1/colls/new_coll"),
        );
        mgr.capture_session_token(&op, &headers);

        // Verify nothing was captured: build a ContainerReference matching the
        // response headers' RID and confirm the cache is still empty.
        let pk_def: PartitionKeyDefinition = serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap();
        let props = ContainerProperties {
            id: "new_coll".into(),
            partition_key: pk_def,
            system_properties: SystemProperties::default(),
        };
        let probe = ContainerReference::new(
            account,
            "db1",
            "db_rid1",
            "new_coll",
            "coll_rid_new",
            &props,
        );
        let probe_op = CosmosOperation::read_item(ItemReference::from_name(
            &probe,
            PartitionKey::from("pk1"),
            "doc1",
        ));
        assert!(mgr.resolve_session_token(&probe_op, None, None).is_none());
    }

    #[test]
    fn capture_skipped_for_container_read_feed() {
        // Container ReadFeed (list containers) targets master, so skip capture.
        let mgr = SessionManager::new();
        let account = AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        );
        let db = DatabaseReference::from_name(account.clone(), "db1");
        let op = CosmosOperation::read_all_containers(db);

        // read_all_containers has resource_type=DocumentCollection, operation_type=ReadFeed
        assert!(is_reading_from_master(
            op.resource_type(),
            op.operation_type()
        ));

        let headers = make_response_headers(
            Some("0:1#100"),
            Some("coll_rid"),
            Some("dbs/db1/colls/coll1"),
        );
        mgr.capture_session_token(&op, &headers);

        // Verify nothing was captured
        let pk_def: PartitionKeyDefinition = serde_json::from_str(r#"{"paths":["/pk"]}"#).unwrap();
        let props = ContainerProperties {
            id: "coll1".into(),
            partition_key: pk_def,
            system_properties: SystemProperties::default(),
        };
        let probe = ContainerReference::new(account, "db1", "db_rid1", "coll1", "coll_rid", &props);
        let probe_op = CosmosOperation::read_item(ItemReference::from_name(
            &probe,
            PartitionKey::from("pk1"),
            "doc1",
        ));
        assert!(mgr.resolve_session_token(&probe_op, None, None).is_none());
    }
}

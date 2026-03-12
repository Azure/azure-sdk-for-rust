// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! High-level session token management for the operation pipeline.
//!
//! [`SessionManager`] wraps [`SessionContainer`] and provides consistency-gated
//! resolve / capture / clear operations that the pipeline calls directly.

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
    ///    [`OperationOptions`](crate::options::OperationOptions), use that.
    /// 2. Otherwise, look up the cached token by the operation's container RID.
    ///
    /// Returns `None` if no token is available.
    pub(crate) fn resolve_session_token(
        &self,
        operation: &CosmosOperation,
        user_token: Option<&SessionToken>,
    ) -> Option<SessionToken> {
        // User-provided token takes precedence
        if let Some(token) = user_token {
            return Some(token.clone());
        }

        // TODO(partition-key-range-parents): When a PKRange cache is available,
        // use it to resolve parent range IDs during splits/merges. Currently
        // only the direct RID is looked up. Java uses PartitionKeyRangeCache to
        // map child ranges back to their parent session tokens.

        // Look up from cache using the container RID, with name fallback
        let container = operation.container()?;
        let rid = container.rid();
        let name_path = format!(
            "dbs/{}/colls/{}",
            container.database_name(),
            container.name()
        );

        self.container.get_or_resolve_session_token(rid, &name_path)
    }

    /// Captures the session token from a response into the cache.
    ///
    /// Only captures if:
    /// - The operation is NOT a master/metadata resource operation.
    /// - The operation targets a container (has a `ContainerReference`).
    /// - The response headers contain a session token.
    /// - The response headers contain an owner ID (collection RID).
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

        let owner_id = match &headers.owner_id {
            Some(id) if !id.is_empty() => id.as_str(),
            _ => return,
        };

        // Build the name path from the operation's container if available,
        // or from the owner_full_name header.
        let name_path: Option<String> = operation
            .container()
            .map(|c| format!("dbs/{}/colls/{}", c.database_name(), c.name()))
            .or_else(|| {
                headers
                    .owner_full_name
                    .as_ref()
                    .filter(|n| !n.is_empty())
                    .cloned()
            });

        self.container
            .set_session_token(owner_id, name_path.as_deref(), session_token);
    }

    /// Clears cached session tokens for an operation's container.
    ///
    /// Called on 404/1002 (`ReadSessionNotAvailable`) during session retry.
    /// Uses name-based clearing so that stale tokens from a potentially
    /// recreated container are removed.
    pub(crate) fn clear_session_token(&self, operation: &CosmosOperation) {
        if let Some(container) = operation.container() {
            let name_path = format!(
                "dbs/{}/colls/{}",
                container.database_name(),
                container.name()
            );
            self.container.clear_by_collection_name(&name_path);
        }
    }
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
        assert!(mgr.resolve_session_token(&op, None).is_none());
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
        let result = mgr.resolve_session_token(&op, Some(&user_token));
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

        let token = mgr.resolve_session_token(&op, None).unwrap();
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
        assert!(mgr.resolve_session_token(&op, None).is_none());
    }

    #[test]
    fn capture_skips_missing_owner_id() {
        let mgr = SessionManager::new();
        let container = test_container();
        let op = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));

        let headers = make_response_headers(Some("0:1#100"), None, None);
        mgr.capture_session_token(&op, &headers);
        assert!(mgr.resolve_session_token(&op, None).is_none());
    }

    #[test]
    fn clear_removes_cached_tokens() {
        let mgr = SessionManager::new();
        let container = test_container();
        let op = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));

        let headers = make_response_headers(
            Some("0:1#100"),
            Some("coll_rid1"),
            Some("dbs/db1/colls/coll1"),
        );
        mgr.capture_session_token(&op, &headers);
        assert!(mgr.resolve_session_token(&op, None).is_some());

        mgr.clear_session_token(&op);
        assert!(mgr.resolve_session_token(&op, None).is_none());
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

        let token = mgr.resolve_session_token(&op, None).unwrap();
        assert!(token.as_str().contains("200"));
    }

    #[test]
    fn resolve_via_name_fallback() {
        let mgr = SessionManager::new();
        let container = test_container();
        let op = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));

        // Capture using a different RID but same name path
        let headers = make_response_headers(
            Some("0:1#100"),
            Some("different_rid"),
            Some("dbs/db1/colls/coll1"),
        );
        mgr.capture_session_token(&op, &headers);

        // Should still resolve via name→RID index
        let token = mgr.resolve_session_token(&op, None).unwrap();
        assert_eq!(token.as_str(), "0:1#100");
    }

    #[test]
    fn capture_maps_rid_from_owner_id() {
        let mgr = SessionManager::new();
        let container = test_container();
        let op = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));

        let headers = make_response_headers(
            Some("0:1#100"),
            Some("some_rid"),
            Some("dbs/db1/colls/coll1"),
        );
        mgr.capture_session_token(&op, &headers);

        // Name path derived from the container reference, RID from owner_id header.
        let resolved_rid = mgr.container.resolve_rid("dbs/db1/colls/coll1");
        assert!(resolved_rid.is_some());
    }

    #[test]
    fn empty_owner_id_skips_capture() {
        let mgr = SessionManager::new();
        let container = test_container();
        let op = CosmosOperation::read_item(ItemReference::from_name(
            &container,
            PartitionKey::from("pk1"),
            "doc1",
        ));

        let headers = make_response_headers(Some("0:1#100"), Some(""), None);
        mgr.capture_session_token(&op, &headers);
        assert!(mgr.resolve_session_token(&op, None).is_none());
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

        let t1 = mgr.resolve_session_token(&op1, None).unwrap();
        let t2 = mgr.resolve_session_token(&op2, None).unwrap();
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

        let token = mgr.resolve_session_token(&op, None).unwrap();
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
    fn capture_allowed_for_container_create() {
        // Container Create targets data partitions, so we should capture.
        let mgr = SessionManager::new();
        let account = AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        );
        let db = DatabaseReference::from_name(account, "db1");
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

        // The token should have been captured (name-based resolve)
        let resolved = mgr.container.resolve_rid("dbs/db1/colls/new_coll");
        assert!(resolved.is_some());
    }

    #[test]
    fn capture_skipped_for_container_read_feed() {
        // Container ReadFeed (list containers) targets master, so skip capture.
        let mgr = SessionManager::new();
        let account = AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        );
        let db = DatabaseReference::from_name(account, "db1");
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

        // Should NOT have captured
        let resolved = mgr.container.resolve_rid("dbs/db1/colls/coll1");
        assert!(resolved.is_none());
    }
}

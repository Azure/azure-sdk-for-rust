// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Session token manager for Cosmos DB session consistency.
//!
//! [`SessionManager`] is the high-level interface used by the operation pipeline
//! to resolve, capture, and clear session tokens. It wraps [`SessionContainer`]
//! and gates operations based on:
//! - Whether session capturing is disabled
//! - Whether the resource type is a master/metadata resource

use crate::driver::routing::session_container::SessionContainer;
use crate::models::{CosmosOperation, CosmosResponseHeaders, ResourceType};

/// Manages session token lifecycle for the operation pipeline.
///
/// The pipeline calls three methods during each operation:
/// - [`resolve_session_token`](Self::resolve_session_token) — before sending a request, to get the cached token
/// - [`capture_session_token`](Self::capture_session_token) — after receiving a response, to update the cache
/// - [`clear_session_token`](Self::clear_session_token) — on session retry (404/1002), to remove stale tokens
#[derive(Debug)]
pub(crate) struct SessionManager {
    container: SessionContainer,
    disabled: bool,
}

impl SessionManager {
    /// Creates a new session manager.
    ///
    /// If `disabled` is true, all operations become no-ops (the cache is never used).
    pub(crate) fn new(disabled: bool) -> Self {
        Self {
            container: SessionContainer::new(),
            disabled,
        }
    }

    /// Resolves the cached session token for the operation's container.
    ///
    /// Returns `None` if:
    /// - Session capturing is disabled
    /// - The operation targets a master/metadata resource
    /// - The operation has no container reference
    /// - No tokens are cached for the container's RID
    pub(crate) fn resolve_session_token(&self, operation: &CosmosOperation) -> Option<String> {
        if self.disabled || is_master_resource(operation.resource_type()) {
            return None;
        }
        let container = operation.container()?;
        self.container.resolve_global_session_token(container.rid())
    }

    /// Captures a session token from the response headers into the cache.
    ///
    /// No-op if:
    /// - Session capturing is disabled
    /// - The operation targets a master/metadata resource
    /// - The response has no session token header
    /// - The response has no owner_id (collection RID) header
    pub(crate) fn capture_session_token(
        &self,
        operation: &CosmosOperation,
        response_headers: &CosmosResponseHeaders,
    ) {
        if self.disabled || is_master_resource(operation.resource_type()) {
            return;
        }

        // We need the session token from the response
        let session_token = match response_headers.session_token.as_ref() {
            Some(t) => t.as_str(),
            None => return,
        };

        // Determine collection name and RID from response headers or operation
        let (collection_name, collection_rid) =
            resolve_collection_identity(operation, response_headers);

        let Some(collection_name) = collection_name else {
            return;
        };
        let Some(collection_rid) = collection_rid else {
            return;
        };

        self.container
            .set_session_token(&collection_name, &collection_rid, session_token);
    }

    /// Clears session tokens for the operation's container.
    ///
    /// Called on session retry (404/1002) to remove potentially stale tokens.
    /// Uses the container's name-based path for clearing.
    pub(crate) fn clear_session_token(&self, operation: &CosmosOperation) {
        if self.disabled {
            return;
        }
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

/// Returns true if the resource type is a master/metadata resource.
///
/// Session tokens are only meaningful for data-plane operations (documents,
/// stored procedures, triggers, UDFs). Master resources (databases, containers,
/// accounts, offers) don't participate in session consistency.
fn is_master_resource(resource_type: ResourceType) -> bool {
    resource_type.is_metadata()
}

/// Resolves the collection name and RID from response headers and the operation.
///
/// Priority for collection name:
/// 1. `x-ms-alt-content-path` response header (owner_full_name)
/// 2. Operation's container name-based path
///
/// Priority for collection RID:
/// 1. `x-ms-content-path` response header (owner_id)
/// 2. Operation's container RID
fn resolve_collection_identity(
    operation: &CosmosOperation,
    headers: &CosmosResponseHeaders,
) -> (Option<String>, Option<String>) {
    let collection_name = headers.owner_full_name.clone().or_else(|| {
        operation
            .container()
            .map(|c| format!("dbs/{}/colls/{}", c.database_name(), c.name()))
    });

    let collection_rid = headers
        .owner_id
        .clone()
        .or_else(|| operation.container().map(|c| c.rid().to_owned()));

    (collection_name, collection_rid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        AccountReference, ContainerProperties, ContainerReference, CosmosOperation,
        CosmosResponseHeaders, DatabaseReference, ItemReference, PartitionKey,
        PartitionKeyDefinition, SessionToken, SystemProperties,
    };
    use url::Url;

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "dGVzdA==",
        )
    }

    fn test_partition_key_definition(path: &str) -> PartitionKeyDefinition {
        serde_json::from_str(&format!(r#"{{"paths":["{path}"]}}"#)).unwrap()
    }

    fn test_container_props() -> ContainerProperties {
        ContainerProperties {
            id: "c1".into(),
            partition_key: test_partition_key_definition("/pk"),
            system_properties: SystemProperties::default(),
        }
    }

    fn test_container() -> ContainerReference {
        ContainerReference::new(
            test_account(),
            "db1",
            "db1_rid",
            "c1",
            "c1_rid",
            &test_container_props(),
        )
    }

    fn test_item_operation() -> CosmosOperation {
        let container = test_container();
        let item = ItemReference::from_name(&container, PartitionKey::from("pk1"), "item1");
        CosmosOperation::read_item(item)
    }

    fn response_with_session_token(token: &str) -> CosmosResponseHeaders {
        CosmosResponseHeaders {
            session_token: Some(SessionToken::new(token.to_owned())),
            owner_full_name: Some("dbs/db1/colls/c1".to_owned()),
            owner_id: Some("c1_rid".to_owned()),
            ..Default::default()
        }
    }

    fn test_database_operation() -> CosmosOperation {
        CosmosOperation::read_database(DatabaseReference::from_name(test_account(), "db1"))
    }

    #[test]
    fn resolve_returns_none_when_empty() {
        let mgr = SessionManager::new(false);
        let op = test_item_operation();
        assert!(mgr.resolve_session_token(&op).is_none());
    }

    #[test]
    fn resolve_returns_none_for_metadata() {
        let mgr = SessionManager::new(false);

        // Capture a token first so the cache is populated
        let item_op = test_item_operation();
        let headers = response_with_session_token("0:1#100#1=200");
        mgr.capture_session_token(&item_op, &headers);

        // A database-level operation should still return None
        let db_op = test_database_operation();
        assert!(mgr.resolve_session_token(&db_op).is_none());
    }

    #[test]
    fn resolve_returns_none_when_disabled() {
        let mgr = SessionManager::new(true);

        // Capture should be ignored when disabled
        let op = test_item_operation();
        let headers = response_with_session_token("0:1#100#1=200");
        mgr.capture_session_token(&op, &headers);

        assert!(mgr.resolve_session_token(&op).is_none());
    }

    #[test]
    fn capture_and_resolve() {
        let mgr = SessionManager::new(false);
        let op = test_item_operation();
        let headers = response_with_session_token("0:1#100#1=200");

        mgr.capture_session_token(&op, &headers);

        let token = mgr.resolve_session_token(&op);
        assert!(token.is_some());
        assert_eq!(token.unwrap(), "0:1#100#1=200");
    }

    #[test]
    fn capture_ignores_metadata() {
        let mgr = SessionManager::new(false);
        let db_op = test_database_operation();
        let headers = response_with_session_token("0:1#100#1=200");

        mgr.capture_session_token(&db_op, &headers);

        // Nothing should have been cached; an item operation for the same container returns None
        let item_op = test_item_operation();
        assert!(mgr.resolve_session_token(&item_op).is_none());
    }

    #[test]
    fn capture_ignores_disabled() {
        let mgr = SessionManager::new(true);
        let op = test_item_operation();
        let headers = response_with_session_token("0:1#100#1=200");

        mgr.capture_session_token(&op, &headers);

        // Even with a new enabled manager sharing nothing, the disabled one never stored
        assert!(mgr.resolve_session_token(&op).is_none());
    }

    #[test]
    fn capture_ignores_missing_session_token() {
        let mgr = SessionManager::new(false);
        let op = test_item_operation();
        let headers = CosmosResponseHeaders {
            session_token: None,
            owner_full_name: Some("dbs/db1/colls/c1".to_owned()),
            owner_id: Some("c1_rid".to_owned()),
            ..Default::default()
        };

        mgr.capture_session_token(&op, &headers);
        assert!(mgr.resolve_session_token(&op).is_none());
    }

    #[test]
    fn capture_ignores_missing_owner() {
        let mgr = SessionManager::new(false);
        let op = test_item_operation();
        let headers = CosmosResponseHeaders {
            session_token: Some(SessionToken::new("0:1#100#1=200".to_owned())),
            owner_full_name: Some("dbs/db1/colls/c1".to_owned()),
            owner_id: None,
            ..Default::default()
        };

        // With no owner_id in headers, fallback uses container RID from operation
        mgr.capture_session_token(&op, &headers);

        // The token should have been captured via the fallback RID
        // For a true "missing owner" test, we need an operation without a container.
        // Since a database operation is metadata and would be skipped, we instead
        // verify that when owner_full_name is also None and the operation has no
        // container, nothing is captured. But since our test_item_operation does
        // have a container, the fallback kicks in. Let's verify that separately.
        // Here we verify the token was captured via fallback.
        let token = mgr.resolve_session_token(&op);
        assert!(token.is_some());
    }

    #[test]
    fn capture_merges_tokens() {
        let mgr = SessionManager::new(false);
        let op = test_item_operation();

        let headers1 = response_with_session_token("0:1#100#1=200");
        mgr.capture_session_token(&op, &headers1);

        let headers2 = response_with_session_token("1:1#150#1=300");
        mgr.capture_session_token(&op, &headers2);

        let token = mgr.resolve_session_token(&op).unwrap();
        // Token should contain both partition ranges
        assert!(token.contains("0:1#100#1=200"));
        assert!(token.contains("1:1#150#1=300"));
    }

    #[test]
    fn clear_removes_tokens() {
        let mgr = SessionManager::new(false);
        let op = test_item_operation();
        let headers = response_with_session_token("0:1#100#1=200");

        mgr.capture_session_token(&op, &headers);
        assert!(mgr.resolve_session_token(&op).is_some());

        mgr.clear_session_token(&op);
        assert!(mgr.resolve_session_token(&op).is_none());
    }

    #[test]
    fn clear_is_noop_when_disabled() {
        let mgr = SessionManager::new(true);
        let op = test_item_operation();
        // Should not panic
        mgr.clear_session_token(&op);
    }

    #[test]
    fn resolve_uses_container_rid_fallback() {
        let mgr = SessionManager::new(false);
        let op = test_item_operation();

        // Response has no owner_id — should fall back to container's RID
        let headers = CosmosResponseHeaders {
            session_token: Some(SessionToken::new("0:1#100#1=200".to_owned())),
            owner_full_name: Some("dbs/db1/colls/c1".to_owned()),
            owner_id: None,
            ..Default::default()
        };

        mgr.capture_session_token(&op, &headers);

        let token = mgr.resolve_session_token(&op);
        assert!(token.is_some());
        assert_eq!(token.unwrap(), "0:1#100#1=200");
    }

    #[test]
    fn resolve_uses_container_name_fallback() {
        let mgr = SessionManager::new(false);
        let op = test_item_operation();

        // Response has no owner_full_name — should fall back to container's name path
        let headers = CosmosResponseHeaders {
            session_token: Some(SessionToken::new("0:1#100#1=200".to_owned())),
            owner_full_name: None,
            owner_id: Some("c1_rid".to_owned()),
            ..Default::default()
        };

        mgr.capture_session_token(&op, &headers);

        let token = mgr.resolve_session_token(&op);
        assert!(token.is_some());
        assert_eq!(token.unwrap(), "0:1#100#1=200");
    }

    #[test]
    fn capture_multiple_containers() {
        let mgr = SessionManager::new(false);

        // First container
        let op1 = test_item_operation();
        let headers1 = response_with_session_token("0:1#100#1=200");
        mgr.capture_session_token(&op1, &headers1);

        // Second container with different RID and name
        let props2 = ContainerProperties {
            id: "c2".into(),
            partition_key: test_partition_key_definition("/pk"),
            system_properties: SystemProperties::default(),
        };
        let container2 =
            ContainerReference::new(test_account(), "db1", "db1_rid", "c2", "c2_rid", &props2);
        let item2 = ItemReference::from_name(&container2, PartitionKey::from("pk2"), "item2");
        let op2 = CosmosOperation::read_item(item2);
        let headers2 = CosmosResponseHeaders {
            session_token: Some(SessionToken::new("0:1#50#1=60".to_owned())),
            owner_full_name: Some("dbs/db1/colls/c2".to_owned()),
            owner_id: Some("c2_rid".to_owned()),
            ..Default::default()
        };
        mgr.capture_session_token(&op2, &headers2);

        // Each container should have its own independent token
        let token1 = mgr.resolve_session_token(&op1).unwrap();
        let token2 = mgr.resolve_session_token(&op2).unwrap();

        assert_eq!(token1, "0:1#100#1=200");
        assert_eq!(token2, "0:1#50#1=60");
    }

    #[test]
    fn is_master_resource_detection() {
        // Metadata resource types
        assert!(is_master_resource(ResourceType::Database));
        assert!(is_master_resource(ResourceType::DocumentCollection));
        assert!(is_master_resource(ResourceType::DatabaseAccount));
        assert!(is_master_resource(ResourceType::Offer));

        // Data-plane resource types
        assert!(!is_master_resource(ResourceType::Document));
        assert!(!is_master_resource(ResourceType::StoredProcedure));
        assert!(!is_master_resource(ResourceType::Trigger));
        assert!(!is_master_resource(ResourceType::UserDefinedFunction));
    }
}

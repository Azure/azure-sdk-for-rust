// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Generic resource reference type for Cosmos DB resources.

use crate::models::{
    AccountReference, ContainerReference, DatabaseReference, ItemReference, ResourceType,
    StoredProcedureReference, TriggerReference, UdfReference,
};
use std::borrow::Cow;

/// A generic reference to any Cosmos DB resource.
///
/// Contains the resource type, optional parent references (account, database, container),
/// and either a name or resource identifier (RID) for the resource itself.
///
/// Construct references from typed references via `From<T>` implementations.
///
/// Operation code can refine the scope using internal helpers such as
/// `with_resource_type` and `into_feed_reference`.
#[derive(Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct CosmosResourceReference {
    /// The type of resource being referenced.
    resource_type: ResourceType,
    /// Reference to the parent account (always required).
    account: AccountReference,
    /// Reference to the parent database (optional, depends on resource type).
    database: Option<DatabaseReference>,
    /// Reference to the parent container (optional, depends on resource type).
    container: Option<ContainerReference>,
    /// The resource name (mutually exclusive with RID for identification).
    name: Option<Cow<'static, str>>,
    /// The resource identifier (RID) (mutually exclusive with name for identification).
    rid: Option<Cow<'static, str>>,
}

impl CosmosResourceReference {
    /// Returns the resource type.
    pub(crate) fn resource_type(&self) -> ResourceType {
        self.resource_type
    }

    /// Returns a reference to the account.
    pub(crate) fn account(&self) -> &AccountReference {
        &self.account
    }

    /// Returns a reference to the database, if applicable.
    #[cfg(test)]
    pub(crate) fn database(&self) -> Option<&DatabaseReference> {
        self.database.as_ref()
    }

    /// Returns a reference to the container, if applicable.
    pub(crate) fn container(&self) -> Option<&ContainerReference> {
        self.container.as_ref()
    }

    /// Returns the resource name, if set.
    #[cfg(test)]
    pub(crate) fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    /// Returns the resource identifier (RID), if set.
    #[cfg(test)]
    pub(crate) fn rid(&self) -> Option<&str> {
        self.rid.as_deref()
    }

    /// Returns `true` if this reference is name-based.
    #[cfg(test)]
    pub(crate) fn is_by_name(&self) -> bool {
        self.name.is_some()
    }

    /// Returns `true` if this reference is RID-based.
    #[cfg(test)]
    pub(crate) fn is_by_rid(&self) -> bool {
        self.rid.is_some()
    }

    /// Sets the resource name.
    pub(crate) fn with_name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.name = Some(name.into());
        self.rid = None;
        self
    }

    /// Sets the resource RID.
    pub(crate) fn with_rid(mut self, rid: impl Into<Cow<'static, str>>) -> Self {
        self.id = Some(ResourceIdentifier::by_rid(rid.into().into_owned()));
        self
    }

    /// Overrides the resource type while preserving account/database/container scope.
    pub(crate) fn with_resource_type(mut self, resource_type: ResourceType) -> Self {
        self.resource_type = resource_type;
        self
    }

    /// Converts this reference to a feed (collection-level) scope.
    pub(crate) fn into_feed_reference(mut self) -> Self {
        self.name = None;
        self.rid = None;

        match self.resource_type {
            ResourceType::Database => {
                self.database = None;
                self.container = None;
            }
            ResourceType::DocumentCollection => {
                self.container = None;
            }
            ResourceType::Offer | ResourceType::DatabaseAccount => {
                self.database = None;
                self.container = None;
            }
            _ => {}
        }

        self
    }

    /// Returns the full resource link for this resource.
    ///
    /// Prefers a name-based path when available, and falls back to RID-based
    /// addressing when required.
    pub(crate) fn resource_link(&self) -> String {
        self.resolved_resource_link()
            .expect("CosmosResourceReference is guaranteed to have a valid path")
    }

    fn resolved_resource_link(&self) -> Option<String> {
        match self.resource_type {
            ResourceType::DatabaseAccount => Some(String::new()),
            ResourceType::Database => {
                if let Some(db) = self.database.as_ref() {
                    db.name_based_path().or_else(|| db.rid_based_path())
                } else {
                    Some("/dbs".to_string())
                }
            }
            ResourceType::DocumentCollection => {
                if let Some(container) = self.container.as_ref() {
                    Some(container.name_based_path())
                } else {
                    let db_path = self
                        .database
                        .as_ref()?
                        .name_based_path()
                        .or_else(|| self.database.as_ref()?.rid_based_path())?;
                    if let Some(name) = self.name.as_ref() {
                        Some(format!("{}/colls/{}", db_path, name))
                    } else {
                        Some(format!("{}/colls", db_path))
                    }
                }
            }
            ResourceType::Document => {
                let container = self.container.as_ref()?;
                if let Some(name) = self.name.as_ref() {
                    Some(format!("{}/docs/{}", container.name_based_path(), name))
                } else {
                    let rid = self.rid.as_ref()?;
                    Some(format!("{}/docs/{}", container.rid_based_path(), rid))
                }
            }
            ResourceType::StoredProcedure
            | ResourceType::Trigger
            | ResourceType::UserDefinedFunction
            | ResourceType::PartitionKeyRange => {
                let container = self.container.as_ref()?;
                let segment = self.resource_type.path_segment();
                if let Some(name) = self.name.as_ref() {
                    Some(format!(
                        "{}/{}/{}",
                        container.name_based_path(),
                        segment,
                        name
                    ))
                } else {
                    let rid = self.rid.as_ref()?;
                    Some(format!(
                        "{}/{}/{}",
                        container.rid_based_path(),
                        segment,
                        rid
                    ))
                }
            }
            ResourceType::Offer => {
                let rid = self.rid.as_ref()?;
                Some(format!("/offers/{}", rid))
            }
        }
    }

    /// Returns the resource link for authorization signing.
    ///
    /// The resource link is an unencoded path used for generating the
    /// authorization signature. Prefers name-based paths over RID-based.
    ///
    /// **Important**: For feed operations (create, list, query) where no specific
    /// item is targeted, this returns the **parent's** path, not the collection path.
    /// This matches the Cosmos DB signature algorithm requirements.
    ///
    /// Examples:
    /// - Creating a database: signing link = "" (empty, account has no parent)
    /// - Creating a container in "mydb": signing link = "dbs/mydb"
    /// - Creating a document: signing link = "dbs/mydb/colls/mycoll"
    /// - Reading a specific database "mydb": signing link = "dbs/mydb"
    /// - Reading a specific document: signing link = "dbs/mydb/colls/mycoll/docs/mydoc"
    ///
    /// This method always returns a valid path because `CosmosResourceReference`
    /// validates that the required identifiers are present at construction time.
    pub(crate) fn link_for_signing(&self) -> String {
        // Check if this is a feed operation (no specific item targeted)
        let is_feed = self.is_feed_reference();

        if is_feed {
            // For feed operations, return parent's path
            self.parent_signing_link()
        } else {
            // For item operations, return the full path
            self.resource_link()
        }
    }

    /// Returns true if this reference targets a feed (collection) rather than a specific item.
    fn is_feed_reference(&self) -> bool {
        match self.resource_type {
            ResourceType::DatabaseAccount => false,
            ResourceType::Database => self.database.is_none(),
            ResourceType::DocumentCollection => self.container.is_none() && self.name.is_none(),
            ResourceType::Document => self.name.is_none() && self.rid.is_none(),
            ResourceType::StoredProcedure
            | ResourceType::Trigger
            | ResourceType::UserDefinedFunction
            | ResourceType::PartitionKeyRange => self.name.is_none() && self.rid.is_none(),
            ResourceType::Offer => self.rid.is_none(),
        }
    }

    /// Returns the parent's path for signing feed operations.
    fn parent_signing_link(&self) -> String {
        match self.resource_type {
            ResourceType::DatabaseAccount => String::new(),
            ResourceType::Database => {
                // Parent is account, which has no path
                String::new()
            }
            ResourceType::DocumentCollection => {
                // Parent is database
                self.database
                    .as_ref()
                    .and_then(|db| db.name_based_path().or_else(|| db.rid_based_path()))
                    .map(|p| p.trim_start_matches('/').to_string())
                    .unwrap_or_default()
            }
            ResourceType::Document
            | ResourceType::StoredProcedure
            | ResourceType::Trigger
            | ResourceType::UserDefinedFunction
            | ResourceType::PartitionKeyRange => {
                // Parent is container — both paths are always available
                self.container
                    .as_ref()
                    .map(|c| c.name_based_path())
                    .map(|p| p.trim_start_matches('/').to_string())
                    .unwrap_or_default()
            }
            ResourceType::Offer => String::new(),
        }
    }

    /// Returns the URL path for this resource.
    ///
    /// This path can be appended to the account endpoint to form the
    /// full request URL. Prefers name-based paths over RID-based.
    ///
    /// This method always returns a valid path because `CosmosResourceReference`
    /// validates that the required identifiers are present at construction time.
    pub(crate) fn request_path(&self) -> String {
        self.resource_link()
    }
}

// =============================================================================
// From implementations for typed references
// =============================================================================

impl From<DatabaseReference> for CosmosResourceReference {
    /// Converts a `DatabaseReference` into a `CosmosResourceReference`.
    ///
    /// The resulting reference has `ResourceType::Database` and preserves
    /// the name-based or RID-based addressing mode.
    fn from(database: DatabaseReference) -> Self {
        let name = database.name().map(|value| Cow::Owned(value.to_owned()));
        let rid = database.rid().map(|value| Cow::Owned(value.to_owned()));
        let account = database.account().clone();

        Self {
            resource_type: ResourceType::Database,
            account,
            database: Some(database),
            container: None,
            name,
            rid,
        }
    }
}

impl From<AccountReference> for CosmosResourceReference {
    /// Converts an `AccountReference` into an account-level `CosmosResourceReference`.
    fn from(account: AccountReference) -> Self {
        Self {
            resource_type: ResourceType::DatabaseAccount,
            account,
            database: None,
            container: None,
            name: None,
            rid: None,
        }
    }
}

impl From<ContainerReference> for CosmosResourceReference {
    /// Converts a `ContainerReference` into a `CosmosResourceReference`.
    ///
    /// The resulting reference has `ResourceType::DocumentCollection` and uses
    /// name-based addressing (both name and RID are always available on
    /// a resolved `ContainerReference`).
    fn from(container: ContainerReference) -> Self {
        let name = Cow::Owned(container.name().to_owned());
        let rid = Cow::Owned(container.rid().to_owned());
        let account = container.account().clone();
        let database = Some(container.database());

        Self {
            resource_type: ResourceType::DocumentCollection,
            account,
            database,
            container: Some(container),
            name: Some(name),
            rid: Some(rid),
        }
    }
}

impl From<ItemReference> for CosmosResourceReference {
    /// Converts an `ItemReference` into a `CosmosResourceReference`.
    ///
    /// The resulting reference has `ResourceType::Document` and preserves
    /// the name-based or RID-based addressing mode.
    fn from(item: ItemReference) -> Self {
        let container = item.container().clone();
        let account = container.account().clone();
        let database = Some(container.database());
        let name = item.name().map(|value| Cow::Owned(value.to_owned()));
        let rid = item.rid().map(|value| Cow::Owned(value.to_owned()));

        Self {
            resource_type: ResourceType::Document,
            account,
            database,
            container: Some(container),
            name,
            rid,
        }
    }
}

impl From<StoredProcedureReference> for CosmosResourceReference {
    /// Converts a `StoredProcedureReference` into a `CosmosResourceReference`.
    ///
    /// The resulting reference has `ResourceType::StoredProcedure` and preserves
    /// the name-based or RID-based addressing mode.
    fn from(stored_procedure: StoredProcedureReference) -> Self {
        let container = stored_procedure.container().clone();
        let account = container.account().clone();
        let database = Some(container.database());
        let name = stored_procedure
            .name()
            .map(|value| Cow::Owned(value.to_owned()));
        let rid = stored_procedure
            .rid()
            .map(|value| Cow::Owned(value.to_owned()));

        Self {
            resource_type: ResourceType::StoredProcedure,
            account,
            database,
            container: Some(container),
            name,
            rid,
        }
    }
}

impl From<TriggerReference> for CosmosResourceReference {
    /// Converts a `TriggerReference` into a `CosmosResourceReference`.
    fn from(trigger: TriggerReference) -> Self {
        let container = trigger.container().clone();
        let account = container.account().clone();
        let database = Some(container.database());
        let name = trigger.name().map(|value| Cow::Owned(value.to_owned()));
        let rid = trigger.rid().map(|value| Cow::Owned(value.to_owned()));

        Self {
            resource_type: ResourceType::Trigger,
            account,
            database,
            container: Some(container),
            name,
            rid,
        }
    }
}

impl From<UdfReference> for CosmosResourceReference {
    /// Converts a `UdfReference` into a `CosmosResourceReference`.
    fn from(udf: UdfReference) -> Self {
        let container = udf.container().clone();
        let account = container.account().clone();
        let database = Some(container.database());
        let name = udf.name().map(|value| Cow::Owned(value.to_owned()));
        let rid = udf.rid().map(|value| Cow::Owned(value.to_owned()));

        Self {
            resource_type: ResourceType::UserDefinedFunction,
            account,
            database,
            container: Some(container),
            name,
            rid,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{PartitionKey, PartitionKeyDefinition};

    use url::Url;

    fn test_account() -> AccountReference {
        AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "test-key",
        )
    }

    fn test_database() -> DatabaseReference {
        DatabaseReference::from_name(test_account(), "testdb")
    }

    fn test_container_props() -> crate::models::ContainerProperties {
        crate::models::ContainerProperties::new(
            "testcontainer",
            PartitionKeyDefinition::new(["/pk"]),
        )
    }

    fn test_container() -> ContainerReference {
        ContainerReference::new(
            test_account(),
            "testdb",
            "dbRid123",
            "testcontainer",
            "collRid456",
            &test_container_props(),
        )
    }

    #[test]
    fn from_account_reference() {
        let ref_: CosmosResourceReference = test_account().into();
        assert_eq!(ref_.resource_type(), ResourceType::DatabaseAccount);
        assert!(ref_.database().is_none());
        assert!(ref_.container().is_none());
        assert_eq!(ref_.resource_link(), String::new());
    }

    #[test]
    fn from_database_reference() {
        let ref_: CosmosResourceReference = test_database().into();
        assert_eq!(ref_.resource_type(), ResourceType::Database);
        assert!(ref_.database().is_some());
        assert!(ref_.container().is_none());
        assert_eq!(ref_.name(), Some("testdb"));
        assert!(ref_.is_by_name());
        assert_eq!(ref_.resource_link(), "/dbs/testdb".to_string());
    }

    #[test]
    fn from_container_reference() {
        let ref_: CosmosResourceReference = test_container().into();
        assert_eq!(ref_.resource_type(), ResourceType::DocumentCollection);
        assert!(ref_.database().is_some());
        assert!(ref_.container().is_some());
        assert_eq!(ref_.name(), Some("testcontainer"));
        assert!(ref_.is_by_name());
        assert_eq!(
            ref_.resource_link(),
            "/dbs/testdb/colls/testcontainer".to_string()
        );
    }

    #[test]
    fn from_item_reference() {
        let item_ref =
            ItemReference::from_name(&test_container(), PartitionKey::from("pk"), "doc1");
        let ref_: CosmosResourceReference = item_ref.into();
        assert_eq!(ref_.resource_type(), ResourceType::Document);
        assert!(ref_.database().is_some());
        assert!(ref_.container().is_some());
        assert_eq!(ref_.name(), Some("doc1"));
        assert!(ref_.is_by_name());
        assert_eq!(
            ref_.resource_link(),
            "/dbs/testdb/colls/testcontainer/docs/doc1".to_string()
        );
    }

    #[test]
    fn from_stored_procedure_reference_name() {
        let sproc_ref =
            crate::models::StoredProcedureReference::from_name(&test_container(), "mysproc");
        let ref_: CosmosResourceReference = sproc_ref.into();
        assert_eq!(ref_.resource_type(), ResourceType::StoredProcedure);
        assert_eq!(ref_.name(), Some("mysproc"));
        assert!(ref_.is_by_name());
        assert_eq!(
            ref_.resource_link(),
            "/dbs/testdb/colls/testcontainer/sprocs/mysproc".to_string()
        );
    }

    #[test]
    fn from_stored_procedure_reference_rid() {
        let sproc_ref =
            crate::models::StoredProcedureReference::from_rid(&test_container(), "sprocRid789");
        let ref_: CosmosResourceReference = sproc_ref.into();

        assert_eq!(ref_.resource_type(), ResourceType::StoredProcedure);
        assert_eq!(ref_.rid(), Some("sprocRid789"));
        assert!(ref_.is_by_rid());
        assert_eq!(
            ref_.resource_link(),
            "/dbs/dbRid123/colls/collRid456/sprocs/sprocRid789".to_string()
        );
    }

    #[test]
    fn from_stored_procedure_reference() {
        let sproc_ref =
            crate::models::StoredProcedureReference::from_name(&test_container(), "mysproc");
        let ref_: CosmosResourceReference = sproc_ref.into();

        assert_eq!(ref_.resource_type(), ResourceType::StoredProcedure);
        assert_eq!(ref_.name(), Some("mysproc"));
        assert!(ref_.is_by_name());
        assert_eq!(
            ref_.resource_link(),
            "/dbs/testdb/colls/testcontainer/sprocs/mysproc".to_string()
        );
    }

    #[test]
    fn from_trigger_reference() {
        let trigger_ref = TriggerReference::from_name(&test_container(), "mytrigger");
        let ref_: CosmosResourceReference = trigger_ref.into();
        assert_eq!(ref_.resource_type(), ResourceType::Trigger);
        assert_eq!(ref_.name(), Some("mytrigger"));
        assert!(ref_.is_by_name());
        assert_eq!(
            ref_.resource_link(),
            "/dbs/testdb/colls/testcontainer/triggers/mytrigger".to_string()
        );
    }

    #[test]
    fn from_udf_reference() {
        let udf_ref = UdfReference::from_name(&test_container(), "myudf");
        let ref_: CosmosResourceReference = udf_ref.into();
        assert_eq!(ref_.resource_type(), ResourceType::UserDefinedFunction);
        assert_eq!(ref_.name(), Some("myudf"));
        assert!(ref_.is_by_name());
        assert_eq!(
            ref_.resource_link(),
            "/dbs/testdb/colls/testcontainer/udfs/myudf".to_string()
        );
    }

    #[test]
    fn feed_scope_helpers() {
        let db_ref: CosmosResourceReference = test_database().into();
        let ref_ = db_ref
            .with_resource_type(ResourceType::DocumentCollection)
            .into_feed_reference();
        assert_eq!(ref_.resource_type(), ResourceType::DocumentCollection);
        assert!(ref_.database().is_some());
        assert!(ref_.container().is_none());
        assert_eq!(ref_.resource_link(), "/dbs/testdb/colls".to_string());
    }

    #[test]
    fn link_for_signing_prefers_name_based() {
        let item_ref =
            ItemReference::from_name(&test_container(), PartitionKey::from("pk"), "doc1");
        let ref_: CosmosResourceReference = item_ref.into();
        assert_eq!(
            ref_.link_for_signing(),
            "/dbs/testdb/colls/testcontainer/docs/doc1"
        );

        let ref_: CosmosResourceReference = test_database().into();
        assert_eq!(ref_.link_for_signing(), "/dbs/testdb");

        let ref_: CosmosResourceReference = test_account().into();
        assert_eq!(ref_.link_for_signing(), "");
    }

    #[test]
    fn link_for_signing_falls_back_to_rid() {
        let item_ref =
            ItemReference::from_rid(&test_container(), PartitionKey::from("pk"), "docRid789");
        let ref_: CosmosResourceReference = item_ref.into();
        assert_eq!(
            ref_.link_for_signing(),
            "/dbs/dbRid123/colls/collRid456/docs/docRid789"
        );
    }

    #[test]
    fn request_path_matches_link_for_signing() {
        let item_ref =
            ItemReference::from_name(&test_container(), PartitionKey::from("pk"), "doc1");
        let ref_: CosmosResourceReference = item_ref.into();
        assert_eq!(ref_.request_path(), ref_.link_for_signing());

        let ref_: CosmosResourceReference = test_database().into();
        assert_eq!(ref_.request_path(), ref_.link_for_signing());
    }
}

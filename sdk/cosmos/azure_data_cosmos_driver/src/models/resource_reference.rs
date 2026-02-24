// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Resource reference types for Cosmos DB resources.
//!
//! These types provide compile-time safe references to Cosmos DB resources.
//! Each reference enforces either all-names or all-RIDs addressing through
//! internal enums, preventing mixed addressing modes.

use crate::models::{
    resource_id::{ResourceIdentifierType, ResourceName, ResourceRid},
    AccountReference, ImmutableContainerProperties, PartitionKey,
};

use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

// =============================================================================
// DatabaseReference
// =============================================================================

/// A reference to a Cosmos DB database.
///
/// Contains either the name or resource identifier (RID) of the database,
/// along with a reference to its parent account. The addressing mode (name vs RID)
/// is enforced at compile time through internal enums.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct DatabaseReference {
    /// Reference to the parent account.
    account: AccountReference,
    /// The database identifier (by name or by RID).
    id: ResourceIdentifierType,
}

impl DatabaseReference {
    /// Creates a new database reference by name.
    pub fn from_name(account: AccountReference, name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            account,
            id: ResourceIdentifierType::ByName(ResourceName::new(name)),
        }
    }

    /// Creates a new database reference by RID.
    pub fn from_rid(account: AccountReference, rid: impl Into<Cow<'static, str>>) -> Self {
        Self {
            account,
            id: ResourceIdentifier::ByRid(ResourceId::new(rid)),
        }
    }

    /// Returns a reference to the parent account.
    pub fn account(&self) -> &AccountReference {
        &self.account
    }

    /// Returns the database name, if this is a name-based reference.
    pub fn name(&self) -> Option<&str> {
        self.id.name()
    }

    /// Returns the database RID, if this is a RID-based reference.
    pub fn rid(&self) -> Option<&str> {
        self.id.rid()
    }

    /// Returns `true` if this is a name-based reference.
    pub fn is_by_name(&self) -> bool {
        matches!(self.id, ResourceIdentifierType::ByName(_))
    }

    /// Returns `true` if this is a RID-based reference.
    pub fn is_by_rid(&self) -> bool {
        matches!(self.id, ResourceIdentifierType::ByRid(_))
    }

    /// Returns the name-based relative path: `/dbs/{name}`
    ///
    /// Returns `None` if this is a RID-based reference.
    pub fn name_based_path(&self) -> Option<String> {
        self.id.name().map(|n| format!("/dbs/{}", n))
    }

    /// Returns the RID-based relative path: `/dbs/{rid}`
    ///
    /// Returns `None` if this is a name-based reference.
    pub fn rid_based_path(&self) -> Option<String> {
        self.id.rid().map(|r| format!("/dbs/{}", r))
    }
}

// =============================================================================
// ContainerReference
// =============================================================================

/// A resolved reference to a Cosmos DB container.
///
/// Always carries both the name-based and RID-based identifiers for the container
/// and its parent database, along with immutable container properties (partition key
/// definition and unique key policy). This guarantees that both addressing modes
/// are available without additional I/O.
///
/// Instances are created via async factory methods that resolve the container
/// metadata from the Cosmos DB service or cache.
///
/// ## Equality and Hashing
///
/// Two `ContainerReference` values are considered equal if they refer to the same
/// account, container RID, and container name. This detects both delete + recreate
/// (same name, different RID) and rename scenarios (same RID, different name).
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ContainerReference {
    /// Reference to the parent account.
    account: AccountReference,
    /// The database user-provided name.
    db_name: ResourceName,
    /// The database internal RID.
    db_rid: ResourceRid,
    /// The container user-provided name.
    container_name: ResourceName,
    /// The container internal RID.
    container_rid: ResourceRid,
    /// Immutable container properties (partition key, unique key policy).
    immutable_properties: Arc<ImmutableContainerProperties>,
}

impl PartialEq for ContainerReference {
    fn eq(&self, other: &Self) -> bool {
        self.account == other.account
            && self.container_rid == other.container_rid
            && self.container_name == other.container_name
    }
}

impl Eq for ContainerReference {}

impl Hash for ContainerReference {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.account.hash(state);
        self.container_rid.hash(state);
        self.container_name.hash(state);
    }
}

impl ContainerReference {
    /// Creates a fully resolved container reference.
    ///
    /// All fields are required — the caller must have already resolved both
    /// name-based and RID-based identifiers (typically by reading the container
    /// from the Cosmos DB service).
    ///
    /// The immutable properties (partition key definition, unique key policy)
    /// are extracted from `container_properties` and stored internally.
    ///
    /// Not exposed publicly — use [`CosmosDriver::resolve_container()`](crate::driver::CosmosDriver::resolve_container)
    /// to obtain a resolved container reference.
    pub(crate) fn new(
        account: AccountReference,
        db_name: impl Into<ResourceName>,
        db_rid: impl Into<ResourceRid>,
        container_name: impl Into<ResourceName>,
        container_rid: impl Into<ResourceRid>,
        container_properties: &crate::models::ContainerProperties,
    ) -> Self {
        Self {
            account,
            db_name: db_name.into(),
            db_rid: db_rid.into(),
            container_name: container_name.into(),
            container_rid: container_rid.into(),
            immutable_properties: Arc::new(
                ImmutableContainerProperties::from_container_properties(container_properties),
            ),
        }
    }

    /// Returns a reference to the parent account.
    pub fn account(&self) -> &AccountReference {
        &self.account
    }

    /// Returns the container name.
    pub fn name(&self) -> &str {
        self.container_name.as_str()
    }

    /// Returns the container RID.
    pub fn rid(&self) -> &str {
        self.container_rid.as_str()
    }

    /// Returns the database name.
    pub fn database_name(&self) -> &str {
        self.db_name.as_str()
    }

    /// Returns the database RID.
    pub fn database_rid(&self) -> &str {
        self.db_rid.as_str()
    }

    /// Returns the partition key definition for this container.
    pub fn partition_key_definition(&self) -> &crate::models::PartitionKeyDefinition {
        self.immutable_properties.partition_key()
    }

    /// Returns a `DatabaseReference` for the parent database (name-based).
    pub(crate) fn database(&self) -> DatabaseReference {
        DatabaseReference {
            account: self.account.clone(),
            id: ResourceIdentifierType::ByName(self.db_name.clone()),
        }
    }

    /// Returns the name-based relative path: `/dbs/{db_name}/colls/{container_name}`
    pub fn name_based_path(&self) -> String {
        format!("/dbs/{}/colls/{}", self.db_name, self.container_name)
    }

    /// Returns the RID-based relative path: `/dbs/{db_rid}/colls/{container_rid}`
    pub fn rid_based_path(&self) -> String {
        format!("/dbs/{}/colls/{}", self.db_rid, self.container_rid)
    }
}

// =============================================================================
// ItemReference
// =============================================================================

/// A reference to a Cosmos DB item (document).
///
/// Contains the container reference, partition key, and item identifier (name or RID).
/// The partition key is required because all item operations in Cosmos DB require it.
///
/// The resource link is pre-computed for efficiency.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct ItemReference {
    /// Reference to the parent container.
    container: ContainerReference,
    /// The partition key for the item.
    partition_key: PartitionKey,
    /// The item identifier (name or RID).
    item_identifier: ResourceIdentifierType,
    /// Pre-computed resource link.
    resource_link: String,
}

impl ItemReference {
    /// Creates a new item reference by name.
    ///
    /// # Arguments
    ///
    /// * `container` - Reference to the parent container.
    /// * `partition_key` - The partition key for the item.
    /// * `item_name` - The document ID (name) of the item.
    pub fn from_name(
        container: &ContainerReference,
        partition_key: PartitionKey,
        item_name: impl Into<Cow<'static, str>>,
    ) -> Self {
        let name = ResourceName::new(item_name);
        let resource_link = format!("{}/docs/{}", container.name_based_path(), name);
        Self {
            container: container.clone(),
            partition_key,
            item_identifier: ResourceIdentifierType::by_name(name),
            resource_link,
        }
    }

    /// Creates a new item reference by RID.
    ///
    /// # Arguments
    ///
    /// * `container` - Reference to the parent container.
    /// * `partition_key` - The partition key for the item.
    /// * `item_rid` - The internal RID of the item.
    pub fn from_rid(
        container: &ContainerReference,
        partition_key: PartitionKey,
        item_rid: impl Into<Cow<'static, str>>,
    ) -> Self {
        let rid = ResourceRid::new(item_rid);
        let resource_link = format!("{}/docs/{}", container.rid_based_path(), rid);
        Self {
            container: container.clone(),
            partition_key,
            item_identifier: ResourceIdentifierType::by_rid(rid),
            resource_link,
        }
    }

    /// Returns a reference to the parent container.
    pub fn container(&self) -> &ContainerReference {
        &self.container
    }

    /// Returns a reference to the parent account.
    pub fn account(&self) -> &AccountReference {
        self.container.account()
    }

    /// Returns a reference to the partition key.
    pub fn partition_key(&self) -> &PartitionKey {
        &self.partition_key
    }

    /// Returns the item name (document ID), if this is a name-based reference.
    pub fn name(&self) -> Option<&str> {
        self.item_identifier.name()
    }

    /// Returns the item RID, if this is a RID-based reference.
    pub fn rid(&self) -> Option<&str> {
        self.item_identifier.rid()
    }

    /// Returns `true` if this is a name-based reference.
    pub fn is_by_name(&self) -> bool {
        self.item_identifier.is_by_name()
    }

    /// Returns `true` if this is a RID-based reference.
    pub fn is_by_rid(&self) -> bool {
        self.item_identifier.is_by_rid()
    }

    /// Returns the pre-computed resource link for this item.
    ///
    /// For name-based references: `/dbs/{db}/colls/{coll}/docs/{item}`
    /// For RID-based references: `/dbs/{db_rid}/colls/{coll_rid}/docs/{item_rid}`
    pub fn resource_link(&self) -> &str {
        &self.resource_link
    }
}

// =============================================================================
// StoredProcedureReference
// =============================================================================

/// A reference to a Cosmos DB stored procedure.
///
/// Contains the parent container and either the name or RID of the stored procedure.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct StoredProcedureReference {
    /// Reference to the parent container.
    container: ContainerReference,
    /// The stored procedure identifier.
    stored_procedure_identifier: ResourceIdentifierType,
    /// Pre-computed resource link.
    resource_link: String,
}

impl StoredProcedureReference {
    /// Creates a new stored procedure reference by name.
    pub fn from_name(
        container: &ContainerReference,
        stored_procedure_name: impl Into<Cow<'static, str>>,
    ) -> Self {
        let stored_procedure_name = ResourceName::new(stored_procedure_name);
        let resource_link = format!(
            "{}/sprocs/{}",
            container.name_based_path(),
            stored_procedure_name
        );
        Self {
            container: container.clone(),
            stored_procedure_identifier: ResourceIdentifierType::by_name(stored_procedure_name),
            resource_link,
        }
    }

    /// Creates a new stored procedure reference by RID.
    pub fn from_rid(
        container: &ContainerReference,
        stored_procedure_rid: impl Into<Cow<'static, str>>,
    ) -> Self {
        let stored_procedure_rid = ResourceRid::new(stored_procedure_rid);
        let resource_link = format!(
            "{}/sprocs/{}",
            container.rid_based_path(),
            stored_procedure_rid
        );
        Self {
            container: container.clone(),
            stored_procedure_identifier: ResourceIdentifierType::by_rid(stored_procedure_rid),
            resource_link,
        }
    }

    /// Returns a reference to the parent container.
    pub fn container(&self) -> &ContainerReference {
        &self.container
    }

    /// Returns a reference to the parent account.
    pub fn account(&self) -> &AccountReference {
        self.container.account()
    }

    /// Returns the stored procedure name, if this is a name-based reference.
    pub fn name(&self) -> Option<&str> {
        self.stored_procedure_identifier.name()
    }

    /// Returns the stored procedure RID, if this is a RID-based reference.
    pub fn rid(&self) -> Option<&str> {
        self.stored_procedure_identifier.rid()
    }

    /// Returns `true` if this is a name-based reference.
    pub fn is_by_name(&self) -> bool {
        self.stored_procedure_identifier.is_by_name()
    }

    /// Returns `true` if this is a RID-based reference.
    pub fn is_by_rid(&self) -> bool {
        self.stored_procedure_identifier.is_by_rid()
    }

    /// Returns the pre-computed resource link for this stored procedure.
    ///
    /// For name-based references: `/dbs/{db}/colls/{coll}/sprocs/{name}`
    /// For RID-based references: `/dbs/{db_rid}/colls/{coll_rid}/sprocs/{rid}`
    pub fn resource_link(&self) -> &str {
        &self.resource_link
    }
}

// =============================================================================
// TriggerReference
// =============================================================================

/// A reference to a Cosmos DB trigger resource.
///
/// Contains the parent container and either the name or RID of the trigger.
///
/// Note: This is different from `TriggerInvocation` which specifies which trigger
/// to invoke during an operation. This type is for referencing trigger definitions.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct TriggerReference {
    /// Reference to the parent container.
    container: ContainerReference,
    /// The trigger identifier.
    trigger_identifier: ResourceIdentifierType,
    /// Pre-computed resource link.
    resource_link: String,
}

impl TriggerReference {
    /// Creates a new trigger reference by name.
    pub fn from_name(
        container: &ContainerReference,
        trigger_name: impl Into<Cow<'static, str>>,
    ) -> Self {
        let trigger_name = ResourceName::new(trigger_name);
        let resource_link = format!("{}/triggers/{}", container.name_based_path(), trigger_name);
        Self {
            container: container.clone(),
            trigger_identifier: ResourceIdentifierType::by_name(trigger_name),
            resource_link,
        }
    }

    /// Creates a new trigger reference by RID.
    pub fn from_rid(
        container: &ContainerReference,
        trigger_rid: impl Into<Cow<'static, str>>,
    ) -> Self {
        let trigger_rid = ResourceRid::new(trigger_rid);
        let resource_link = format!("{}/triggers/{}", container.rid_based_path(), trigger_rid);
        Self {
            container: container.clone(),
            trigger_identifier: ResourceIdentifierType::by_rid(trigger_rid),
            resource_link,
        }
    }

    /// Returns a reference to the parent container.
    pub fn container(&self) -> &ContainerReference {
        &self.container
    }

    /// Returns a reference to the parent account.
    pub fn account(&self) -> &AccountReference {
        self.container.account()
    }

    /// Returns the trigger name, if this is a name-based reference.
    pub fn name(&self) -> Option<&str> {
        self.trigger_identifier.name()
    }

    /// Returns the trigger RID, if this is a RID-based reference.
    pub fn rid(&self) -> Option<&str> {
        self.trigger_identifier.rid()
    }

    /// Returns `true` if this is a name-based reference.
    pub fn is_by_name(&self) -> bool {
        self.trigger_identifier.is_by_name()
    }

    /// Returns `true` if this is a RID-based reference.
    pub fn is_by_rid(&self) -> bool {
        self.trigger_identifier.is_by_rid()
    }

    /// Returns the pre-computed resource link for this trigger.
    pub fn resource_link(&self) -> &str {
        &self.resource_link
    }
}

// =============================================================================
// UdfReference (User-Defined Function)
// =============================================================================

/// A reference to a Cosmos DB user-defined function (UDF).
///
/// Contains the parent container and either the name or RID of the UDF.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub struct UdfReference {
    /// Reference to the parent container.
    container: ContainerReference,
    /// The UDF identifier.
    udf_identifier: ResourceIdentifierType,
    /// Pre-computed resource link.
    resource_link: String,
}

impl UdfReference {
    /// Creates a new UDF reference by name.
    pub fn from_name(
        container: &ContainerReference,
        udf_name: impl Into<Cow<'static, str>>,
    ) -> Self {
        let udf_name = ResourceName::new(udf_name);
        let resource_link = format!("{}/udfs/{}", container.name_based_path(), udf_name);
        Self {
            container: container.clone(),
            udf_identifier: ResourceIdentifierType::by_name(udf_name),
            resource_link,
        }
    }

    /// Creates a new UDF reference by RID.
    pub fn from_rid(container: &ContainerReference, udf_rid: impl Into<Cow<'static, str>>) -> Self {
        let udf_rid = ResourceRid::new(udf_rid);
        let resource_link = format!("{}/udfs/{}", container.rid_based_path(), udf_rid);
        Self {
            container: container.clone(),
            udf_identifier: ResourceIdentifierType::by_rid(udf_rid),
            resource_link,
        }
    }

    /// Returns a reference to the parent container.
    pub fn container(&self) -> &ContainerReference {
        &self.container
    }

    /// Returns a reference to the parent account.
    pub fn account(&self) -> &AccountReference {
        self.container.account()
    }

    /// Returns the UDF name, if this is a name-based reference.
    pub fn name(&self) -> Option<&str> {
        self.udf_identifier.name()
    }

    /// Returns the UDF RID, if this is a RID-based reference.
    pub fn rid(&self) -> Option<&str> {
        self.udf_identifier.rid()
    }

    /// Returns `true` if this is a name-based reference.
    pub fn is_by_name(&self) -> bool {
        self.udf_identifier.is_by_name()
    }

    /// Returns `true` if this is a RID-based reference.
    pub fn is_by_rid(&self) -> bool {
        self.udf_identifier.is_by_rid()
    }

    /// Returns the pre-computed resource link for this UDF.
    pub fn resource_link(&self) -> &str {
        &self.resource_link
    }
}

// =============================================================================
// PartitionKeyRangeReference
// =============================================================================

/// A reference to a Cosmos DB partition key range.
///
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PartitionKeyRangeReference {
    /// Reference to the parent container.
    container: ContainerReference,
    /// The partition key range identifier.
    range_id: ResourceName,
}

impl PartitionKeyRangeReference {
    /// Creates a new partition key range reference from a range ID.
    pub fn from_range_id(
        container: &ContainerReference,
        range_id: impl Into<Cow<'static, str>>,
    ) -> Self {
        let range_id = ResourceName::new(range_id);
        Self {
            container: container.clone(),
            range_id,
        }
    }

    /// Returns a reference to the parent container.
    pub fn container(&self) -> &ContainerReference {
        &self.container
    }

    /// Returns a reference to the parent account.
    pub fn account(&self) -> &AccountReference {
        self.container.account()
    }

    /// Returns the partition key range ID.
    pub fn range_id(&self) -> &str {
        self.range_id.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{ContainerProperties, PartitionKeyDefinition, SystemProperties};
    use url::Url;

    fn test_partition_key_definition(path: &str) -> PartitionKeyDefinition {
        serde_json::from_str(&format!(r#"{{"paths":["{path}"]}}"#)).unwrap()
    }

    fn make_container_reference() -> ContainerReference {
        let account = AccountReference::with_master_key(
            Url::parse("https://example.documents.azure.com:443/").unwrap(),
            "test-key",
        );
        let partition_key = test_partition_key_definition("/tenantId");
        let container_properties = ContainerProperties {
            id: "my-container".into(),
            partition_key,
            system_properties: SystemProperties::default(),
        };

        ContainerReference::new(
            account,
            "my-db",
            "db-rid",
            "my-container",
            "container-rid",
            &container_properties,
        )
    }

    #[test]
    fn container_partition_key_definition_is_available() {
        let container = make_container_reference();
        let partition_key_definition = container.partition_key_definition();

        assert_eq!(partition_key_definition.paths().len(), 1);
        assert_eq!(partition_key_definition.paths()[0].as_ref(), "/tenantId");
    }
}

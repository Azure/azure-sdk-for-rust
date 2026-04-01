// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Throughput control configuration and registry.
//!
//! Throughput control groups allow limiting the request rate for specific containers.
//! Groups are registered at startup in [`CosmosDriverRuntimeBuilder`](crate::options::CosmosDriverRuntimeBuilder)
//! and are immutable after runtime creation (except for mutable target values).
//!
//! Each group is uniquely identified by the combination of container reference and group name.
//! At most one group per container can be marked as `is_default = true`.

use crate::{
    models::{ContainerReference, ThroughputControlGroupName},
    options::PriorityLevel,
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

/// Mutable runtime values for a server-side throughput bucket control group.
#[derive(Clone, Debug)]
pub(crate) struct ServerSideThroughputBucketMutableValues {
    /// Throughput bucket assignment.
    throughput_bucket: u32,
}

impl ServerSideThroughputBucketMutableValues {
    /// Creates new mutable values.
    pub(crate) fn new(throughput_bucket: u32) -> Self {
        Self { throughput_bucket }
    }

    /// Returns the throughput bucket.
    pub(crate) fn throughput_bucket(&self) -> u32 {
        self.throughput_bucket
    }
}

/// Mutable runtime values for a server-side priority-based throttling control group.
#[derive(Clone, Debug)]
pub(crate) struct ServerSidePriorityMutableValues {
    /// Priority level for throttling.
    priority_level: PriorityLevel,
}

impl ServerSidePriorityMutableValues {
    /// Creates new mutable values.
    pub(crate) fn new(priority_level: PriorityLevel) -> Self {
        Self { priority_level }
    }

    /// Returns the priority level.
    pub(crate) fn priority_level(&self) -> PriorityLevel {
        self.priority_level
    }
}

/// Configuration for a throughput control group.
///
/// Registered at the runtime level and associated with a container.
/// Throughput control is enforced server-side.
///
/// # Immutability
///
/// Once registered, the group's type and `is_default` flag are immutable.
/// Only the target values (priority level, bucket) can be modified at runtime.
#[derive(Clone, Debug)]
#[non_exhaustive]
#[allow(private_interfaces)] // mutable fields use crate-internal types, accessed via public methods
pub enum ThroughputControlGroupOptions {
    /// Server-side enforced throughput control using throughput buckets.
    ///
    /// The Cosmos DB service enforces the throughput limits.
    /// See <https://learn.microsoft.com/azure/cosmos-db/nosql/throughput-buckets>
    ServerSideThroughputBucket {
        /// Unique name identifying this control group.
        name: ThroughputControlGroupName,
        /// Reference to the container this group applies to.
        container: ContainerReference,
        /// Whether this group is used by default for requests without explicit assignment.
        is_default: bool,
        /// Mutable runtime values (wrapped in RwLock for thread-safe updates).
        mutable: Arc<RwLock<ServerSideThroughputBucketMutableValues>>,
    },

    /// Server-side enforced throughput control using priority-based throttling.
    ///
    /// The Cosmos DB service enforces the throughput limits.
    /// See <https://learn.microsoft.com/azure/cosmos-db/priority-based-execution>
    ServerSidePriorityBasedThrottling {
        /// Unique name identifying this control group.
        name: ThroughputControlGroupName,
        /// Reference to the container this group applies to.
        container: ContainerReference,
        /// Whether this group is used by default for requests without explicit assignment.
        is_default: bool,
        /// Mutable runtime values (wrapped in RwLock for thread-safe updates).
        mutable: Arc<RwLock<ServerSidePriorityMutableValues>>,
    },
}

impl ThroughputControlGroupOptions {
    /// Creates a new server-side throughput bucket control group.
    pub fn server_side_throughput_bucket(
        name: impl Into<ThroughputControlGroupName>,
        container: ContainerReference,
        throughput_bucket: u32,
        is_default: bool,
    ) -> Self {
        Self::ServerSideThroughputBucket {
            name: name.into(),
            container,
            is_default,
            mutable: Arc::new(RwLock::new(ServerSideThroughputBucketMutableValues::new(
                throughput_bucket,
            ))),
        }
    }

    /// Creates a new server-side priority-based throttling control group.
    pub fn server_side_priority_based_throttling(
        name: impl Into<ThroughputControlGroupName>,
        container: ContainerReference,
        priority_level: PriorityLevel,
        is_default: bool,
    ) -> Self {
        Self::ServerSidePriorityBasedThrottling {
            name: name.into(),
            container,
            is_default,
            mutable: Arc::new(RwLock::new(ServerSidePriorityMutableValues::new(
                priority_level,
            ))),
        }
    }

    /// Returns the name of the throughput control group.
    pub fn name(&self) -> &ThroughputControlGroupName {
        match self {
            Self::ServerSideThroughputBucket { name, .. } => name,
            Self::ServerSidePriorityBasedThrottling { name, .. } => name,
        }
    }

    /// Returns the container this group applies to.
    pub fn container(&self) -> &ContainerReference {
        match self {
            Self::ServerSideThroughputBucket { container, .. } => container,
            Self::ServerSidePriorityBasedThrottling { container, .. } => container,
        }
    }

    /// Returns whether this group is the default for its container.
    pub fn is_default(&self) -> bool {
        match self {
            Self::ServerSideThroughputBucket { is_default, .. } => *is_default,
            Self::ServerSidePriorityBasedThrottling { is_default, .. } => *is_default,
        }
    }

    /// Returns the registry key for this group.
    pub fn key(&self) -> ThroughputControlGroupKey {
        ThroughputControlGroupKey {
            container: self.container().clone(),
            name: self.name().clone(),
        }
    }

    // ========== Throughput bucket accessor ==========

    /// Returns the current throughput bucket (server-side bucket groups only).
    ///
    /// Returns `None` for other group types.
    pub fn throughput_bucket(&self) -> Option<u32> {
        match self {
            Self::ServerSideThroughputBucket { mutable, .. } => {
                Some(mutable.read().unwrap().throughput_bucket())
            }
            Self::ServerSidePriorityBasedThrottling { .. } => None,
        }
    }

    /// Sets the throughput bucket (server-side bucket groups only).
    ///
    /// Does nothing for other group types.
    pub fn set_throughput_bucket(&self, bucket: u32) {
        match self {
            Self::ServerSideThroughputBucket { mutable, .. } => {
                mutable.write().unwrap().throughput_bucket = bucket;
            }
            Self::ServerSidePriorityBasedThrottling { .. } => {
                tracing::debug!(
                    group_name = self.name().as_str(),
                    "set_throughput_bucket called on non-bucket group variant, ignored"
                );
            }
        }
    }

    // ========== Priority level accessors ==========

    /// Returns the current priority level.
    ///
    /// Returns `None` for server-side bucket groups.
    pub fn priority_level(&self) -> Option<PriorityLevel> {
        match self {
            Self::ServerSidePriorityBasedThrottling { mutable, .. } => {
                Some(mutable.read().unwrap().priority_level())
            }
            Self::ServerSideThroughputBucket { .. } => None,
        }
    }

    /// Sets the priority level (server-side priority groups only).
    ///
    /// Does nothing for server-side bucket groups.
    pub fn set_priority_level(&self, level: PriorityLevel) {
        match self {
            Self::ServerSidePriorityBasedThrottling { mutable, .. } => {
                mutable.write().unwrap().priority_level = level;
            }
            Self::ServerSideThroughputBucket { .. } => {
                tracing::debug!(
                    group_name = self.name().as_str(),
                    "set_priority_level called on non-priority group variant, ignored"
                );
            }
        }
    }
}

/// Composite key for identifying a throughput control group.
///
/// Groups are uniquely identified by the combination of container and name.
/// The same group name can be registered for different containers.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ThroughputControlGroupKey {
    /// The container this group applies to.
    pub container: ContainerReference,
    /// The group name.
    pub name: ThroughputControlGroupName,
}

impl ThroughputControlGroupKey {
    /// Creates a new key.
    pub fn new(container: ContainerReference, name: impl Into<ThroughputControlGroupName>) -> Self {
        Self {
            container,
            name: name.into(),
        }
    }
}

/// A snapshot of a throughput control group's current state.
///
/// This provides an immutable view of the group's configuration at a point in time.
#[non_exhaustive]
#[derive(Clone, Debug)]
pub struct ThroughputControlGroupSnapshot {
    /// The group name.
    pub(crate) name: ThroughputControlGroupName,
    /// The container this group applies to.
    pub(crate) container: ContainerReference,
    /// Whether this is the default group for the container.
    pub(crate) is_default: bool,
    /// The current throughput bucket (server-side bucket only).
    pub(crate) throughput_bucket: Option<u32>,
    /// The current priority level.
    pub(crate) priority_level: Option<PriorityLevel>,
}

impl ThroughputControlGroupSnapshot {
    /// Creates a new snapshot with the required fields.
    ///
    /// Optional fields (`throughput_bucket`, `priority_level`)
    /// default to `None` and can be set via fluent `with_*` methods.
    pub fn new(
        name: ThroughputControlGroupName,
        container: ContainerReference,
        is_default: bool,
    ) -> Self {
        Self {
            name,
            container,
            is_default,
            throughput_bucket: None,
            priority_level: None,
        }
    }

    /// Sets the throughput bucket (server-side only).
    pub fn with_throughput_bucket(mut self, bucket: u32) -> Self {
        self.throughput_bucket = Some(bucket);
        self
    }

    /// Sets the priority level.
    pub fn with_priority_level(mut self, level: PriorityLevel) -> Self {
        self.priority_level = Some(level);
        self
    }

    /// Returns the group name.
    pub fn name(&self) -> &ThroughputControlGroupName {
        &self.name
    }

    /// Returns the container reference.
    pub fn container(&self) -> &ContainerReference {
        &self.container
    }

    /// Returns whether this is the default group.
    pub fn is_default(&self) -> bool {
        self.is_default
    }

    /// Returns the throughput bucket, if set.
    pub fn throughput_bucket(&self) -> Option<u32> {
        self.throughput_bucket
    }

    /// Returns the priority level, if set.
    pub fn priority_level(&self) -> Option<PriorityLevel> {
        self.priority_level
    }
}

impl From<&ThroughputControlGroupOptions> for ThroughputControlGroupSnapshot {
    fn from(group: &ThroughputControlGroupOptions) -> Self {
        let mut snapshot = Self::new(
            group.name().clone(),
            group.container().clone(),
            group.is_default(),
        );
        if let Some(bucket) = group.throughput_bucket() {
            snapshot = snapshot.with_throughput_bucket(bucket);
        }
        if let Some(level) = group.priority_level() {
            snapshot = snapshot.with_priority_level(level);
        }
        snapshot
    }
}

/// Error when registering a throughput control group.
///
/// This error type is intentionally not boxed since registration errors are
/// configuration-time errors that should be rare and visible to developers.
#[derive(Clone, Debug, PartialEq)]
pub enum ThroughputControlGroupRegistrationError {
    /// A group with the same key (container + name) already exists.
    DuplicateGroup(ThroughputControlGroupKey),
    /// Another group is already marked as default for this container.
    DuplicateDefault {
        container: ContainerReference,
        existing_default: ThroughputControlGroupName,
    },
}

impl std::fmt::Display for ThroughputControlGroupRegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DuplicateGroup(key) => {
                write!(
                    f,
                    "Throughput control group '{}' already registered for container",
                    key.name
                )
            }
            Self::DuplicateDefault {
                existing_default, ..
            } => {
                write!(
                    f,
                    "Container already has a default throughput control group: '{}'",
                    existing_default
                )
            }
        }
    }
}

impl std::error::Error for ThroughputControlGroupRegistrationError {}

/// Registry for throughput control groups.
///
/// Manages the collection of registered groups and provides lookup functionality.
/// This registry is built during `CosmosDriverRuntimeBuilder::build()` and is
/// immutable after runtime creation (except for mutable values within groups).
#[non_exhaustive]
#[derive(Clone, Debug, Default)]
pub struct ThroughputControlGroupRegistry {
    /// Groups keyed by (container, name) tuple.
    groups: HashMap<ThroughputControlGroupKey, Arc<ThroughputControlGroupOptions>>,
    /// Default group for each container.
    defaults: HashMap<ContainerReference, ThroughputControlGroupName>,
}

impl ThroughputControlGroupRegistry {
    /// Creates an empty registry.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a throughput control group.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - A group with the same (container, name) key already exists
    /// - Another group is already marked as default for the same container
    #[allow(clippy::result_large_err)]
    pub fn register(
        &mut self,
        group: ThroughputControlGroupOptions,
    ) -> Result<(), ThroughputControlGroupRegistrationError> {
        let key = group.key();

        // Check for duplicate key
        if self.groups.contains_key(&key) {
            return Err(ThroughputControlGroupRegistrationError::DuplicateGroup(key));
        }

        // Check for duplicate default
        if group.is_default() {
            if let Some(existing_default) = self.defaults.get(group.container()) {
                return Err(ThroughputControlGroupRegistrationError::DuplicateDefault {
                    container: group.container().clone(),
                    existing_default: existing_default.clone(),
                });
            }
            self.defaults
                .insert(group.container().clone(), group.name().clone());
        }

        self.groups.insert(key, Arc::new(group));
        Ok(())
    }

    /// Returns a group by its key (container + name).
    pub fn get(
        &self,
        key: &ThroughputControlGroupKey,
    ) -> Option<&Arc<ThroughputControlGroupOptions>> {
        self.groups.get(key)
    }

    /// Returns a group by container and name.
    pub fn get_by_container_and_name(
        &self,
        container: &ContainerReference,
        name: &ThroughputControlGroupName,
    ) -> Option<&Arc<ThroughputControlGroupOptions>> {
        let key = ThroughputControlGroupKey {
            container: container.clone(),
            name: name.clone(),
        };
        self.groups.get(&key)
    }

    /// Returns the default group for a container, if one exists.
    pub fn get_default_for_container(
        &self,
        container: &ContainerReference,
    ) -> Option<&Arc<ThroughputControlGroupOptions>> {
        self.defaults.get(container).and_then(|name| {
            let key = ThroughputControlGroupKey {
                container: container.clone(),
                name: name.clone(),
            };
            self.groups.get(&key)
        })
    }

    /// Returns all groups registered for a specific container.
    pub fn groups_for_container(
        &self,
        container: &ContainerReference,
    ) -> Vec<&Arc<ThroughputControlGroupOptions>> {
        self.groups
            .iter()
            .filter(|(key, _)| &key.container == container)
            .map(|(_, group)| group)
            .collect()
    }

    /// Returns the total number of registered groups.
    pub fn len(&self) -> usize {
        self.groups.len()
    }

    /// Returns true if no groups are registered.
    pub fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    /// Returns an iterator over all registered groups.
    pub fn iter(
        &self,
    ) -> impl Iterator<
        Item = (
            &ThroughputControlGroupKey,
            &Arc<ThroughputControlGroupOptions>,
        ),
    > {
        self.groups.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AccountReference, PartitionKeyDefinition, SystemProperties};
    use url::Url;

    fn test_partition_key_definition(path: &str) -> PartitionKeyDefinition {
        serde_json::from_str(&format!(r#"{{"paths":["{path}"]}}"#)).unwrap()
    }

    fn test_container_props() -> crate::models::ContainerProperties {
        crate::models::ContainerProperties {
            id: "testcontainer".into(),
            partition_key: test_partition_key_definition("/pk"),
            system_properties: SystemProperties::default(),
        }
    }

    fn test_container() -> ContainerReference {
        let account = AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "test-key",
        );
        ContainerReference::new(
            account,
            "testdb",
            "testdb_rid",
            "testcontainer",
            "testcontainer_rid",
            &test_container_props(),
        )
    }

    fn test_container_2() -> ContainerReference {
        let account = AccountReference::with_master_key(
            Url::parse("https://test.documents.azure.com:443/").unwrap(),
            "test-key",
        );
        ContainerReference::new(
            account,
            "testdb",
            "testdb_rid",
            "container2",
            "container2_rid",
            &test_container_props(),
        )
    }

    #[test]
    fn server_side_bucket_group_creation() {
        let container = test_container();
        let group = ThroughputControlGroupOptions::server_side_throughput_bucket(
            "bucket-group",
            container.clone(),
            100,
            false,
        );

        assert_eq!(group.name().as_str(), "bucket-group");
        assert!(!group.is_default());
        assert_eq!(group.throughput_bucket(), Some(100));
        assert!(group.priority_level().is_none());
    }

    #[test]
    fn server_side_priority_group_creation() {
        let container = test_container();
        let group = ThroughputControlGroupOptions::server_side_priority_based_throttling(
            "priority-group",
            container.clone(),
            PriorityLevel::Low,
            true,
        );

        assert_eq!(group.name().as_str(), "priority-group");
        assert!(group.is_default());
        assert!(group.throughput_bucket().is_none());
        assert_eq!(group.priority_level(), Some(PriorityLevel::Low));
    }

    #[test]
    fn registry_registration() {
        let mut registry = ThroughputControlGroupRegistry::new();
        let container = test_container();

        let group = ThroughputControlGroupOptions::server_side_throughput_bucket(
            "test-group",
            container.clone(),
            100,
            false,
        );

        assert!(registry.register(group).is_ok());
        assert_eq!(registry.len(), 1);

        let key = ThroughputControlGroupKey::new(container.clone(), "test-group");
        assert!(registry.get(&key).is_some());
    }

    #[test]
    fn registry_rejects_duplicate_key() {
        let mut registry = ThroughputControlGroupRegistry::new();
        let container = test_container();

        let group1 = ThroughputControlGroupOptions::server_side_throughput_bucket(
            "same-name",
            container.clone(),
            100,
            false,
        );
        let group2 = ThroughputControlGroupOptions::server_side_throughput_bucket(
            "same-name",
            container.clone(),
            100,
            false,
        );

        assert!(registry.register(group1).is_ok());
        let result = registry.register(group2);
        assert!(matches!(
            result,
            Err(ThroughputControlGroupRegistrationError::DuplicateGroup(_))
        ));
    }

    #[test]
    fn registry_rejects_duplicate_default() {
        let mut registry = ThroughputControlGroupRegistry::new();
        let container = test_container();

        let group1 = ThroughputControlGroupOptions::server_side_throughput_bucket(
            "default-1",
            container.clone(),
            100,
            true, // default
        );
        let group2 = ThroughputControlGroupOptions::server_side_throughput_bucket(
            "default-2",
            container.clone(),
            200,
            true, // also default - should fail
        );

        assert!(registry.register(group1).is_ok());
        let result = registry.register(group2);
        assert!(matches!(
            result,
            Err(ThroughputControlGroupRegistrationError::DuplicateDefault { .. })
        ));
    }

    #[test]
    fn same_name_different_containers_allowed() {
        let mut registry = ThroughputControlGroupRegistry::new();
        let container1 = test_container();
        let container2 = test_container_2();

        let group1 = ThroughputControlGroupOptions::server_side_throughput_bucket(
            "shared-name",
            container1.clone(),
            100,
            true,
        );
        let group2 = ThroughputControlGroupOptions::server_side_throughput_bucket(
            "shared-name",
            container2.clone(),
            200,
            true, // Both can be default since different containers
        );

        assert!(registry.register(group1).is_ok());
        assert!(registry.register(group2).is_ok());
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn get_default_for_container() {
        let mut registry = ThroughputControlGroupRegistry::new();
        let container = test_container();

        let default_group = ThroughputControlGroupOptions::server_side_throughput_bucket(
            "default-group",
            container.clone(),
            100,
            true,
        );
        let other_group = ThroughputControlGroupOptions::server_side_throughput_bucket(
            "other-group",
            container.clone(),
            200,
            false,
        );

        registry.register(default_group).unwrap();
        registry.register(other_group).unwrap();

        let default = registry.get_default_for_container(&container);
        assert!(default.is_some());
        assert_eq!(default.unwrap().name().as_str(), "default-group");
    }

    #[test]
    fn groups_for_container() {
        let mut registry = ThroughputControlGroupRegistry::new();
        let container1 = test_container();
        let container2 = test_container_2();

        let group1 = ThroughputControlGroupOptions::server_side_throughput_bucket(
            "group1",
            container1.clone(),
            100,
            false,
        );
        let group2 = ThroughputControlGroupOptions::server_side_throughput_bucket(
            "group2",
            container1.clone(),
            200,
            false,
        );
        let group3 = ThroughputControlGroupOptions::server_side_throughput_bucket(
            "group3",
            container2.clone(),
            300,
            false,
        );

        registry.register(group1).unwrap();
        registry.register(group2).unwrap();
        registry.register(group3).unwrap();

        let c1_groups = registry.groups_for_container(&container1);
        assert_eq!(c1_groups.len(), 2);

        let c2_groups = registry.groups_for_container(&container2);
        assert_eq!(c2_groups.len(), 1);
    }

    #[test]
    fn snapshot_captures_current_state() {
        let container = test_container();
        let group = ThroughputControlGroupOptions::server_side_priority_based_throttling(
            "snapshot-test",
            container.clone(),
            PriorityLevel::Low,
            true,
        );

        let snapshot = ThroughputControlGroupSnapshot::from(&group);
        assert_eq!(snapshot.name().as_str(), "snapshot-test");
        assert!(snapshot.is_default());
        assert_eq!(snapshot.priority_level(), Some(PriorityLevel::Low));
        assert!(snapshot.throughput_bucket().is_none());
    }

    #[test]
    fn snapshot_from_priority_group_has_no_bucket() {
        let container = test_container();
        let group = ThroughputControlGroupOptions::server_side_priority_based_throttling(
            "priority-only",
            container,
            PriorityLevel::High,
            false,
        );

        let snapshot = ThroughputControlGroupSnapshot::from(&group);
        assert_eq!(snapshot.priority_level(), Some(PriorityLevel::High));
        assert!(snapshot.throughput_bucket().is_none());
    }

    #[test]
    fn snapshot_from_bucket_group_has_no_priority() {
        let container = test_container();
        let group = ThroughputControlGroupOptions::server_side_throughput_bucket(
            "bucket-only",
            container,
            42,
            false,
        );

        let snapshot = ThroughputControlGroupSnapshot::from(&group);
        assert_eq!(snapshot.throughput_bucket(), Some(42));
        assert!(snapshot.priority_level().is_none());
    }

    #[test]
    fn registry_lookup_by_container_and_name() {
        let mut registry = ThroughputControlGroupRegistry::new();
        let container = test_container();

        let group = ThroughputControlGroupOptions::server_side_throughput_bucket(
            "lookup-test",
            container.clone(),
            10,
            false,
        );
        registry.register(group).unwrap();

        let name = ThroughputControlGroupName::new("lookup-test");
        let found = registry.get_by_container_and_name(&container, &name);
        assert!(found.is_some());
        assert_eq!(found.unwrap().throughput_bucket(), Some(10));

        let missing = ThroughputControlGroupName::new("no-such-group");
        assert!(registry
            .get_by_container_and_name(&container, &missing)
            .is_none());
    }
}

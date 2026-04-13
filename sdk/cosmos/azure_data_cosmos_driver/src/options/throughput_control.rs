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

/// Mutable runtime values for a throughput control group.
#[derive(Clone, Debug, Default)]
struct MutableValues {
    throughput_bucket: Option<u32>,
    priority_level: Option<PriorityLevel>,
}

/// Configuration for a throughput control group.
///
/// Registered at the runtime level and associated with a container.
/// Throughput control is enforced server-side.
///
/// A group can have a throughput bucket, a priority level, or both.
///
/// # Immutability
///
/// Once registered, the group's name, container, and `is_default` flag are immutable.
/// Only the target values (priority level, bucket) can be modified at runtime.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ThroughputControlGroupOptions {
    name: ThroughputControlGroupName,
    container: ContainerReference,
    is_default: bool,
    mutable: Arc<RwLock<MutableValues>>,
}

impl ThroughputControlGroupOptions {
    /// Creates a new throughput control group.
    pub fn new(
        name: impl Into<ThroughputControlGroupName>,
        container: ContainerReference,
        is_default: bool,
    ) -> Self {
        Self {
            name: name.into(),
            container,
            is_default,
            mutable: Arc::new(RwLock::new(MutableValues::default())),
        }
    }

    /// Sets the initial throughput bucket value.
    pub fn with_throughput_bucket(self, bucket: u32) -> Self {
        self.mutable.write().unwrap().throughput_bucket = Some(bucket);
        self
    }

    /// Sets the initial priority level.
    pub fn with_priority_level(self, level: PriorityLevel) -> Self {
        self.mutable.write().unwrap().priority_level = Some(level);
        self
    }

    /// Returns the name of the throughput control group.
    pub fn name(&self) -> &ThroughputControlGroupName {
        &self.name
    }

    /// Returns the container this group applies to.
    pub fn container(&self) -> &ContainerReference {
        &self.container
    }

    /// Returns whether this group is the default for its container.
    pub fn is_default(&self) -> bool {
        self.is_default
    }

    /// Returns the registry key for this group.
    pub(crate) fn key(&self) -> ThroughputControlGroupKey {
        ThroughputControlGroupKey {
            container: self.container.clone(),
            name: self.name.clone(),
        }
    }

    /// Returns the current throughput bucket, if set.
    pub fn throughput_bucket(&self) -> Option<u32> {
        self.mutable.read().unwrap().throughput_bucket
    }

    /// Sets the throughput bucket.
    pub fn set_throughput_bucket(&self, bucket: u32) {
        self.mutable.write().unwrap().throughput_bucket = Some(bucket);
    }

    /// Returns the current priority level, if set.
    pub fn priority_level(&self) -> Option<PriorityLevel> {
        self.mutable.read().unwrap().priority_level
    }

    /// Sets the priority level.
    pub fn set_priority_level(&self, level: PriorityLevel) {
        self.mutable.write().unwrap().priority_level = Some(level);
    }
}

/// Composite key for identifying a throughput control group.
///
/// Groups are uniquely identified by the combination of container and name.
/// The same group name can be registered for different containers.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct ThroughputControlGroupKey {
    /// The container this group applies to.
    pub(crate) container: ContainerReference,
    /// The group name.
    pub(crate) name: ThroughputControlGroupName,
}

#[allow(dead_code)] // used in tests
impl ThroughputControlGroupKey {
    /// Creates a new key.
    pub(crate) fn new(
        container: ContainerReference,
        name: impl Into<ThroughputControlGroupName>,
    ) -> Self {
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
#[allow(dead_code)] // name/container/is_default kept for diagnostics and future use
pub(crate) struct ThroughputControlGroupSnapshot {
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

#[allow(dead_code)] // some methods kept for diagnostics and future use
impl ThroughputControlGroupSnapshot {
    /// Creates a new snapshot with the required fields.
    ///
    /// Optional fields (`throughput_bucket`, `priority_level`)
    /// default to `None` and can be set via fluent `with_*` methods.
    pub(crate) fn new(
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

    /// Sets the throughput bucket.
    pub(crate) fn with_throughput_bucket(mut self, bucket: u32) -> Self {
        self.throughput_bucket = Some(bucket);
        self
    }

    /// Sets the priority level.
    pub(crate) fn with_priority_level(mut self, level: PriorityLevel) -> Self {
        self.priority_level = Some(level);
        self
    }

    /// Returns the group name.
    pub(crate) fn name(&self) -> &ThroughputControlGroupName {
        &self.name
    }

    /// Returns the container reference.
    pub(crate) fn container(&self) -> &ContainerReference {
        &self.container
    }

    /// Returns whether this is the default group.
    pub(crate) fn is_default(&self) -> bool {
        self.is_default
    }

    /// Returns the throughput bucket, if set.
    pub(crate) fn throughput_bucket(&self) -> Option<u32> {
        self.throughput_bucket
    }

    /// Returns the priority level, if set.
    pub(crate) fn priority_level(&self) -> Option<PriorityLevel> {
        self.priority_level
    }
}

impl From<&ThroughputControlGroupOptions> for ThroughputControlGroupSnapshot {
    fn from(group: &ThroughputControlGroupOptions) -> Self {
        let mutable = group.mutable.read().unwrap();
        let mut snapshot = Self::new(
            group.name().clone(),
            group.container().clone(),
            group.is_default(),
        );
        if let Some(bucket) = mutable.throughput_bucket {
            snapshot = snapshot.with_throughput_bucket(bucket);
        }
        if let Some(level) = mutable.priority_level {
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
pub(crate) enum ThroughputControlGroupRegistrationError {
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
pub(crate) struct ThroughputControlGroupRegistry {
    /// Groups keyed by (container, name) tuple.
    groups: HashMap<ThroughputControlGroupKey, Arc<ThroughputControlGroupOptions>>,
    /// Default group for each container.
    defaults: HashMap<ContainerReference, ThroughputControlGroupName>,
}

#[allow(dead_code)] // some methods only used in tests
impl ThroughputControlGroupRegistry {
    /// Creates an empty registry.
    pub(crate) fn new() -> Self {
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
    pub(crate) fn register(
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
    pub(crate) fn get(
        &self,
        key: &ThroughputControlGroupKey,
    ) -> Option<&Arc<ThroughputControlGroupOptions>> {
        self.groups.get(key)
    }

    /// Returns a group by container and name.
    pub(crate) fn get_by_container_and_name(
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
    pub(crate) fn get_default_for_container(
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
    pub(crate) fn groups_for_container(
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
    pub(crate) fn len(&self) -> usize {
        self.groups.len()
    }

    /// Returns true if no groups are registered.
    pub(crate) fn is_empty(&self) -> bool {
        self.groups.is_empty()
    }

    /// Returns an iterator over all registered groups.
    pub(crate) fn iter(
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
    fn bucket_group_creation() {
        let container = test_container();
        let group = ThroughputControlGroupOptions::new("bucket-group", container.clone(), false)
            .with_throughput_bucket(100);

        assert_eq!(group.name().as_str(), "bucket-group");
        assert!(!group.is_default());
        assert_eq!(group.throughput_bucket(), Some(100));
        assert!(group.priority_level().is_none());
    }

    #[test]
    fn priority_group_creation() {
        let container = test_container();
        let group = ThroughputControlGroupOptions::new("priority-group", container.clone(), true)
            .with_priority_level(PriorityLevel::Low);

        assert_eq!(group.name().as_str(), "priority-group");
        assert!(group.is_default());
        assert!(group.throughput_bucket().is_none());
        assert_eq!(group.priority_level(), Some(PriorityLevel::Low));
    }

    #[test]
    fn registry_registration() {
        let mut registry = ThroughputControlGroupRegistry::new();
        let container = test_container();

        let group = ThroughputControlGroupOptions::new("test-group", container.clone(), false)
            .with_throughput_bucket(100);

        assert!(registry.register(group).is_ok());
        assert_eq!(registry.len(), 1);

        let key = ThroughputControlGroupKey::new(container.clone(), "test-group");
        assert!(registry.get(&key).is_some());
    }

    #[test]
    fn registry_rejects_duplicate_key() {
        let mut registry = ThroughputControlGroupRegistry::new();
        let container = test_container();

        let group1 = ThroughputControlGroupOptions::new("same-name", container.clone(), false)
            .with_throughput_bucket(100);
        let group2 = ThroughputControlGroupOptions::new("same-name", container.clone(), false)
            .with_throughput_bucket(100);

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

        let group1 = ThroughputControlGroupOptions::new(
            "default-1",
            container.clone(),
            true, // default
        )
        .with_throughput_bucket(100);
        let group2 = ThroughputControlGroupOptions::new(
            "default-2",
            container.clone(),
            true, // also default - should fail
        )
        .with_throughput_bucket(200);

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

        let group1 = ThroughputControlGroupOptions::new("shared-name", container1.clone(), true)
            .with_throughput_bucket(100);
        let group2 = ThroughputControlGroupOptions::new(
            "shared-name",
            container2.clone(),
            true, // Both can be default since different containers
        )
        .with_throughput_bucket(200);

        assert!(registry.register(group1).is_ok());
        assert!(registry.register(group2).is_ok());
        assert_eq!(registry.len(), 2);
    }

    #[test]
    fn get_default_for_container() {
        let mut registry = ThroughputControlGroupRegistry::new();
        let container = test_container();

        let default_group =
            ThroughputControlGroupOptions::new("default-group", container.clone(), true)
                .with_throughput_bucket(100);
        let other_group =
            ThroughputControlGroupOptions::new("other-group", container.clone(), false)
                .with_throughput_bucket(200);

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

        let group1 = ThroughputControlGroupOptions::new("group1", container1.clone(), false)
            .with_throughput_bucket(100);
        let group2 = ThroughputControlGroupOptions::new("group2", container1.clone(), false)
            .with_throughput_bucket(200);
        let group3 = ThroughputControlGroupOptions::new("group3", container2.clone(), false)
            .with_throughput_bucket(300);

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
        let group = ThroughputControlGroupOptions::new("snapshot-test", container.clone(), true)
            .with_priority_level(PriorityLevel::Low);

        let snapshot = ThroughputControlGroupSnapshot::from(&group);
        assert_eq!(snapshot.name().as_str(), "snapshot-test");
        assert!(snapshot.is_default());
        assert_eq!(snapshot.priority_level(), Some(PriorityLevel::Low));
        assert!(snapshot.throughput_bucket().is_none());
    }

    #[test]
    fn registry_lookup_by_container_and_name() {
        let mut registry = ThroughputControlGroupRegistry::new();
        let container = test_container();

        let group = ThroughputControlGroupOptions::new("lookup-test", container.clone(), false)
            .with_throughput_bucket(10);
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

    #[test]
    fn set_throughput_bucket_succeeds() {
        let container = test_container();
        let group = ThroughputControlGroupOptions::new("bucket-group", container, false)
            .with_throughput_bucket(100);

        group.set_throughput_bucket(200);
        assert_eq!(group.throughput_bucket(), Some(200));
    }

    #[test]
    fn set_priority_level_succeeds() {
        let container = test_container();
        let group = ThroughputControlGroupOptions::new("priority-group", container, false)
            .with_priority_level(PriorityLevel::Low);

        group.set_priority_level(PriorityLevel::High);
        assert_eq!(group.priority_level(), Some(PriorityLevel::High));
    }

    #[test]
    fn group_with_both_bucket_and_priority() {
        let container = test_container();
        let group = ThroughputControlGroupOptions::new("both-group", container, false)
            .with_throughput_bucket(42)
            .with_priority_level(PriorityLevel::Low);

        assert_eq!(group.throughput_bucket(), Some(42));
        assert_eq!(group.priority_level(), Some(PriorityLevel::Low));
    }

    #[test]
    fn snapshot_reflects_mutation() {
        let container = test_container();
        let group = ThroughputControlGroupOptions::new("mutate-test", container, false)
            .with_throughput_bucket(100)
            .with_priority_level(PriorityLevel::Low);

        group.set_throughput_bucket(200);
        group.set_priority_level(PriorityLevel::High);

        let snapshot = ThroughputControlGroupSnapshot::from(&group);
        assert_eq!(snapshot.throughput_bucket(), Some(200));
        assert_eq!(snapshot.priority_level(), Some(PriorityLevel::High));
    }
}

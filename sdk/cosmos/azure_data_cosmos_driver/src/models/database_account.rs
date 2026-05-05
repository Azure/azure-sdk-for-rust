// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Database account properties model.
//!
//! Represents account-level configuration returned by the Cosmos DB service.
//! This is a wire-format model deserialized from the service response when
//! reading the database account.
//!
//! # .NET Comparison
//!
//! This corresponds to the .NET SDK's `AccountProperties` class, specifically
//! the properties relevant to PPAF dynamic enablement:
//! - `EnablePartitionLevelFailover` → [`DatabaseAccountProperties::enable_partition_level_failover`]
//! - `EnableMultipleWriteLocations` → [`DatabaseAccountProperties::enable_multiple_write_locations`]

use serde::Deserialize;

/// Properties of a Cosmos DB database account.
///
/// Deserialized from the service response when reading account metadata.
/// Contains account-level configuration flags used for routing decisions.
///
/// # Rust Concept: Serde and `#[serde(rename)]`
///
/// Rust uses the `serde` crate for serialization/deserialization (like JSON.NET
/// or Jackson). The `#[derive(Deserialize)]` macro auto-generates deserialization
/// code at compile time. `#[serde(rename = "...")]` maps Rust's `snake_case`
/// field names to the service's `camelCase` JSON property names.
///
/// The `#[serde(default)]` attribute means missing JSON fields get Rust's
/// `Default` value (e.g., `None` for `Option<T>`, `false` for `bool`).
///
/// # Example
///
/// ```
/// use azure_data_cosmos_driver::models::DatabaseAccountProperties;
///
/// let json = r#"{
///     "enablePerPartitionFailoverBehavior": true,
///     "enableMultipleWriteLocations": false
/// }"#;
///
/// let props: DatabaseAccountProperties = serde_json::from_str(json).unwrap();
/// assert_eq!(props.enable_partition_level_failover(), Some(true));
/// assert_eq!(props.enable_multiple_write_locations(), Some(false));
/// ```
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq)]
#[non_exhaustive]
pub struct DatabaseAccountProperties {
    /// Whether per-partition automatic failover is enabled for this account.
    ///
    /// `None` means the service did not report this property (treat as disabled).
    /// This maps to the .NET SDK's `AccountProperties.EnablePartitionLevelFailover`.
    #[serde(
        rename = "enablePerPartitionFailoverBehavior",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    enable_partition_level_failover: Option<bool>,

    /// Whether the account supports multi-region writes.
    ///
    /// When `true`, the account can write to multiple regions simultaneously.
    /// PPAF only applies to **single-write** accounts (where this is `false`).
    #[serde(
        rename = "enableMultipleWriteLocations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    enable_multiple_write_locations: Option<bool>,
}

impl DatabaseAccountProperties {
    /// Returns whether per-partition automatic failover is enabled.
    ///
    /// `None` means the service didn't report this property.
    pub fn enable_partition_level_failover(&self) -> Option<bool> {
        self.enable_partition_level_failover
    }

    /// Returns whether multi-region writes are enabled.
    ///
    /// `None` means the service didn't report this property.
    pub fn enable_multiple_write_locations(&self) -> Option<bool> {
        self.enable_multiple_write_locations
    }

    /// Returns `true` if this is a single-write account.
    ///
    /// PPAF only applies to single-write accounts. A missing value is treated
    /// as single-write (conservative default).
    pub fn is_single_write_account(&self) -> bool {
        !self.enable_multiple_write_locations.unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_full_properties() {
        let json = r#"{
            "enablePerPartitionFailoverBehavior": true,
            "enableMultipleWriteLocations": false
        }"#;

        let props: DatabaseAccountProperties = serde_json::from_str(json).unwrap();
        assert_eq!(props.enable_partition_level_failover(), Some(true));
        assert_eq!(props.enable_multiple_write_locations(), Some(false));
        assert!(props.is_single_write_account());
    }

    #[test]
    fn deserialize_missing_fields_defaults_to_none() {
        let json = r#"{}"#;

        let props: DatabaseAccountProperties = serde_json::from_str(json).unwrap();
        assert_eq!(props.enable_partition_level_failover(), None);
        assert_eq!(props.enable_multiple_write_locations(), None);
        assert!(props.is_single_write_account());
    }

    #[test]
    fn deserialize_ppaf_disabled() {
        let json = r#"{
            "enablePerPartitionFailoverBehavior": false
        }"#;

        let props: DatabaseAccountProperties = serde_json::from_str(json).unwrap();
        assert_eq!(props.enable_partition_level_failover(), Some(false));
    }

    #[test]
    fn multi_write_account_detected() {
        let json = r#"{
            "enableMultipleWriteLocations": true
        }"#;

        let props: DatabaseAccountProperties = serde_json::from_str(json).unwrap();
        assert!(!props.is_single_write_account());
    }

    #[test]
    fn ignores_unknown_fields() {
        let json = r#"{
            "enablePerPartitionFailoverBehavior": true,
            "someOtherProperty": "value",
            "anotherFlag": 42
        }"#;

        let props: DatabaseAccountProperties = serde_json::from_str(json).unwrap();
        assert_eq!(props.enable_partition_level_failover(), Some(true));
    }
}

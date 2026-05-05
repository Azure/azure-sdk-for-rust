// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Per-Partition Automatic Failover (PPAF) configuration types.
//!
//! PPAF allows the SDK to fail over write requests at the partition key range
//! level, rather than failing over the entire account. This module provides
//! client-side policy configuration for controlling PPAF behavior.
//!
//! # .NET Comparison
//!
//! This corresponds to the .NET SDK's:
//! - `ConnectionPolicy.EnablePartitionLevelFailover` → [`PartitionLevelFailoverPolicy`]
//! - `ConnectionPolicy.DisablePartitionLevelFailoverClientLevelOverride` → [`PartitionLevelFailoverPolicy::Disabled`]
//!
//! In .NET, PPAF is controlled by a combination of a boolean property and an
//! override flag. In Rust, we model this as a single enum with three variants,
//! which is more idiomatic and eliminates the "two booleans" ambiguity.

/// Policy for partition-level automatic failover behavior.
///
/// This controls whether the driver uses per-partition failover for write
/// operations on single-write accounts. The policy is set at the driver
/// (client) level and interacts with the server-side account configuration.
///
/// # Resolution Logic
///
/// The effective PPAF state is computed from this policy and the server-side
/// `enablePerPartitionFailoverBehavior` account property:
///
/// | Policy            | Server Flag    | Effective PPAF |
/// |-------------------|----------------|----------------|
/// | `ServerControlled`| `Some(true)`   | Enabled        |
/// | `ServerControlled`| `Some(false)`  | Disabled       |
/// | `ServerControlled`| `None`         | Disabled       |
/// | `Enabled`         | _(any)_        | Enabled        |
/// | `Disabled`        | _(any)_        | Disabled       |
///
/// # Rust Concept: Enums as Algebraic Types
///
/// Unlike C#/Java enums which are just named integers, Rust enums are
/// **algebraic data types** (also called "sum types"). Each variant can
/// optionally carry data. Here we use a simple enum (no data), but this
/// pattern scales to complex cases like `Result<T, E>` and `Option<T>`.
///
/// # Example
///
/// ```
/// use azure_data_cosmos_driver::options::PartitionLevelFailoverPolicy;
///
/// // Default: let the server decide
/// let policy = PartitionLevelFailoverPolicy::default();
/// assert_eq!(policy, PartitionLevelFailoverPolicy::ServerControlled);
///
/// // Force enable regardless of server config
/// let policy = PartitionLevelFailoverPolicy::Enabled;
///
/// // Force disable (client override)
/// let policy = PartitionLevelFailoverPolicy::Disabled;
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum PartitionLevelFailoverPolicy {
    /// Let the server-side account configuration decide whether PPAF is enabled.
    ///
    /// When the server reports `enablePerPartitionFailoverBehavior: true`, PPAF
    /// will be enabled. Otherwise, it remains disabled. This is the default.
    #[default]
    ServerControlled,

    /// Force-enable PPAF regardless of server configuration.
    ///
    /// Use this when you want partition-level failover even if the account
    /// hasn't enabled it server-side.
    Enabled,

    /// Force-disable PPAF regardless of server configuration.
    ///
    /// This is equivalent to .NET's `DisablePartitionLevelFailoverClientLevelOverride = true`.
    /// The server-side flag is ignored entirely.
    Disabled,
}

impl PartitionLevelFailoverPolicy {
    /// Resolves the effective PPAF enabled state given the server-side flag.
    ///
    /// # Arguments
    ///
    /// * `server_flag` - The `enablePerPartitionFailoverBehavior` value from
    ///   the Cosmos DB account properties. `None` means the server didn't
    ///   report a value (treated as disabled).
    ///
    /// # Example
    ///
    /// ```
    /// use azure_data_cosmos_driver::options::PartitionLevelFailoverPolicy;
    ///
    /// // ServerControlled respects the server flag
    /// assert!(PartitionLevelFailoverPolicy::ServerControlled.resolve(Some(true)));
    /// assert!(!PartitionLevelFailoverPolicy::ServerControlled.resolve(Some(false)));
    /// assert!(!PartitionLevelFailoverPolicy::ServerControlled.resolve(None));
    ///
    /// // Enabled always returns true
    /// assert!(PartitionLevelFailoverPolicy::Enabled.resolve(None));
    ///
    /// // Disabled always returns false
    /// assert!(!PartitionLevelFailoverPolicy::Disabled.resolve(Some(true)));
    /// ```
    pub fn resolve(&self, server_flag: Option<bool>) -> bool {
        match self {
            Self::ServerControlled => server_flag.unwrap_or(false),
            Self::Enabled => true,
            Self::Disabled => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_server_controlled() {
        assert_eq!(
            PartitionLevelFailoverPolicy::default(),
            PartitionLevelFailoverPolicy::ServerControlled
        );
    }

    #[test]
    fn resolve_server_controlled() {
        let policy = PartitionLevelFailoverPolicy::ServerControlled;
        assert!(policy.resolve(Some(true)));
        assert!(!policy.resolve(Some(false)));
        assert!(!policy.resolve(None));
    }

    #[test]
    fn resolve_enabled_overrides_server() {
        let policy = PartitionLevelFailoverPolicy::Enabled;
        assert!(policy.resolve(Some(true)));
        assert!(policy.resolve(Some(false)));
        assert!(policy.resolve(None));
    }

    #[test]
    fn resolve_disabled_overrides_server() {
        let policy = PartitionLevelFailoverPolicy::Disabled;
        assert!(!policy.resolve(Some(true)));
        assert!(!policy.resolve(Some(false)));
        assert!(!policy.resolve(None));
    }
}

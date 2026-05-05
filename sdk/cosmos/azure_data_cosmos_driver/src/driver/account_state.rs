// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Cached account state for PPAF dynamic enablement.
//!
//! This module manages the runtime account state that drives per-partition
//! automatic failover decisions. It is separate from the options hierarchy
//! because it represents **server-discovered capability**, not client configuration.
//!
//! # .NET Comparison
//!
//! This corresponds to the combination of:
//! - `GlobalPartitionEndpointManagerCore.isPartitionLevelAutomaticFailoverEnabled` (atomic int)
//! - The cached `AccountProperties` in `GlobalEndpointManager`
//!
//! In .NET, the atomic flag is toggled via `Interlocked.Exchange`. In Rust,
//! we use `AtomicBool` which provides the same lock-free atomic operations.
//!
//! # Rust Concept: Atomic Types
//!
//! `std::sync::atomic::AtomicBool` provides lock-free thread-safe boolean operations.
//! Unlike `RwLock<bool>`, atomics never block — they use CPU-level atomic instructions.
//! This is critical for the PPAF check which happens on every request (hot path).
//!
//! `Ordering::SeqCst` (sequentially consistent) is the strongest ordering guarantee —
//! all threads see atomic operations in the same order. It's slightly slower than
//! `Relaxed` but eliminates subtle concurrency bugs. For a flag checked once per
//! request and updated rarely, the performance difference is negligible.

use crate::{models::DatabaseAccountProperties, options::PartitionLevelFailoverPolicy};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};

/// Cached account state used for PPAF routing decisions.
///
/// This is owned by the driver and updated when account metadata is refreshed.
/// The PPAF enabled flag is stored as an `AtomicBool` for lock-free reads on
/// the hot path (every request checks this).
///
/// # Thread Safety
///
/// - `is_ppaf_enabled`: `AtomicBool` — lock-free reads/writes
/// - `account_properties`: `RwLock` — for infrequent metadata updates
///
/// # Rust Concept: `Arc` (Atomic Reference Counting)
///
/// `Arc<T>` is Rust's thread-safe reference-counted smart pointer (like
/// `std::shared_ptr` in C++ or reference counting in Swift). When you clone
/// an `Arc`, it increments a reference count atomically. When all `Arc`s are
/// dropped, the inner value is deallocated. This lets multiple threads share
/// ownership of data without knowing which thread will use it last.
///
/// # Example
///
/// ```
/// use azure_data_cosmos_driver::driver::AccountState;
/// use azure_data_cosmos_driver::models::DatabaseAccountProperties;
/// use azure_data_cosmos_driver::options::PartitionLevelFailoverPolicy;
///
/// let state = AccountState::new();
///
/// // Initially PPAF is disabled
/// assert!(!state.is_ppaf_enabled());
///
/// // Simulate account refresh with PPAF enabled
/// let props: DatabaseAccountProperties = serde_json::from_str(
///     r#"{"enablePerPartitionFailoverBehavior": true}"#
/// ).unwrap();
///
/// state.update_from_account_properties(
///     &props,
///     PartitionLevelFailoverPolicy::ServerControlled,
/// );
///
/// assert!(state.is_ppaf_enabled());
/// ```
#[derive(Debug)]
pub struct AccountState {
    /// Lock-free PPAF enabled flag — the resolved effective state.
    ///
    /// This is the "hot path" flag checked on every request.
    /// Updated by `update_from_account_properties()`.
    is_ppaf_enabled: AtomicBool,

    /// Whether the account supports multi-region writes.
    /// PPAF only applies to single-write accounts.
    is_multi_write: AtomicBool,

    /// Cached account properties from the last metadata refresh.
    account_properties: RwLock<Option<DatabaseAccountProperties>>,
}

impl AccountState {
    /// Creates a new account state with PPAF disabled.
    pub fn new() -> Self {
        Self {
            is_ppaf_enabled: AtomicBool::new(false),
            is_multi_write: AtomicBool::new(false),
            account_properties: RwLock::new(None),
        }
    }

    /// Returns whether PPAF is currently enabled.
    ///
    /// This is a lock-free atomic read, safe to call on every request.
    pub fn is_ppaf_enabled(&self) -> bool {
        self.is_ppaf_enabled.load(Ordering::SeqCst)
    }

    /// Returns whether the account supports multi-region writes.
    pub fn is_multi_write(&self) -> bool {
        self.is_multi_write.load(Ordering::SeqCst)
    }

    /// Returns a snapshot of the cached account properties.
    pub fn account_properties(&self) -> Option<DatabaseAccountProperties> {
        self.account_properties
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    /// Updates account state from refreshed account properties.
    ///
    /// This resolves the effective PPAF state using the client policy and the
    /// server-side flag, then atomically updates the cached state.
    ///
    /// # Arguments
    ///
    /// * `properties` - Fresh account properties from the service
    /// * `client_policy` - The client-side PPAF policy from `DriverOptions`
    ///
    /// # Returns
    ///
    /// `true` if the effective PPAF state changed, `false` otherwise.
    ///
    /// # .NET Comparison
    ///
    /// This combines the logic from:
    /// - `GlobalEndpointManager.InitializeAccountPropertiesAndStartBackgroundRefresh()`
    /// - `DocumentClient.UpdatePartitionLevelFailoverConfigWithAccountRefresh()`
    pub fn update_from_account_properties(
        &self,
        properties: &DatabaseAccountProperties,
        client_policy: PartitionLevelFailoverPolicy,
    ) -> bool {
        // Resolve effective PPAF state
        let server_flag = properties.enable_partition_level_failover();
        let new_ppaf = client_policy.resolve(server_flag);

        // Update multi-write state
        let new_multi_write = !properties.is_single_write_account();
        self.is_multi_write.store(new_multi_write, Ordering::SeqCst);

        // Atomically swap PPAF flag — returns the previous value
        // This is equivalent to .NET's `Interlocked.Exchange`
        let previous = self.is_ppaf_enabled.swap(new_ppaf, Ordering::SeqCst);

        // Update cached properties
        {
            let mut guard = self
                .account_properties
                .write()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            *guard = Some(properties.clone());
        }

        // Return whether state changed
        previous != new_ppaf
    }

    /// Directly sets the PPAF enabled state.
    ///
    /// Used for testing or when the state needs to be set without a full
    /// account properties update.
    ///
    /// # .NET Comparison
    ///
    /// This maps to `GlobalPartitionEndpointManagerCore.SetIsPPAFEnabled()`.
    pub fn set_ppaf_enabled(&self, enabled: bool) {
        self.is_ppaf_enabled.store(enabled, Ordering::SeqCst);
    }
}

impl Default for AccountState {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe shared reference to account state.
///
/// This is cloneable and can be shared across threads via `Arc`.
///
/// # Rust Concept: Newtype Pattern
///
/// `SharedAccountState(Arc<AccountState>)` is a "newtype" — a single-field
/// tuple struct that wraps another type. It's zero-cost at runtime (same
/// memory layout as the inner type) but gives us a distinct type for:
/// - Adding methods specific to shared usage
/// - Implementing traits differently than the inner type
/// - Self-documenting code about ownership semantics
#[derive(Clone, Debug)]
pub struct SharedAccountState(Arc<AccountState>);

impl SharedAccountState {
    /// Creates a new shared account state.
    pub fn new() -> Self {
        Self(Arc::new(AccountState::new()))
    }

    /// Returns whether PPAF is currently enabled (lock-free).
    pub fn is_ppaf_enabled(&self) -> bool {
        self.0.is_ppaf_enabled()
    }

    /// Returns whether the account supports multi-region writes.
    pub fn is_multi_write(&self) -> bool {
        self.0.is_multi_write()
    }

    /// Updates state from refreshed account properties.
    ///
    /// Returns `true` if the effective PPAF state changed.
    pub fn update_from_account_properties(
        &self,
        properties: &DatabaseAccountProperties,
        client_policy: PartitionLevelFailoverPolicy,
    ) -> bool {
        self.0
            .update_from_account_properties(properties, client_policy)
    }

    /// Returns a snapshot of the cached account properties.
    pub fn account_properties(&self) -> Option<DatabaseAccountProperties> {
        self.0.account_properties()
    }

    /// Directly sets the PPAF enabled state.
    pub fn set_ppaf_enabled(&self, enabled: bool) {
        self.0.set_ppaf_enabled(enabled);
    }
}

impl Default for SharedAccountState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_props(ppaf: Option<bool>, multi_write: Option<bool>) -> DatabaseAccountProperties {
        let json = match (ppaf, multi_write) {
            (Some(p), Some(m)) => format!(
                r#"{{"enablePerPartitionFailoverBehavior": {p}, "enableMultipleWriteLocations": {m}}}"#
            ),
            (Some(p), None) => format!(r#"{{"enablePerPartitionFailoverBehavior": {p}}}"#),
            (None, Some(m)) => format!(r#"{{"enableMultipleWriteLocations": {m}}}"#),
            (None, None) => "{}".to_string(),
        };
        serde_json::from_str(&json).unwrap()
    }

    #[test]
    fn initial_state_ppaf_disabled() {
        let state = AccountState::new();
        assert!(!state.is_ppaf_enabled());
        assert!(!state.is_multi_write());
        assert!(state.account_properties().is_none());
    }

    #[test]
    fn update_enables_ppaf_with_server_controlled() {
        let state = AccountState::new();
        let props = make_props(Some(true), Some(false));

        let changed = state
            .update_from_account_properties(&props, PartitionLevelFailoverPolicy::ServerControlled);

        assert!(changed);
        assert!(state.is_ppaf_enabled());
        assert!(!state.is_multi_write());
    }

    #[test]
    fn update_keeps_ppaf_disabled_when_server_says_false() {
        let state = AccountState::new();
        let props = make_props(Some(false), Some(false));

        let changed = state
            .update_from_account_properties(&props, PartitionLevelFailoverPolicy::ServerControlled);

        assert!(!changed); // was false, still false
        assert!(!state.is_ppaf_enabled());
    }

    #[test]
    fn client_enabled_overrides_server_false() {
        let state = AccountState::new();
        let props = make_props(Some(false), Some(false));

        let changed =
            state.update_from_account_properties(&props, PartitionLevelFailoverPolicy::Enabled);

        assert!(changed);
        assert!(state.is_ppaf_enabled());
    }

    #[test]
    fn client_disabled_overrides_server_true() {
        let state = AccountState::new();
        let props = make_props(Some(true), Some(false));

        let changed =
            state.update_from_account_properties(&props, PartitionLevelFailoverPolicy::Disabled);

        assert!(!changed); // was false, still false
        assert!(!state.is_ppaf_enabled());
    }

    #[test]
    fn multi_write_detected() {
        let state = AccountState::new();
        let props = make_props(Some(true), Some(true));

        state
            .update_from_account_properties(&props, PartitionLevelFailoverPolicy::ServerControlled);

        assert!(state.is_multi_write());
    }

    #[test]
    fn no_change_returns_false() {
        let state = AccountState::new();
        let props = make_props(Some(true), Some(false));

        // First update: false -> true = changed
        assert!(state.update_from_account_properties(
            &props,
            PartitionLevelFailoverPolicy::ServerControlled,
        ));

        // Second update with same: true -> true = no change
        assert!(!state.update_from_account_properties(
            &props,
            PartitionLevelFailoverPolicy::ServerControlled,
        ));
    }

    #[test]
    fn shared_state_delegates_correctly() {
        let shared = SharedAccountState::new();
        assert!(!shared.is_ppaf_enabled());

        let props = make_props(Some(true), Some(false));
        assert!(shared.update_from_account_properties(
            &props,
            PartitionLevelFailoverPolicy::ServerControlled,
        ));

        assert!(shared.is_ppaf_enabled());
        assert!(shared.account_properties().is_some());
    }

    #[test]
    fn direct_set_ppaf() {
        let state = AccountState::new();
        assert!(!state.is_ppaf_enabled());

        state.set_ppaf_enabled(true);
        assert!(state.is_ppaf_enabled());

        state.set_ppaf_enabled(false);
        assert!(!state.is_ppaf_enabled());
    }

    #[test]
    fn missing_server_flag_treated_as_disabled() {
        let state = AccountState::new();
        let props = make_props(None, Some(false));

        state
            .update_from_account_properties(&props, PartitionLevelFailoverPolicy::ServerControlled);

        assert!(!state.is_ppaf_enabled());
    }
}

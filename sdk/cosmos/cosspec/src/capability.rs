// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use std::fmt;

/// A backend feature a spec may demand with `#[requires(...)]`.
///
/// Capabilities do two jobs:
///
/// * **Filtering:** a spec runs only in cells whose backend and service profile
///   provide every capability it requires. Otherwise it is skipped with a reason, or
///   fails when `AZURE_COSMOS_SPEC_STRICT=1`.
/// * **Enforcement:** a framework call that needs a capability (for example,
///   [`ExclusiveContainer::split`](crate::ExclusiveContainer::split)) fails with
///   [`InvalidConfiguration`](crate::InvalidConfiguration) unless the spec declared it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Capability {
    /// Split or merge physical partitions arbitrarily, by EPK range.
    TopologyControl,
    /// Split a one-partition container into two, or merge a two-partition
    /// container back into one. Every backend with `TopologyControl` provides it,
    /// and backends without arbitrary control can too.
    SimpleSplitMerge,
    /// Take regions offline or bring them back. Only usable on an
    /// [`ExclusiveAccount`](crate::ExclusiveAccount).
    RegionControl,
    /// The account has more than one region.
    MultiRegion,
    /// Every region of the account accepts writes.
    MultiWrite,
    /// A dedicated account can be created for one spec.
    AccountIsolation,
}

impl Capability {
    /// Every capability, in declaration order.
    pub const ALL: &'static [Capability] = &[
        Capability::TopologyControl,
        Capability::SimpleSplitMerge,
        Capability::RegionControl,
        Capability::MultiRegion,
        Capability::MultiWrite,
        Capability::AccountIsolation,
    ];

    /// The variant name, as written in `#[requires(...)]`.
    pub fn name(self) -> &'static str {
        match self {
            Capability::TopologyControl => "TopologyControl",
            Capability::SimpleSplitMerge => "SimpleSplitMerge",
            Capability::RegionControl => "RegionControl",
            Capability::MultiRegion => "MultiRegion",
            Capability::MultiWrite => "MultiWrite",
            Capability::AccountIsolation => "AccountIsolation",
        }
    }
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

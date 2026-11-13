// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Service profiles (account shapes) and container profiles (container shapes).

use crate::{
    testdata::{self, Dataset},
    Capability, RegionAlias,
};
use serde::Deserialize;
use std::fmt;

/// The shape of the account a spec runs against.
///
/// Service profiles say nothing about consistency: specs that depend on a
/// consistency level set it on the request.
///
/// In configuration files, a profile is written as its variant name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize)]
pub enum ServiceProfile {
    /// One region that accepts reads and writes: `PrimaryReadWrite`.
    SingleRegion,
    /// Two regions: `PrimaryReadWrite` accepts writes, `SecondaryReadOnly` doesn't.
    MultiRead,
    /// Two regions that both accept writes: `PrimaryReadWrite` and `SecondaryReadWrite`.
    MultiWrite,
}

impl ServiceProfile {
    /// Every service profile. Used by `#[service_profiles(all)]`, the default.
    pub const ALL: &'static [ServiceProfile] = &[
        ServiceProfile::SingleRegion,
        ServiceProfile::MultiRead,
        ServiceProfile::MultiWrite,
    ];

    /// The variant name, as written in `#[service_profiles(...)]` and used in test names.
    pub fn name(self) -> &'static str {
        match self {
            ServiceProfile::SingleRegion => "SingleRegion",
            ServiceProfile::MultiRead => "MultiRead",
            ServiceProfile::MultiWrite => "MultiWrite",
        }
    }

    /// Parses a profile name.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|p| p.name() == name)
    }

    /// The account's regions, by alias, in preference order. The first one is
    /// the write region.
    pub fn regions(self) -> &'static [RegionAlias] {
        match self {
            ServiceProfile::SingleRegion => &[RegionAlias::PrimaryReadWrite],
            ServiceProfile::MultiRead => &[
                RegionAlias::PrimaryReadWrite,
                RegionAlias::SecondaryReadOnly,
            ],
            ServiceProfile::MultiWrite => &[
                RegionAlias::PrimaryReadWrite,
                RegionAlias::SecondaryReadWrite,
            ],
        }
    }

    /// The capabilities this account shape provides on its own.
    pub fn capabilities(self) -> &'static [Capability] {
        match self {
            ServiceProfile::SingleRegion => &[],
            ServiceProfile::MultiRead => &[Capability::MultiRegion],
            ServiceProfile::MultiWrite => &[Capability::MultiRegion, Capability::MultiWrite],
        }
    }
}

impl fmt::Display for ServiceProfile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// The shape and contents of a container a spec asks for with `#[profile(...)]`.
#[derive(Clone, Copy, Debug)]
pub struct ContainerProfile {
    /// The profile name, used in container names.
    pub name: &'static str,
    /// The partition key path, for example `/pk`.
    pub partition_key_path: &'static str,
    /// The number of physical partitions to start with. Backends that cannot
    /// control this exactly provision at least this many.
    pub physical_partitions: u32,
    /// The items seeded into the container before the spec runs.
    pub dataset: fn() -> Dataset,
}

/// An empty container partitioned on `/pk` with one physical partition.
pub fn empty() -> ContainerProfile {
    ContainerProfile {
        name: "empty",
        partition_key_path: "/pk",
        physical_partitions: 1,
        dataset: Dataset::empty,
    }
}

/// [`testdata::items::basic_items`] in a container partitioned on `/pk` with one
/// physical partition.
pub fn basic_items() -> ContainerProfile {
    ContainerProfile {
        name: "basic_items",
        partition_key_path: "/pk",
        physical_partitions: 1,
        dataset: testdata::items::basic_items,
    }
}

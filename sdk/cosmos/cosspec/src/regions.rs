// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Region aliases: how specs name an account's regions.
//!
//! Specs never use real region names, because live accounts can be deployed
//! anywhere. Each [`ServiceProfile`] names its regions with [`RegionAlias`]es, and
//! each backend account resolves them to real regions with a [`RegionMap`].

use crate::{InvalidConfiguration, ServiceProfile, SpecResult};
use azure_data_cosmos::options::Region;
use serde::Deserialize;
use std::fmt;

/// A role a region plays in a service profile.
///
/// In configuration files, an alias is written as its variant name.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize)]
pub enum RegionAlias {
    /// The first preferred region. It accepts writes in every profile.
    PrimaryReadWrite,
    /// The second region of a [`MultiRead`](ServiceProfile::MultiRead) account.
    SecondaryReadOnly,
    /// The second region of a [`MultiWrite`](ServiceProfile::MultiWrite) account.
    SecondaryReadWrite,
}

impl RegionAlias {
    /// The variant name, as written in configuration and shown in results.
    pub fn name(self) -> &'static str {
        match self {
            RegionAlias::PrimaryReadWrite => "PrimaryReadWrite",
            RegionAlias::SecondaryReadOnly => "SecondaryReadOnly",
            RegionAlias::SecondaryReadWrite => "SecondaryReadWrite",
        }
    }
}

impl fmt::Display for RegionAlias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// The real region behind each of an account's region aliases, in preference order.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RegionMap {
    profile: ServiceProfile,
    regions: Vec<(RegionAlias, Region)>,
}

impl RegionMap {
    /// Maps each of `profile`'s aliases to a region with `resolve`.
    ///
    /// Fails if `resolve` has no region for an alias, or maps two aliases to
    /// the same region.
    pub fn new(
        profile: ServiceProfile,
        mut resolve: impl FnMut(RegionAlias) -> Option<Region>,
    ) -> SpecResult<Self> {
        let mut regions: Vec<(RegionAlias, Region)> = Vec::new();
        for &alias in profile.regions() {
            let region =
                resolve(alias).ok_or_else(|| format!("{profile} needs a region for {alias}"))?;
            if let Some((other, _)) = regions.iter().find(|(_, r)| *r == region) {
                return Err(format!("{other} and {alias} are both mapped to {region}").into());
            }
            regions.push((alias, region));
        }
        Ok(Self { profile, regions })
    }

    /// The real region for `alias`.
    ///
    /// Fails with [`InvalidConfiguration`] if the account's profile has no such
    /// region, for example `SecondaryReadOnly` on a `MultiWrite` account.
    pub fn region(&self, alias: RegionAlias) -> SpecResult<Region> {
        self.regions
            .iter()
            .find(|(a, _)| *a == alias)
            .map(|(_, region)| region.clone())
            .ok_or_else(|| {
                InvalidConfiguration(format!("{} has no {alias} region", self.profile)).into()
            })
    }

    /// The real regions, in preference order. The first one accepts writes.
    pub fn preferred(&self) -> Vec<Region> {
        self.regions.iter().map(|(_, r)| r.clone()).collect()
    }

    /// Every alias with its region, in preference order.
    pub fn iter(&self) -> impl Iterator<Item = (RegionAlias, &Region)> {
        self.regions.iter().map(|(a, r)| (*a, r))
    }
}

#[cfg(test)]
mod tests {
    use super::{RegionAlias, RegionMap};
    use crate::ServiceProfile;
    use azure_data_cosmos::options::Region;

    #[test]
    fn resolves_aliases_in_preference_order() {
        let map = RegionMap::new(ServiceProfile::MultiRead, |alias| match alias {
            RegionAlias::PrimaryReadWrite => Some(Region::new("North Europe")),
            _ => Some(Region::new("West Europe")),
        })
        .unwrap();
        assert_eq!(
            map.preferred(),
            [Region::new("northeurope"), Region::new("westeurope")]
        );
        assert_eq!(
            map.region(RegionAlias::SecondaryReadOnly).unwrap(),
            Region::new("westeurope")
        );
    }

    #[test]
    fn rejects_aliases_outside_the_profile() {
        let map = RegionMap::new(ServiceProfile::MultiWrite, |alias| {
            Some(Region::new(alias.name()))
        })
        .unwrap();
        let error = map.region(RegionAlias::SecondaryReadOnly).unwrap_err();
        assert!(error
            .to_string()
            .contains("MultiWrite has no SecondaryReadOnly"));
    }

    #[test]
    fn rejects_missing_and_duplicate_regions() {
        let missing = RegionMap::new(ServiceProfile::MultiRead, |alias| {
            (alias == RegionAlias::PrimaryReadWrite).then_some(Region::EAST_US)
        });
        assert!(missing
            .unwrap_err()
            .to_string()
            .contains("SecondaryReadOnly"));
        let duplicate = RegionMap::new(ServiceProfile::MultiRead, |_| Some(Region::EAST_US));
        assert!(duplicate.unwrap_err().to_string().contains("both mapped"));
    }
}

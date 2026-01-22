// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Region Names relevant to Azure Cosmos DB APIs.

use std::borrow::Cow;
use std::fmt;
use std::hash::{Hash, Hasher};

/// A newtype wrapper for Azure region names that provides canonical comparison.
///
/// Region names are compared case-insensitively and ignoring whitespace characters.
/// This ensures that "West US", "westus", and "WEST US" are all considered equal.
#[derive(Clone, Debug)]
pub struct RegionName(Cow<'static, str>);

impl RegionName {
    /// Creates a new `RegionName` from a static string.
    pub const fn from_static(s: &'static str) -> Self {
        Self(Cow::Borrowed(s))
    }

    /// Returns the canonical form of the region name (lowercase, no whitespace).
    fn canonical(&self) -> String {
        self.0
            .chars()
            .filter(|c| !c.is_whitespace())
            .flat_map(|c| c.to_lowercase())
            .collect()
    }

    /// Returns the original region name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for RegionName {
    fn from(s: String) -> Self {
        Self(Cow::Owned(s))
    }
}

impl From<&'static str> for RegionName {
    fn from(s: &'static str) -> Self {
        Self(Cow::Borrowed(s))
    }
}

impl From<Cow<'static, str>> for RegionName {
    fn from(s: Cow<'static, str>) -> Self {
        Self(s)
    }
}

impl AsRef<str> for RegionName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for RegionName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl PartialEq for RegionName {
    fn eq(&self, other: &Self) -> bool {
        self.canonical() == other.canonical()
    }
}

impl Eq for RegionName {}

impl Hash for RegionName {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.canonical().hash(state);
    }
}

// cSpell:disable
pub const WEST_US: RegionName = RegionName::from_static("West US");
pub const WEST_US_2: RegionName = RegionName::from_static("West US 2");
pub const WEST_CENTRAL_US: RegionName = RegionName::from_static("West Central US");
pub const EAST_US: RegionName = RegionName::from_static("East US");
pub const EAST_US_2: RegionName = RegionName::from_static("East US 2");
pub const EAST_US_3: RegionName = RegionName::from_static("East US 3");
pub const CENTRAL_US: RegionName = RegionName::from_static("Central US");
pub const SOUTH_CENTRAL_US: RegionName = RegionName::from_static("South Central US");
pub const NORTH_CENTRAL_US: RegionName = RegionName::from_static("North Central US");
pub const WEST_EUROPE: RegionName = RegionName::from_static("West Europe");
pub const NORTH_EUROPE: RegionName = RegionName::from_static("North Europe");
pub const EAST_ASIA: RegionName = RegionName::from_static("East Asia");
pub const SOUTHEAST_ASIA: RegionName = RegionName::from_static("Southeast Asia");
pub const JAPAN_EAST: RegionName = RegionName::from_static("Japan East");
pub const JAPAN_WEST: RegionName = RegionName::from_static("Japan West");
pub const AUSTRALIA_EAST: RegionName = RegionName::from_static("Australia East");
pub const AUSTRALIA_SOUTHEAST: RegionName = RegionName::from_static("Australia Southeast");
pub const CENTRAL_INDIA: RegionName = RegionName::from_static("Central India");
pub const SOUTH_INDIA: RegionName = RegionName::from_static("South India");
pub const WEST_INDIA: RegionName = RegionName::from_static("West India");
pub const CANADA_EAST: RegionName = RegionName::from_static("Canada East");
pub const CANADA_CENTRAL: RegionName = RegionName::from_static("Canada Central");
pub const CHINA_NORTH: RegionName = RegionName::from_static("China North");
pub const CHINA_EAST: RegionName = RegionName::from_static("China East");
pub const CHINA_NORTH_2: RegionName = RegionName::from_static("China North 2");
pub const CHINA_EAST_2: RegionName = RegionName::from_static("China East 2");
pub const KOREA_SOUTH: RegionName = RegionName::from_static("Korea South");
pub const KOREA_CENTRAL: RegionName = RegionName::from_static("Korea Central");
pub const UK_WEST: RegionName = RegionName::from_static("UK West");
pub const UK_SOUTH: RegionName = RegionName::from_static("UK South");
pub const BRAZIL_SOUTH: RegionName = RegionName::from_static("Brazil South");
pub const USGOV_ARIZONA: RegionName = RegionName::from_static("USGov Arizona");
pub const USGOV_TEXAS: RegionName = RegionName::from_static("USGov Texas");
pub const USGOV_VIRGINIA: RegionName = RegionName::from_static("USGov Virginia");
pub const EAST_US_2_EUAP: RegionName = RegionName::from_static("East US 2 EUAP");
pub const CENTRAL_US_EUAP: RegionName = RegionName::from_static("Central US EUAP");
pub const FRANCE_CENTRAL: RegionName = RegionName::from_static("France Central");
pub const FRANCE_SOUTH: RegionName = RegionName::from_static("France South");
pub const USDOD_CENTRAL: RegionName = RegionName::from_static("USDoD Central");
pub const USDOD_EAST: RegionName = RegionName::from_static("USDoD East");
pub const AUSTRALIA_CENTRAL: RegionName = RegionName::from_static("Australia Central");
pub const AUSTRALIA_CENTRAL_2: RegionName = RegionName::from_static("Australia Central 2");
pub const SOUTH_AFRICA_NORTH: RegionName = RegionName::from_static("South Africa North");
pub const SOUTH_AFRICA_WEST: RegionName = RegionName::from_static("South Africa West");
pub const UAE_CENTRAL: RegionName = RegionName::from_static("UAE Central");
pub const UAE_NORTH: RegionName = RegionName::from_static("UAE North");
pub const USNAT_EAST: RegionName = RegionName::from_static("USNat East");
pub const USNAT_WEST: RegionName = RegionName::from_static("USNat West");
pub const USSEC_EAST: RegionName = RegionName::from_static("USSec East");
pub const USSEC_WEST: RegionName = RegionName::from_static("USSec West");
pub const USSEC_WEST_CENTRAL: RegionName = RegionName::from_static("USSec West Central");
pub const SWITZERLAND_NORTH: RegionName = RegionName::from_static("Switzerland North");
pub const SWITZERLAND_WEST: RegionName = RegionName::from_static("Switzerland West");
pub const GERMANY_NORTH: RegionName = RegionName::from_static("Germany North");
pub const GERMANY_WEST_CENTRAL: RegionName = RegionName::from_static("Germany West Central");
pub const NORWAY_EAST: RegionName = RegionName::from_static("Norway East");
pub const NORWAY_WEST: RegionName = RegionName::from_static("Norway West");
pub const BRAZIL_SOUTHEAST: RegionName = RegionName::from_static("Brazil Southeast");
pub const WEST_US_3: RegionName = RegionName::from_static("West US 3");
pub const JIO_INDIA_CENTRAL: RegionName = RegionName::from_static("Jio India Central");
pub const JIO_INDIA_WEST: RegionName = RegionName::from_static("Jio India West");
pub const EAST_US_SLV: RegionName = RegionName::from_static("East US SLV");
pub const SWEDEN_CENTRAL: RegionName = RegionName::from_static("Sweden Central");
pub const SWEDEN_SOUTH: RegionName = RegionName::from_static("Sweden South");
pub const QATAR_CENTRAL: RegionName = RegionName::from_static("Qatar Central");
pub const CHINA_NORTH_3: RegionName = RegionName::from_static("China North 3");
pub const CHINA_EAST_3: RegionName = RegionName::from_static("China East 3");
pub const POLAND_CENTRAL: RegionName = RegionName::from_static("Poland Central");
pub const MALAYSIA_SOUTH: RegionName = RegionName::from_static("Malaysia South");
pub const ITALY_NORTH: RegionName = RegionName::from_static("Italy North");
pub const ISRAEL_CENTRAL: RegionName = RegionName::from_static("Israel Central");
pub const MEXICO_CENTRAL: RegionName = RegionName::from_static("Mexico Central");
pub const SPAIN_CENTRAL: RegionName = RegionName::from_static("Spain Central");
pub const TAIWAN_NORTH: RegionName = RegionName::from_static("Taiwan North");
pub const TAIWAN_NORTHWEST: RegionName = RegionName::from_static("Taiwan Northwest");
pub const NEW_ZEALAND_NORTH: RegionName = RegionName::from_static("New Zealand North");
pub const AUSTRIA_EAST: RegionName = RegionName::from_static("Austria East");
pub const BLEU_FRANCE_CENTRAL: RegionName = RegionName::from_static("Bleu France Central");
pub const BLEU_FRANCE_SOUTH: RegionName = RegionName::from_static("Bleu France South");
pub const INDONESIA_CENTRAL: RegionName = RegionName::from_static("Indonesia Central");
pub const SOUTHEAST_US: RegionName = RegionName::from_static("Southeast US");
pub const SOUTHWEST_US: RegionName = RegionName::from_static("Southwest US");
pub const MALAYSIA_WEST: RegionName = RegionName::from_static("Malaysia West");
pub const DELOS_CLOUD_GERMANY_CENTRAL: RegionName =
    RegionName::from_static("Delos Cloud Germany Central");
pub const DELOS_CLOUD_GERMANY_NORTH: RegionName =
    RegionName::from_static("Delos Cloud Germany North");
pub const CHILE_CENTRAL: RegionName = RegionName::from_static("Chile Central");
pub const SOUTH_CENTRAL_US_2: RegionName = RegionName::from_static("South Central US 2");
pub const ISRAEL_NORTHWEST: RegionName = RegionName::from_static("Israel Northwest");
pub const BELGIUM_CENTRAL: RegionName = RegionName::from_static("Belgium Central");
pub const DENMARK_EAST: RegionName = RegionName::from_static("Denmark East");
pub const SOUTHEAST_US_3: RegionName = RegionName::from_static("Southeast US 3");
pub const SOUTHEAST_US_5: RegionName = RegionName::from_static("Southeast US 5");
pub const NORTHEAST_US_5: RegionName = RegionName::from_static("Northeast US 5");
pub const INDIA_SOUTH_CENTRAL: RegionName = RegionName::from_static("India South Central");
pub const SINGAPORE_CENTRAL: RegionName = RegionName::from_static("Singapore Central");
pub const SINGAPORE_NORTH: RegionName = RegionName::from_static("Singapore North");

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn canonical_comparison() {
        let r1 = RegionName::from("West US");
        let r2 = RegionName::from("westus");
        let r3 = RegionName::from("WEST US");
        let r4 = RegionName::from("West  US");
        let r5 = RegionName::from("EastUS");

        assert_eq!(r1, r2);
        assert_eq!(r1, r3);
        assert_eq!(r1, r4);
        assert_ne!(r1, r5);
    }

    #[test]
    fn hash_consistency() {
        let mut set = HashSet::new();
        set.insert(RegionName::from("West US"));

        assert!(set.contains(&RegionName::from("westus")));
        assert!(set.contains(&RegionName::from("WEST US")));
        assert!(set.contains(&RegionName::from("West  US")));
        assert!(!set.contains(&RegionName::from("East US")));
    }

    #[test]
    fn display_preserves_original() {
        let r1 = RegionName::from("West US");
        assert_eq!(r1.to_string(), "West US");

        let r2 = RegionName::from("westus");
        assert_eq!(r2.to_string(), "westus");
    }

    #[test]
    fn as_str_returns_original() {
        let r = RegionName::from("West US");
        assert_eq!(r.as_str(), "West US");
    }

    #[test]
    fn from_cow() {
        let borrowed = RegionName::from(Cow::Borrowed("West US"));
        assert_eq!(borrowed.as_str(), "West US");

        let owned = RegionName::from(Cow::Owned("West US".to_string()));
        assert_eq!(owned.as_str(), "West US");
    }
}

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
pub const WEST_US: &str = "West US";
pub const WEST_US_2: &str = "West US 2";
pub const WEST_CENTRAL_US: &str = "West Central US";
pub const EAST_US: &str = "East US";
pub const EAST_US_2: &str = "East US 2";
pub const EAST_US_3: &str = "East US 3";
pub const CENTRAL_US: &str = "Central US";
pub const SOUTH_CENTRAL_US: &str = "South Central US";
pub const NORTH_CENTRAL_US: &str = "North Central US";
pub const WEST_EUROPE: &str = "West Europe";
pub const NORTH_EUROPE: &str = "North Europe";
pub const EAST_ASIA: &str = "East Asia";
pub const SOUTHEAST_ASIA: &str = "Southeast Asia";
pub const JAPAN_EAST: &str = "Japan East";
pub const JAPAN_WEST: &str = "Japan West";
pub const AUSTRALIA_EAST: &str = "Australia East";
pub const AUSTRALIA_SOUTHEAST: &str = "Australia Southeast";
pub const CENTRAL_INDIA: &str = "Central India";
pub const SOUTH_INDIA: &str = "South India";
pub const WEST_INDIA: &str = "West India";
pub const CANADA_EAST: &str = "Canada East";
pub const CANADA_CENTRAL: &str = "Canada Central";
pub const CHINA_NORTH: &str = "China North";
pub const CHINA_EAST: &str = "China East";
pub const CHINA_NORTH_2: &str = "China North 2";
pub const CHINA_EAST_2: &str = "China East 2";
pub const KOREA_SOUTH: &str = "Korea South";
pub const KOREA_CENTRAL: &str = "Korea Central";
pub const UK_WEST: &str = "UK West";
pub const UK_SOUTH: &str = "UK South";
pub const BRAZIL_SOUTH: &str = "Brazil South";
pub const USGOV_ARIZONA: &str = "USGov Arizona";
pub const USGOV_TEXAS: &str = "USGov Texas";
pub const USGOV_VIRGINIA: &str = "USGov Virginia";
pub const EAST_US_2_EUAP: &str = "East US 2 EUAP";
pub const CENTRAL_US_EUAP: &str = "Central US EUAP";
pub const FRANCE_CENTRAL: &str = "France Central";
pub const FRANCE_SOUTH: &str = "France South";
pub const USDOD_CENTRAL: &str = "USDoD Central";
pub const USDOD_EAST: &str = "USDoD East";
pub const AUSTRALIA_CENTRAL: &str = "Australia Central";
pub const AUSTRALIA_CENTRAL_2: &str = "Australia Central 2";
pub const SOUTH_AFRICA_NORTH: &str = "South Africa North";
pub const SOUTH_AFRICA_WEST: &str = "South Africa West";
pub const UAE_CENTRAL: &str = "UAE Central";
pub const UAE_NORTH: &str = "UAE North";
pub const USNAT_EAST: &str = "USNat East";
pub const USNAT_WEST: &str = "USNat West";
pub const USSEC_EAST: &str = "USSec East";
pub const USSEC_WEST: &str = "USSec West";
pub const USSEC_WEST_CENTRAL: &str = "USSec West Central";
pub const SWITZERLAND_NORTH: &str = "Switzerland North";
pub const SWITZERLAND_WEST: &str = "Switzerland West";
pub const GERMANY_NORTH: &str = "Germany North";
pub const GERMANY_WEST_CENTRAL: &str = "Germany West Central";
pub const NORWAY_EAST: &str = "Norway East";
pub const NORWAY_WEST: &str = "Norway West";
pub const BRAZIL_SOUTHEAST: &str = "Brazil Southeast";
pub const WEST_US_3: &str = "West US 3";
pub const JIO_INDIA_CENTRAL: &str = "Jio India Central";
pub const JIO_INDIA_WEST: &str = "Jio India West";
pub const EAST_US_SLV: &str = "East US SLV";
pub const SWEDEN_CENTRAL: &str = "Sweden Central";
pub const SWEDEN_SOUTH: &str = "Sweden South";
pub const QATAR_CENTRAL: &str = "Qatar Central";
pub const CHINA_NORTH_3: &str = "China North 3";
pub const CHINA_EAST_3: &str = "China East 3";
pub const POLAND_CENTRAL: &str = "Poland Central";
pub const MALAYSIA_SOUTH: &str = "Malaysia South";
pub const ITALY_NORTH: &str = "Italy North";
pub const ISRAEL_CENTRAL: &str = "Israel Central";
pub const MEXICO_CENTRAL: &str = "Mexico Central";
pub const SPAIN_CENTRAL: &str = "Spain Central";
pub const TAIWAN_NORTH: &str = "Taiwan North";
pub const TAIWAN_NORTHWEST: &str = "Taiwan Northwest";
pub const NEW_ZEALAND_NORTH: &str = "New Zealand North";
pub const AUSTRIA_EAST: &str = "Austria East";
pub const BLEU_FRANCE_CENTRAL: &str = "Bleu France Central";
pub const BLEU_FRANCE_SOUTH: &str = "Bleu France South";
pub const INDONESIA_CENTRAL: &str = "Indonesia Central";
pub const SOUTHEAST_US: &str = "Southeast US";
pub const SOUTHWEST_US: &str = "Southwest US";
pub const MALAYSIA_WEST: &str = "Malaysia West";
pub const DELOS_CLOUD_GERMANY_CENTRAL: &str = "Delos Cloud Germany Central";
pub const DELOS_CLOUD_GERMANY_NORTH: &str = "Delos Cloud Germany North";
pub const CHILE_CENTRAL: &str = "Chile Central";
pub const SOUTH_CENTRAL_US_2: &str = "South Central US 2";
pub const ISRAEL_NORTHWEST: &str = "Israel Northwest";
pub const BELGIUM_CENTRAL: &str = "Belgium Central";
pub const DENMARK_EAST: &str = "Denmark East";
pub const SOUTHEAST_US_3: &str = "Southeast US 3";
pub const SOUTHEAST_US_5: &str = "Southeast US 5";
pub const NORTHEAST_US_5: &str = "Northeast US 5";
pub const INDIA_SOUTH_CENTRAL: &str = "India South Central";
pub const SINGAPORE_CENTRAL: &str = "Singapore Central";
pub const SINGAPORE_NORTH: &str = "Singapore North";

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

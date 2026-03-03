// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

// cSpell:disable

//! Region Names relevant to Azure Cosmos DB APIs.

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::borrow::Cow;
use std::fmt;

/// A newtype wrapper for Azure region names that provides canonical comparison.
///
/// Region names are stored in canonical form (lowercase, no whitespace) to ensure
/// efficient comparison. This ensures that "West US", "westus", and "WEST US" are
/// all considered equal and stored identically.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RegionName(Cow<'static, str>);

impl Serialize for RegionName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for RegionName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(RegionName::new(s))
    }
}

/// Creates a `RegionName` from a static string that must already be in canonical form.
///
/// # Panics
/// Panics at compile-time if the string is not canonical (lowercase ASCII letters and digits only).
const fn from_static_canonical(s: &'static str) -> RegionName {
    ensure_canonical_ascii(s);
    RegionName(Cow::Borrowed(s))
}

/// Validates that a string contains only lowercase ASCII letters and digits.
///
/// # Panics
/// Panics at compile-time if the string contains any character that is not a lowercase ASCII letter or digit.
const fn ensure_canonical_ascii(s: &str) {
    let mut i = 0;
    let bytes = s.as_bytes();
    while i < bytes.len() {
        if !bytes[i].is_ascii_lowercase() && !bytes[i].is_ascii_digit() {
            panic!("string contains non-lowercase or non-ASCII character");
        }
        i += 1;
    }
}

impl RegionName {
    /// Creates a new `RegionName`, converting the input to canonical form if needed.
    ///
    /// The canonical form is lowercase with all whitespace removed.
    pub fn new(s: impl Into<Cow<'static, str>>) -> Self {
        let cow = s.into();
        let needs_canonicalization = cow.chars().any(|c| c.is_whitespace() || c.is_uppercase());

        if needs_canonicalization {
            let canonical: String = cow
                .chars()
                .filter(|c| !c.is_whitespace())
                .flat_map(|c| c.to_lowercase())
                .collect();
            Self(Cow::Owned(canonical))
        } else {
            Self(cow)
        }
    }

    /// Returns the canonical region name as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for RegionName {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

impl From<&'static str> for RegionName {
    fn from(s: &'static str) -> Self {
        Self::new(s)
    }
}

impl From<Cow<'static, str>> for RegionName {
    fn from(s: Cow<'static, str>) -> Self {
        Self::new(s)
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

// cSpell:disable
pub const WEST_US: RegionName = from_static_canonical("westus");
pub const WEST_US_2: RegionName = from_static_canonical("westus2");
pub const WEST_CENTRAL_US: RegionName = from_static_canonical("westcentralus");
pub const EAST_US: RegionName = from_static_canonical("eastus");
pub const EAST_US_2: RegionName = from_static_canonical("eastus2");
pub const EAST_US_3: RegionName = from_static_canonical("eastus3");
pub const CENTRAL_US: RegionName = from_static_canonical("centralus");
pub const SOUTH_CENTRAL_US: RegionName = from_static_canonical("southcentralus");
pub const NORTH_CENTRAL_US: RegionName = from_static_canonical("northcentralus");
pub const WEST_EUROPE: RegionName = from_static_canonical("westeurope");
pub const NORTH_EUROPE: RegionName = from_static_canonical("northeurope");
pub const EAST_ASIA: RegionName = from_static_canonical("eastasia");
pub const SOUTHEAST_ASIA: RegionName = from_static_canonical("southeastasia");
pub const JAPAN_EAST: RegionName = from_static_canonical("japaneast");
pub const JAPAN_WEST: RegionName = from_static_canonical("japanwest");
pub const AUSTRALIA_EAST: RegionName = from_static_canonical("australiaeast");
pub const AUSTRALIA_SOUTHEAST: RegionName = from_static_canonical("australiasoutheast");
pub const CENTRAL_INDIA: RegionName = from_static_canonical("centralindia");
pub const SOUTH_INDIA: RegionName = from_static_canonical("southindia");
pub const WEST_INDIA: RegionName = from_static_canonical("westindia");
pub const CANADA_EAST: RegionName = from_static_canonical("canadaeast");
pub const CANADA_CENTRAL: RegionName = from_static_canonical("canadacentral");
pub const CHINA_NORTH: RegionName = from_static_canonical("chinanorth");
pub const CHINA_EAST: RegionName = from_static_canonical("chinaeast");
pub const CHINA_NORTH_2: RegionName = from_static_canonical("chinanorth2");
pub const CHINA_EAST_2: RegionName = from_static_canonical("chinaeast2");
pub const KOREA_SOUTH: RegionName = from_static_canonical("koreasouth");
pub const KOREA_CENTRAL: RegionName = from_static_canonical("koreacentral");
pub const UK_WEST: RegionName = from_static_canonical("ukwest");
pub const UK_SOUTH: RegionName = from_static_canonical("uksouth");
pub const BRAZIL_SOUTH: RegionName = from_static_canonical("brazilsouth");
pub const USGOV_ARIZONA: RegionName = from_static_canonical("usgovarizona");
pub const USGOV_TEXAS: RegionName = from_static_canonical("usgovtexas");
pub const USGOV_VIRGINIA: RegionName = from_static_canonical("usgovvirginia");
pub const EAST_US_2_EUAP: RegionName = from_static_canonical("eastus2euap");
pub const CENTRAL_US_EUAP: RegionName = from_static_canonical("centraluseuap");
pub const FRANCE_CENTRAL: RegionName = from_static_canonical("francecentral");
pub const FRANCE_SOUTH: RegionName = from_static_canonical("francesouth");
pub const USDOD_CENTRAL: RegionName = from_static_canonical("usdodcentral");
pub const USDOD_EAST: RegionName = from_static_canonical("usdodeast");
pub const AUSTRALIA_CENTRAL: RegionName = from_static_canonical("australiacentral");
pub const AUSTRALIA_CENTRAL_2: RegionName = from_static_canonical("australiacentral2");
pub const SOUTH_AFRICA_NORTH: RegionName = from_static_canonical("southafricanorth");
pub const SOUTH_AFRICA_WEST: RegionName = from_static_canonical("southafricawest");
pub const UAE_CENTRAL: RegionName = from_static_canonical("uaecentral");
pub const UAE_NORTH: RegionName = from_static_canonical("uaenorth");
pub const USNAT_EAST: RegionName = from_static_canonical("usnateast");
pub const USNAT_WEST: RegionName = from_static_canonical("usnatwest");
pub const USSEC_EAST: RegionName = from_static_canonical("usseceast");
pub const USSEC_WEST: RegionName = from_static_canonical("ussecwest");
pub const USSEC_WEST_CENTRAL: RegionName = from_static_canonical("ussecwestcentral");
pub const SWITZERLAND_NORTH: RegionName = from_static_canonical("switzerlandnorth");
pub const SWITZERLAND_WEST: RegionName = from_static_canonical("switzerlandwest");
pub const GERMANY_NORTH: RegionName = from_static_canonical("germanynorth");
pub const GERMANY_WEST_CENTRAL: RegionName = from_static_canonical("germanywestcentral");
pub const NORWAY_EAST: RegionName = from_static_canonical("norwayeast");
pub const NORWAY_WEST: RegionName = from_static_canonical("norwaywest");
pub const BRAZIL_SOUTHEAST: RegionName = from_static_canonical("brazilsoutheast");
pub const WEST_US_3: RegionName = from_static_canonical("westus3");
pub const JIO_INDIA_CENTRAL: RegionName = from_static_canonical("jioindiacentral");
pub const JIO_INDIA_WEST: RegionName = from_static_canonical("jioindiawest");
pub const EAST_US_SLV: RegionName = from_static_canonical("eastusslv");
pub const SWEDEN_CENTRAL: RegionName = from_static_canonical("swedencentral");
pub const SWEDEN_SOUTH: RegionName = from_static_canonical("swedensouth");
pub const QATAR_CENTRAL: RegionName = from_static_canonical("qatarcentral");
pub const CHINA_NORTH_3: RegionName = from_static_canonical("chinanorth3");
pub const CHINA_EAST_3: RegionName = from_static_canonical("chinaeast3");
pub const POLAND_CENTRAL: RegionName = from_static_canonical("polandcentral");
pub const MALAYSIA_SOUTH: RegionName = from_static_canonical("malaysiasouth");
pub const ITALY_NORTH: RegionName = from_static_canonical("italynorth");
pub const ISRAEL_CENTRAL: RegionName = from_static_canonical("israelcentral");
pub const MEXICO_CENTRAL: RegionName = from_static_canonical("mexicocentral");
pub const SPAIN_CENTRAL: RegionName = from_static_canonical("spaincentral");
pub const TAIWAN_NORTH: RegionName = from_static_canonical("taiwannorth");
pub const TAIWAN_NORTHWEST: RegionName = from_static_canonical("taiwannorthwest");
pub const NEW_ZEALAND_NORTH: RegionName = from_static_canonical("newzealandnorth");
pub const AUSTRIA_EAST: RegionName = from_static_canonical("austriaeast");
pub const BLEU_FRANCE_CENTRAL: RegionName = from_static_canonical("bleufrancecentral");
pub const BLEU_FRANCE_SOUTH: RegionName = from_static_canonical("bleufrancesouth");
pub const INDONESIA_CENTRAL: RegionName = from_static_canonical("indonesiacentral");
pub const SOUTHEAST_US: RegionName = from_static_canonical("southeastus");
pub const SOUTHWEST_US: RegionName = from_static_canonical("southwestus");
pub const MALAYSIA_WEST: RegionName = from_static_canonical("malaysiawest");
pub const DELOS_CLOUD_GERMANY_CENTRAL: RegionName =
    from_static_canonical("deloscloudgermanycentral");
pub const DELOS_CLOUD_GERMANY_NORTH: RegionName = from_static_canonical("deloscloudgermanynorth");
pub const CHILE_CENTRAL: RegionName = from_static_canonical("chilecentral");
pub const SOUTH_CENTRAL_US_2: RegionName = from_static_canonical("southcentralus2");
pub const ISRAEL_NORTHWEST: RegionName = from_static_canonical("israelnorthwest");
pub const BELGIUM_CENTRAL: RegionName = from_static_canonical("belgiumcentral");
pub const DENMARK_EAST: RegionName = from_static_canonical("denmarkeast");
pub const SOUTHEAST_US_3: RegionName = from_static_canonical("southeastus3");
pub const SOUTHEAST_US_5: RegionName = from_static_canonical("southeastus5");
pub const NORTHEAST_US_5: RegionName = from_static_canonical("northeastus5");
pub const INDIA_SOUTH_CENTRAL: RegionName = from_static_canonical("indiasouthcentral");
pub const SINGAPORE_CENTRAL: RegionName = from_static_canonical("singaporecentral");
pub const SINGAPORE_NORTH: RegionName = from_static_canonical("singaporenorth");

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
    fn stores_canonical_form() {
        let r1 = RegionName::from("West US");
        assert_eq!(r1.to_string(), "westus");
        assert_eq!(r1.as_str(), "westus");

        let r2 = RegionName::from("westus");
        assert_eq!(r2.to_string(), "westus");
        assert_eq!(r2.as_str(), "westus");

        let r3 = RegionName::from("WEST US");
        assert_eq!(r3.as_str(), "westus");
    }

    #[test]
    fn constants_are_canonical() {
        assert_eq!(WEST_US.as_str(), "westus");
        assert_eq!(EAST_US.as_str(), "eastus");
        assert_eq!(WEST_EUROPE.as_str(), "westeurope");
    }

    #[test]
    #[should_panic(expected = "string contains non-lowercase or non-ASCII character")]
    fn from_static_canonical_panics_on_uppercase() {
        let _invalid = from_static_canonical("WestUS");
    }

    #[test]
    #[should_panic(expected = "string contains non-lowercase or non-ASCII character")]
    fn from_static_canonical_panics_on_spaces() {
        let _invalid = from_static_canonical("west us");
    }

    #[test]
    #[should_panic(expected = "string contains non-lowercase or non-ASCII character")]
    fn from_static_canonical_panics_on_special_chars() {
        let _invalid = from_static_canonical("west-us");
    }

    #[test]
    fn from_cow() {
        let borrowed = RegionName::from(Cow::Borrowed("West US"));
        assert_eq!(borrowed.as_str(), "westus");

        let owned = RegionName::from(Cow::Owned("West US".to_string()));
        assert_eq!(owned.as_str(), "westus");
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Azure region identifier types.
//!
//! This module provides the [`Region`] type for representing Azure regions in a type-safe manner
//! with normalization support and constants for all known Azure regions.

use serde::{Deserialize, Deserializer, Serialize};
use std::borrow::Cow;
use std::fmt;

/// Azure region identifier.
///
/// Represents an Azure region with normalization support (case-insensitive, whitespace-agnostic).
/// Input strings like "WESTUS2", "WestUS 2", and "West US 2" are all normalized to "westus2".
///
/// Normalization is applied both when constructing via [`Region::new`] and when
/// deserializing from JSON, so a service response containing `"West US 2"` will
/// produce the same `Region` as the constant [`Region::WEST_US_2`].
///
/// # Examples
///
/// ```
/// use azure_data_cosmos_driver::options::Region;
///
/// // Use predefined constants
/// let region = Region::WEST_US_2;
/// assert_eq!(region.as_str(), "westus2");
/// assert_eq!(region.display_name(), "West US 2");
///
/// // Create from various formats (all equivalent)
/// let r1 = Region::new("WESTUS2");
/// let r2 = Region::new("WestUS 2");
/// let r3 = Region::new("West US 2");
/// assert_eq!(r1, r2);
/// assert_eq!(r2, r3);
///
/// // Unknown regions use normalized name for display_name()
/// let custom = Region::new("East US 9");
/// assert_eq!(custom.as_str(), "eastus9");
/// assert_eq!(custom.display_name(), "eastus9");
/// ```
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
#[serde(transparent)]
pub struct Region {
    normalized: Cow<'static, str>,
}

// Custom Deserialize implementation that normalizes region names on
// deserialization. This ensures service responses like `"West US 2"` produce
// the same canonical `Region` as `Region::new("West US 2")`.
impl<'de> Deserialize<'de> for Region {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = String::deserialize(deserializer)?;
        Ok(Region::new(raw))
    }
}

impl Region {
    // ===== Known Azure Region Constants =====
    // These use the normalized names internally and rely on the display name lookup table.

    pub const EAST_US: Region = Region::new_const("eastus");
    pub const EAST_US_2: Region = Region::new_const("eastus2");
    pub const CENTRAL_US: Region = Region::new_const("centralus");
    pub const NORTH_CENTRAL_US: Region = Region::new_const("northcentralus");
    pub const SOUTH_CENTRAL_US: Region = Region::new_const("southcentralus");
    pub const WEST_CENTRAL_US: Region = Region::new_const("westcentralus");
    pub const WEST_US: Region = Region::new_const("westus");
    pub const WEST_US_2: Region = Region::new_const("westus2");
    pub const CANADA_EAST: Region = Region::new_const("canadaeast");
    pub const CANADA_CENTRAL: Region = Region::new_const("canadacentral");
    pub const BRAZIL_SOUTH: Region = Region::new_const("brazilsouth");
    pub const NORTH_EUROPE: Region = Region::new_const("northeurope");
    pub const WEST_EUROPE: Region = Region::new_const("westeurope");
    pub const FRANCE_CENTRAL: Region = Region::new_const("francecentral");
    pub const FRANCE_SOUTH: Region = Region::new_const("francesouth");
    pub const UK_WEST: Region = Region::new_const("ukwest");
    pub const UK_SOUTH: Region = Region::new_const("uksouth");
    pub const GERMANY_CENTRAL: Region = Region::new_const("germanycentral");
    pub const GERMANY_NORTHEAST: Region = Region::new_const("germanynortheast");
    pub const GERMANY_NORTH: Region = Region::new_const("germanynorth");
    pub const GERMANY_WEST_CENTRAL: Region = Region::new_const("germanywestcentral");
    pub const SWITZERLAND_NORTH: Region = Region::new_const("switzerlandnorth");
    pub const SWITZERLAND_WEST: Region = Region::new_const("switzerlandwest");
    pub const SOUTHEAST_ASIA: Region = Region::new_const("southeastasia");
    pub const EAST_ASIA: Region = Region::new_const("eastasia");
    pub const AUSTRALIA_EAST: Region = Region::new_const("australiaeast");
    pub const AUSTRALIA_SOUTHEAST: Region = Region::new_const("australiasoutheast");
    pub const AUSTRALIA_CENTRAL: Region = Region::new_const("australiacentral");
    pub const AUSTRALIA_CENTRAL_2: Region = Region::new_const("australiacentral2");
    pub const CHINA_EAST: Region = Region::new_const("chinaeast");
    pub const CHINA_NORTH: Region = Region::new_const("chinanorth");
    pub const CENTRAL_INDIA: Region = Region::new_const("centralindia");
    pub const WEST_INDIA: Region = Region::new_const("westindia");
    pub const SOUTH_INDIA: Region = Region::new_const("southindia");
    pub const JAPAN_EAST: Region = Region::new_const("japaneast");
    pub const JAPAN_WEST: Region = Region::new_const("japanwest");
    pub const KOREA_CENTRAL: Region = Region::new_const("koreacentral");
    pub const KOREA_SOUTH: Region = Region::new_const("koreasouth");
    pub const USGOV_VIRGINIA: Region = Region::new_const("usgovvirginia");
    pub const USGOV_IOWA: Region = Region::new_const("usgoviowa");
    pub const USGOV_ARIZONA: Region = Region::new_const("usgovarizona");
    pub const USGOV_TEXAS: Region = Region::new_const("usgovtexas");
    pub const USDOD_EAST: Region = Region::new_const("usdodeast");
    pub const USDOD_CENTRAL: Region = Region::new_const("usdodcentral");
    pub const USSEC_EAST: Region = Region::new_const("usseceast");
    pub const USSEC_WEST: Region = Region::new_const("ussecwest");
    pub const SOUTH_AFRICA_WEST: Region = Region::new_const("southafricawest");
    pub const SOUTH_AFRICA_NORTH: Region = Region::new_const("southafricanorth");
    pub const UAE_CENTRAL: Region = Region::new_const("uaecentral");
    pub const UAE_NORTH: Region = Region::new_const("uaenorth");
    pub const CENTRAL_US_EUAP: Region = Region::new_const("centraluseuap");
    pub const EAST_US_2_EUAP: Region = Region::new_const("eastus2euap");
    pub const NORTH_EUROPE_2: Region = Region::new_const("northeurope2");
    pub const EAST_EUROPE: Region = Region::new_const("easteurope");
    pub const APAC_SOUTHEAST_2: Region = Region::new_const("apacsoutheast2");
    pub const UK_SOUTH_2: Region = Region::new_const("uksouth2");
    pub const UK_NORTH: Region = Region::new_const("uknorth");
    pub const EAST_US_STG: Region = Region::new_const("eastusstg");
    pub const SOUTH_CENTRAL_US_STG: Region = Region::new_const("southcentralusstg");
    pub const NORWAY_EAST: Region = Region::new_const("norwayeast");
    pub const NORWAY_WEST: Region = Region::new_const("norwaywest");
    pub const USGOV_WYOMING: Region = Region::new_const("usgovwyoming");
    pub const USDOD_SOUTHWEST: Region = Region::new_const("usdodsouthwest");
    pub const USDOD_WEST_CENTRAL: Region = Region::new_const("usdodwestcentral");
    pub const USDOD_SOUTH_CENTRAL: Region = Region::new_const("usdodsouthcentral");
    pub const CHINA_EAST_2: Region = Region::new_const("chinaeast2");
    pub const CHINA_NORTH_2: Region = Region::new_const("chinanorth2");
    pub const USNAT_EAST: Region = Region::new_const("usnateast");
    pub const USNAT_WEST: Region = Region::new_const("usnatwest");
    pub const CHINA_NORTH_10: Region = Region::new_const("chinanorth10");
    pub const SWEDEN_CENTRAL: Region = Region::new_const("swedencentral");
    pub const SWEDEN_SOUTH: Region = Region::new_const("swedensouth");
    pub const KOREA_SOUTH_2: Region = Region::new_const("koreasouth2");
    pub const USSEC_WEST_CENTRAL: Region = Region::new_const("ussecwestcentral");

    /// Creates a new region from a string, normalizing the input.
    ///
    /// Normalization removes whitespace and converts to lowercase.
    /// "WESTUS2", "WestUS 2", and "West US 2" all become "westus2".
    ///
    /// # Examples
    ///
    /// ```
    /// use azure_data_cosmos_driver::options::Region;
    ///
    /// let region = Region::new("West US 2");
    /// assert_eq!(region.as_str(), "westus2");
    /// ```
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        let normalized = normalize_region_name(name.into());
        Self { normalized }
    }

    /// Gets the normalized region name (lowercase, no spaces).
    ///
    /// # Examples
    ///
    /// ```
    /// use azure_data_cosmos_driver::options::Region;
    ///
    /// let region = Region::WEST_US_2;
    /// assert_eq!(region.as_str(), "westus2");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.normalized
    }

    /// Gets the display name for the region.
    ///
    /// For known regions, returns the standard display name (e.g., "West US 2").
    /// For unknown regions, returns the normalized name.
    ///
    /// # Examples
    ///
    /// ```
    /// use azure_data_cosmos_driver::options::Region;
    ///
    /// // Known region uses standard display name
    /// let known = Region::WEST_US_2;
    /// assert_eq!(known.display_name(), "West US 2");
    ///
    /// // Unknown region returns normalized name
    /// let custom = Region::new("East US 9");
    /// assert_eq!(custom.display_name(), "eastus9");
    /// ```
    pub fn display_name(&self) -> &str {
        DISPLAY_NAME_MAPPING
            .iter()
            .find(|(normalized, _)| *normalized == self.normalized.as_ref())
            .map(|(_, display)| *display)
            .unwrap_or(&self.normalized)
    }

    /// Internal helper to create const instances.
    const fn new_const(normalized: &'static str) -> Self {
        Self {
            normalized: Cow::Borrowed(normalized),
        }
    }

    /// Gets the internal region ID used for session token tracking.
    ///
    /// Returns `None` for regions not in the known mapping table.
    ///
    /// **Note**: This is an internal implementation detail and may change.
    /// The returned ID should be treated as an opaque value.
    #[cfg(test)]
    pub(crate) fn id(&self) -> Option<u8> {
        REGION_ID_MAPPING
            .iter()
            .find(|(normalized, _)| *normalized == self.normalized.as_ref())
            .map(|(_, id)| *id)
    }
}

impl From<&'static str> for Region {
    fn from(name: &'static str) -> Self {
        Self::new(name)
    }
}

impl From<String> for Region {
    fn from(name: String) -> Self {
        Self::new(name)
    }
}

impl AsRef<str> for Region {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Normalizes a region name by removing whitespace and converting to lowercase.
fn normalize_region_name(name: impl AsRef<str>) -> Cow<'static, str> {
    let normalized: String = name
        .as_ref()
        .chars()
        .filter(|c| !c.is_whitespace())
        .flat_map(|c| c.to_lowercase())
        .collect();
    Cow::Owned(normalized)
}

// ===== Region ID Mapping =====
// Maps normalized region names to internal IDs for session token tracking.
// Source: RegionNameToRegionIdMap.java from Azure SDK for Java
#[cfg(test)]
const REGION_ID_MAPPING: &[(&str, u8)] = &[
    ("eastus", 1),
    ("eastus2", 2),
    ("centralus", 3),
    ("northcentralus", 4),
    ("southcentralus", 5),
    ("westcentralus", 6),
    ("westus", 7),
    ("westus2", 8),
    ("canadaeast", 9),
    ("canadacentral", 10),
    ("brazilsouth", 11),
    ("northeurope", 12),
    ("westeurope", 13),
    ("francecentral", 14),
    ("francesouth", 15),
    ("ukwest", 16),
    ("uksouth", 17),
    ("germanycentral", 18),
    ("germanynortheast", 19),
    ("germanynorth", 20),
    ("germanywestcentral", 21),
    ("switzerlandnorth", 22),
    ("switzerlandwest", 23),
    ("southeastasia", 24),
    ("eastasia", 25),
    ("australiaeast", 26),
    ("australiasoutheast", 27),
    ("australiacentral", 28),
    ("australiacentral2", 29),
    ("chinaeast", 30),
    ("chinanorth", 31),
    ("centralindia", 32),
    ("westindia", 33),
    ("southindia", 34),
    ("japaneast", 35),
    ("japanwest", 36),
    ("koreacentral", 37),
    ("koreasouth", 38),
    ("usgovvirginia", 39),
    ("usgoviowa", 40),
    ("usgovarizona", 41),
    ("usgovtexas", 42),
    ("usdodeast", 43),
    ("usdodcentral", 44),
    ("usseceast", 45),
    ("ussecwest", 46),
    ("southafricawest", 47),
    ("southafricanorth", 48),
    ("uaecentral", 49),
    ("uaenorth", 50),
    ("centraluseuap", 51),
    ("eastus2euap", 52),
    ("northeurope2", 53),
    ("easteurope", 54),
    ("apacsoutheast2", 55),
    ("uksouth2", 56),
    ("uknorth", 57),
    ("eastusstg", 58),
    ("southcentralusstg", 59),
    ("norwayeast", 60),
    ("norwaywest", 61),
    ("usgovwyoming", 62),
    ("usdodsouthwest", 63),
    ("usdodwestcentral", 64),
    ("usdodsouthcentral", 65),
    ("chinaeast2", 66),
    ("chinanorth2", 67),
    ("usnateast", 68),
    ("usnatwest", 69),
    ("chinanorth10", 70),
    ("swedencentral", 71),
    ("swedensouth", 72),
    ("koreasouth2", 73),
    ("ussecwestcentral", 113),
];

// ===== Display Name Mapping =====
// Maps normalized region names to their standard display names.
// Source: RegionNameToRegionIdMap.java from Azure SDK for Java
const DISPLAY_NAME_MAPPING: &[(&str, &str)] = &[
    ("eastus", "East US"),
    ("eastus2", "East US 2"),
    ("centralus", "Central US"),
    ("northcentralus", "North Central US"),
    ("southcentralus", "South Central US"),
    ("westcentralus", "West Central US"),
    ("westus", "West US"),
    ("westus2", "West US 2"),
    ("canadaeast", "Canada East"),
    ("canadacentral", "Canada Central"),
    ("brazilsouth", "Brazil South"),
    ("northeurope", "North Europe"),
    ("westeurope", "West Europe"),
    ("francecentral", "France Central"),
    ("francesouth", "France South"),
    ("ukwest", "UK West"),
    ("uksouth", "UK South"),
    ("germanycentral", "Germany Central"),
    ("germanynortheast", "Germany Northeast"),
    ("germanynorth", "Germany North"),
    ("germanywestcentral", "Germany West Central"),
    ("switzerlandnorth", "Switzerland North"),
    ("switzerlandwest", "Switzerland West"),
    ("southeastasia", "Southeast Asia"),
    ("eastasia", "East Asia"),
    ("australiaeast", "Australia East"),
    ("australiasoutheast", "Australia Southeast"),
    ("australiacentral", "Australia Central"),
    ("australiacentral2", "Australia Central 2"),
    ("chinaeast", "China East"),
    ("chinanorth", "China North"),
    ("centralindia", "Central India"),
    ("westindia", "West India"),
    ("southindia", "South India"),
    ("japaneast", "Japan East"),
    ("japanwest", "Japan West"),
    ("koreacentral", "Korea Central"),
    ("koreasouth", "Korea South"),
    ("usgovvirginia", "USGov Virginia"),
    ("usgoviowa", "USGov Iowa"),
    ("usgovarizona", "USGov Arizona"),
    ("usgovtexas", "USGov Texas"),
    ("usdodeast", "USDoD East"),
    ("usdodcentral", "USDoD Central"),
    ("usseceast", "USSec East"),
    ("ussecwest", "USSec West"),
    ("southafricawest", "South Africa West"),
    ("southafricanorth", "South Africa North"),
    ("uaecentral", "UAE Central"),
    ("uaenorth", "UAE North"),
    ("centraluseuap", "Central US EUAP"),
    ("eastus2euap", "East US 2 EUAP"),
    ("northeurope2", "North Europe 2"),
    ("easteurope", "easteurope"),
    ("apacsoutheast2", "APAC Southeast 2"),
    ("uksouth2", "UK South 2"),
    ("uknorth", "UK North"),
    ("eastusstg", "East US STG"),
    ("southcentralusstg", "South Central US STG"),
    ("norwayeast", "Norway East"),
    ("norwaywest", "Norway West"),
    ("usgovwyoming", "USGov Wyoming"),
    ("usdodsouthwest", "USDoD Southwest"),
    ("usdodwestcentral", "USDoD West Central"),
    ("usdodsouthcentral", "USDoD South Central"),
    ("chinaeast2", "China East 2"),
    ("chinanorth2", "China North 2"),
    ("usnateast", "USNat East"),
    ("usnatwest", "USNat West"),
    ("chinanorth10", "China North 10"),
    ("swedencentral", "Sweden Central"),
    ("swedensouth", "Sweden South"),
    ("koreasouth2", "Korea South 2"),
    ("ussecwestcentral", "USSec West Central"),
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalization() {
        // Test various input formats normalize to the same value
        let r1 = Region::new("WESTUS2");
        let r2 = Region::new("WestUS 2");
        let r3 = Region::new("West US 2");
        let r4 = Region::new("west us 2");

        assert_eq!(r1, r2);
        assert_eq!(r2, r3);
        assert_eq!(r3, r4);
        assert_eq!(r1.as_str(), "westus2");
    }

    #[test]
    fn known_region_constants() {
        assert_eq!(Region::WEST_US_2.as_str(), "westus2");
        assert_eq!(Region::EAST_US.as_str(), "eastus");
        assert_eq!(Region::WEST_EUROPE.as_str(), "westeurope");
    }

    #[test]
    fn display_names() {
        // Known region should have proper display name
        assert_eq!(Region::WEST_US_2.display_name(), "West US 2");
        assert_eq!(Region::EAST_US.display_name(), "East US");

        // Unknown region returns normalized name
        let custom = Region::new("East US 9");
        assert_eq!(custom.as_str(), "eastus9");
        assert_eq!(custom.display_name(), "eastus9");
    }

    #[test]
    fn region_ids() {
        assert_eq!(Region::WEST_US_2.id(), Some(8));
        assert_eq!(Region::EAST_US.id(), Some(1));
        assert_eq!(Region::USSEC_WEST_CENTRAL.id(), Some(113));

        // Unknown region should return None
        let custom = Region::new("East US 9");
        assert_eq!(custom.id(), None);
    }

    #[test]
    fn equality_and_ordering() {
        let mut regions = [Region::WEST_US_2, Region::EAST_US, Region::CENTRAL_US];
        regions.sort();

        assert_eq!(regions[0], Region::CENTRAL_US);
        assert_eq!(regions[1], Region::EAST_US);
        assert_eq!(regions[2], Region::WEST_US_2);
    }

    #[test]
    fn serialization() {
        let region = Region::WEST_US_2;
        let json = serde_json::to_string(&region).unwrap();
        assert_eq!(json, r#""westus2""#);

        let deserialized: Region = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, region);
    }
}

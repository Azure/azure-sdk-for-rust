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

    /// The `eastus` region.
    pub const EAST_US: Region = Region::new_const("eastus");
    /// The `eastus2` region.
    pub const EAST_US_2: Region = Region::new_const("eastus2");
    /// The `centralus` region.
    pub const CENTRAL_US: Region = Region::new_const("centralus");
    /// The `northcentralus` region.
    pub const NORTH_CENTRAL_US: Region = Region::new_const("northcentralus");
    /// The `southcentralus` region.
    pub const SOUTH_CENTRAL_US: Region = Region::new_const("southcentralus");
    /// The `westcentralus` region.
    pub const WEST_CENTRAL_US: Region = Region::new_const("westcentralus");
    /// The `westus` region.
    pub const WEST_US: Region = Region::new_const("westus");
    /// The `westus2` region.
    pub const WEST_US_2: Region = Region::new_const("westus2");
    /// The `canadaeast` region.
    pub const CANADA_EAST: Region = Region::new_const("canadaeast");
    /// The `canadacentral` region.
    pub const CANADA_CENTRAL: Region = Region::new_const("canadacentral");
    /// The `brazilsouth` region.
    pub const BRAZIL_SOUTH: Region = Region::new_const("brazilsouth");
    /// The `northeurope` region.
    pub const NORTH_EUROPE: Region = Region::new_const("northeurope");
    /// The `westeurope` region.
    pub const WEST_EUROPE: Region = Region::new_const("westeurope");
    /// The `francecentral` region.
    pub const FRANCE_CENTRAL: Region = Region::new_const("francecentral");
    /// The `francesouth` region.
    pub const FRANCE_SOUTH: Region = Region::new_const("francesouth");
    /// The `ukwest` region.
    pub const UK_WEST: Region = Region::new_const("ukwest");
    /// The `uksouth` region.
    pub const UK_SOUTH: Region = Region::new_const("uksouth");
    /// The `germanycentral` region.
    pub const GERMANY_CENTRAL: Region = Region::new_const("germanycentral");
    /// The `germanynortheast` region.
    pub const GERMANY_NORTHEAST: Region = Region::new_const("germanynortheast");
    /// The `germanynorth` region.
    pub const GERMANY_NORTH: Region = Region::new_const("germanynorth");
    /// The `germanywestcentral` region.
    pub const GERMANY_WEST_CENTRAL: Region = Region::new_const("germanywestcentral");
    /// The `switzerlandnorth` region.
    pub const SWITZERLAND_NORTH: Region = Region::new_const("switzerlandnorth");
    /// The `switzerlandwest` region.
    pub const SWITZERLAND_WEST: Region = Region::new_const("switzerlandwest");
    /// The `southeastasia` region.
    pub const SOUTHEAST_ASIA: Region = Region::new_const("southeastasia");
    /// The `eastasia` region.
    pub const EAST_ASIA: Region = Region::new_const("eastasia");
    /// The `australiaeast` region.
    pub const AUSTRALIA_EAST: Region = Region::new_const("australiaeast");
    /// The `australiasoutheast` region.
    pub const AUSTRALIA_SOUTHEAST: Region = Region::new_const("australiasoutheast");
    /// The `australiacentral` region.
    pub const AUSTRALIA_CENTRAL: Region = Region::new_const("australiacentral");
    /// The `australiacentral2` region.
    pub const AUSTRALIA_CENTRAL_2: Region = Region::new_const("australiacentral2");
    /// The `chinaeast` region.
    pub const CHINA_EAST: Region = Region::new_const("chinaeast");
    /// The `chinanorth` region.
    pub const CHINA_NORTH: Region = Region::new_const("chinanorth");
    /// The `centralindia` region.
    pub const CENTRAL_INDIA: Region = Region::new_const("centralindia");
    /// The `westindia` region.
    pub const WEST_INDIA: Region = Region::new_const("westindia");
    /// The `southindia` region.
    pub const SOUTH_INDIA: Region = Region::new_const("southindia");
    /// The `japaneast` region.
    pub const JAPAN_EAST: Region = Region::new_const("japaneast");
    /// The `japanwest` region.
    pub const JAPAN_WEST: Region = Region::new_const("japanwest");
    /// The `koreacentral` region.
    pub const KOREA_CENTRAL: Region = Region::new_const("koreacentral");
    /// The `koreasouth` region.
    pub const KOREA_SOUTH: Region = Region::new_const("koreasouth");
    /// The `usgovvirginia` region.
    pub const USGOV_VIRGINIA: Region = Region::new_const("usgovvirginia");
    /// The `usgoviowa` region.
    pub const USGOV_IOWA: Region = Region::new_const("usgoviowa");
    /// The `usgovarizona` region.
    pub const USGOV_ARIZONA: Region = Region::new_const("usgovarizona");
    /// The `usgovtexas` region.
    pub const USGOV_TEXAS: Region = Region::new_const("usgovtexas");
    /// The `usdodeast` region.
    pub const USDOD_EAST: Region = Region::new_const("usdodeast");
    /// The `usdodcentral` region.
    pub const USDOD_CENTRAL: Region = Region::new_const("usdodcentral");
    /// The `usseceast` region.
    pub const USSEC_EAST: Region = Region::new_const("usseceast");
    /// The `ussecwest` region.
    pub const USSEC_WEST: Region = Region::new_const("ussecwest");
    /// The `southafricawest` region.
    pub const SOUTH_AFRICA_WEST: Region = Region::new_const("southafricawest");
    /// The `southafricanorth` region.
    pub const SOUTH_AFRICA_NORTH: Region = Region::new_const("southafricanorth");
    /// The `uaecentral` region.
    pub const UAE_CENTRAL: Region = Region::new_const("uaecentral");
    /// The `uaenorth` region.
    pub const UAE_NORTH: Region = Region::new_const("uaenorth");
    /// The `centraluseuap` region.
    pub const CENTRAL_US_EUAP: Region = Region::new_const("centraluseuap");
    /// The `eastus2euap` region.
    pub const EAST_US_2_EUAP: Region = Region::new_const("eastus2euap");
    /// The `northeurope2` region.
    pub const NORTH_EUROPE_2: Region = Region::new_const("northeurope2");
    /// The `easteurope` region.
    pub const EAST_EUROPE: Region = Region::new_const("easteurope");
    /// The `apacsoutheast2` region.
    pub const APAC_SOUTHEAST_2: Region = Region::new_const("apacsoutheast2");
    /// The `uksouth2` region.
    pub const UK_SOUTH_2: Region = Region::new_const("uksouth2");
    /// The `uknorth` region.
    pub const UK_NORTH: Region = Region::new_const("uknorth");
    /// The `eastusstg` region.
    pub const EAST_US_STG: Region = Region::new_const("eastusstg");
    /// The `southcentralusstg` region.
    pub const SOUTH_CENTRAL_US_STG: Region = Region::new_const("southcentralusstg");
    /// The `norwayeast` region.
    pub const NORWAY_EAST: Region = Region::new_const("norwayeast");
    /// The `norwaywest` region.
    pub const NORWAY_WEST: Region = Region::new_const("norwaywest");
    /// The `usgovwyoming` region.
    pub const USGOV_WYOMING: Region = Region::new_const("usgovwyoming");
    /// The `usdodsouthwest` region.
    pub const USDOD_SOUTHWEST: Region = Region::new_const("usdodsouthwest");
    /// The `usdodwestcentral` region.
    pub const USDOD_WEST_CENTRAL: Region = Region::new_const("usdodwestcentral");
    /// The `usdodsouthcentral` region.
    pub const USDOD_SOUTH_CENTRAL: Region = Region::new_const("usdodsouthcentral");
    /// The `chinaeast2` region.
    pub const CHINA_EAST_2: Region = Region::new_const("chinaeast2");
    /// The `chinanorth2` region.
    pub const CHINA_NORTH_2: Region = Region::new_const("chinanorth2");
    /// The `usnateast` region.
    pub const USNAT_EAST: Region = Region::new_const("usnateast");
    /// The `usnatwest` region.
    pub const USNAT_WEST: Region = Region::new_const("usnatwest");
    /// The `chinanorth10` region.
    pub const CHINA_NORTH_10: Region = Region::new_const("chinanorth10");
    /// The `swedencentral` region.
    pub const SWEDEN_CENTRAL: Region = Region::new_const("swedencentral");
    /// The `swedensouth` region.
    pub const SWEDEN_SOUTH: Region = Region::new_const("swedensouth");
    /// The `koreasouth2` region.
    pub const KOREA_SOUTH_2: Region = Region::new_const("koreasouth2");
    /// The `ussecwestcentral` region.
    pub const USSEC_WEST_CENTRAL: Region = Region::new_const("ussecwestcentral");
    /// The `austriaeast` region.
    pub const AUSTRIA_EAST: Region = Region::new_const("austriaeast");
    /// The `belgiumcentral` region.
    pub const BELGIUM_CENTRAL: Region = Region::new_const("belgiumcentral");
    /// The `bleufrancecentral` region.
    pub const BLEU_FRANCE_CENTRAL: Region = Region::new_const("bleufrancecentral");
    /// The `bleufrancesouth` region.
    pub const BLEU_FRANCE_SOUTH: Region = Region::new_const("bleufrancesouth");
    /// The `brazilsoutheast` region.
    pub const BRAZIL_SOUTHEAST: Region = Region::new_const("brazilsoutheast");
    /// The `chilecentral` region.
    pub const CHILE_CENTRAL: Region = Region::new_const("chilecentral");
    /// The `chinaeast3` region.
    pub const CHINA_EAST_3: Region = Region::new_const("chinaeast3");
    /// The `chinanorth3` region.
    pub const CHINA_NORTH_3: Region = Region::new_const("chinanorth3");
    /// The `deloscloudgermanycentral` region.
    pub const DELOS_CLOUD_GERMANY_CENTRAL: Region = Region::new_const("deloscloudgermanycentral");
    /// The `deloscloudgermanynorth` region.
    pub const DELOS_CLOUD_GERMANY_NORTH: Region = Region::new_const("deloscloudgermanynorth");
    /// The `denmarkeast` region.
    pub const DENMARK_EAST: Region = Region::new_const("denmarkeast");
    /// The `eastus3` region.
    pub const EAST_US_3: Region = Region::new_const("eastus3");
    /// The `eastusslv` region.
    pub const EAST_US_SLV: Region = Region::new_const("eastusslv");
    /// The `indiasouthcentral` region.
    pub const INDIA_SOUTH_CENTRAL: Region = Region::new_const("indiasouthcentral");
    /// The `indonesiacentral` region.
    pub const INDONESIA_CENTRAL: Region = Region::new_const("indonesiacentral");
    /// The `israelcentral` region.
    pub const ISRAEL_CENTRAL: Region = Region::new_const("israelcentral");
    /// The `israelnorthwest` region.
    pub const ISRAEL_NORTHWEST: Region = Region::new_const("israelnorthwest");
    /// The `italynorth` region.
    pub const ITALY_NORTH: Region = Region::new_const("italynorth");
    /// The `jioindiacentral` region.
    pub const JIO_INDIA_CENTRAL: Region = Region::new_const("jioindiacentral");
    /// The `jioindiawest` region.
    pub const JIO_INDIA_WEST: Region = Region::new_const("jioindiawest");
    /// The `malaysiasouth` region.
    pub const MALAYSIA_SOUTH: Region = Region::new_const("malaysiasouth");
    /// The `malaysiawest` region.
    pub const MALAYSIA_WEST: Region = Region::new_const("malaysiawest");
    /// The `mexicocentral` region.
    pub const MEXICO_CENTRAL: Region = Region::new_const("mexicocentral");
    /// The `newzealandnorth` region.
    pub const NEW_ZEALAND_NORTH: Region = Region::new_const("newzealandnorth");
    /// The `northeastus5` region.
    pub const NORTHEAST_US_5: Region = Region::new_const("northeastus5");
    /// The `polandcentral` region.
    pub const POLAND_CENTRAL: Region = Region::new_const("polandcentral");
    /// The `qatarcentral` region.
    pub const QATAR_CENTRAL: Region = Region::new_const("qatarcentral");
    /// The `singaporecentral` region.
    pub const SINGAPORE_CENTRAL: Region = Region::new_const("singaporecentral");
    /// The `singaporenorth` region.
    pub const SINGAPORE_NORTH: Region = Region::new_const("singaporenorth");
    /// The `southcentralus2` region.
    pub const SOUTH_CENTRAL_US_2: Region = Region::new_const("southcentralus2");
    /// The `southeastus` region.
    pub const SOUTHEAST_US: Region = Region::new_const("southeastus");
    /// The `southeastus3` region.
    pub const SOUTHEAST_US_3: Region = Region::new_const("southeastus3");
    /// The `southeastus5` region.
    pub const SOUTHEAST_US_5: Region = Region::new_const("southeastus5");
    /// The `southwestus` region.
    pub const SOUTHWEST_US: Region = Region::new_const("southwestus");
    /// The `spaincentral` region.
    pub const SPAIN_CENTRAL: Region = Region::new_const("spaincentral");
    /// The `taiwannorth` region.
    pub const TAIWAN_NORTH: Region = Region::new_const("taiwannorth");
    /// The `taiwannorthwest` region.
    pub const TAIWAN_NORTHWEST: Region = Region::new_const("taiwannorthwest");
    /// The `westus3` region.
    pub const WEST_US_3: Region = Region::new_const("westus3");

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
    ("austriaeast", "Austria East"),
    ("belgiumcentral", "Belgium Central"),
    ("bleufrancecentral", "Bleu France Central"),
    ("bleufrancesouth", "Bleu France South"),
    ("brazilsoutheast", "Brazil Southeast"),
    ("chilecentral", "Chile Central"),
    ("chinaeast3", "China East 3"),
    ("chinanorth3", "China North 3"),
    ("deloscloudgermanycentral", "Delos Cloud Germany Central"),
    ("deloscloudgermanynorth", "Delos Cloud Germany North"),
    ("denmarkeast", "Denmark East"),
    ("eastus3", "East US 3"),
    ("eastusslv", "East US SLV"),
    ("indiasouthcentral", "India South Central"),
    ("indonesiacentral", "Indonesia Central"),
    ("israelcentral", "Israel Central"),
    ("israelnorthwest", "Israel Northwest"),
    ("italynorth", "Italy North"),
    ("jioindiacentral", "Jio India Central"),
    ("jioindiawest", "Jio India West"),
    ("malaysiasouth", "Malaysia South"),
    ("malaysiawest", "Malaysia West"),
    ("mexicocentral", "Mexico Central"),
    ("newzealandnorth", "New Zealand North"),
    ("northeastus5", "Northeast US 5"),
    ("polandcentral", "Poland Central"),
    ("qatarcentral", "Qatar Central"),
    ("singaporecentral", "Singapore Central"),
    ("singaporenorth", "Singapore North"),
    ("southcentralus2", "South Central US 2"),
    ("southeastus", "Southeast US"),
    ("southeastus3", "Southeast US 3"),
    ("southeastus5", "Southeast US 5"),
    ("southwestus", "Southwest US"),
    ("spaincentral", "Spain Central"),
    ("taiwannorth", "Taiwan North"),
    ("taiwannorthwest", "Taiwan Northwest"),
    ("westus3", "West US 3"),
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

// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::models::PartitionKeyKind;
use azure_data_cosmos_driver::models::{
    effective_partition_key::EffectivePartitionKey as DriverEffectivePartitionKey,
    PartitionKeyValue as DriverPartitionKeyValue, PartitionKeyVersion,
};
use std::fmt;

/// A strongly-typed wrapper around the hex-encoded effective partition key string.
///
/// This SDK type wraps the driver's canonical EPK implementation while keeping
/// the SDK's public API surface explicit and stable.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EffectivePartitionKey(DriverEffectivePartitionKey);

impl EffectivePartitionKey {
    /// Returns the underlying string representation.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Display for EffectivePartitionKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<str> for EffectivePartitionKey {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<String> for EffectivePartitionKey {
    fn from(value: String) -> Self {
        Self(DriverEffectivePartitionKey::from(value))
    }
}

impl From<&str> for EffectivePartitionKey {
    fn from(value: &str) -> Self {
        Self(DriverEffectivePartitionKey::from(value))
    }
}

/// Returns an [`EffectivePartitionKey`] representing the hashed partition key.
///
/// Versions 1 and 2 map directly to the driver's partition key version enum.
/// Any other version falls back to V2 for forward-compatible behavior.
#[allow(dead_code)] // Currently exercised only by tests; kept for upcoming SDK API.
pub fn get_hashed_partition_key_string(
    pk_value: &[DriverPartitionKeyValue],
    kind: PartitionKeyKind,
    version: u8,
) -> EffectivePartitionKey {
    if pk_value.is_empty() {
        return EffectivePartitionKey(DriverEffectivePartitionKey::min());
    }

    let version = match version {
        1 => PartitionKeyVersion::V1,
        2 => PartitionKeyVersion::V2,
        unsupported => {
            tracing::warn!(
                "Partition key hashing version {} is unsupported in SDK API; defaulting to V2",
                unsupported
            );
            PartitionKeyVersion::V2
        }
    };

    EffectivePartitionKey(DriverEffectivePartitionKey::compute(
        pk_value, kind, version,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_pk_returns_min() {
        let result = get_hashed_partition_key_string(&[], PartitionKeyKind::Hash, 0);
        assert_eq!(result.as_str(), "");
    }

    #[test]
    fn single_string_hash_v2_matches_baseline() {
        let comp = DriverPartitionKeyValue::from("customer42".to_string());
        let result = get_hashed_partition_key_string(&[comp], PartitionKeyKind::Hash, 2);
        assert_eq!(result.as_str(), "19819C94CE42A1654CCC8110539D9589");
    }

    #[test]
    fn effective_partition_key_hash_v2_examples() {
        let cases: Vec<(DriverPartitionKeyValue, &str)> = vec![
            (
                DriverPartitionKeyValue::from(String::from("")),
                "32E9366E637A71B4E710384B2F4970A0",
            ),
            (
                DriverPartitionKeyValue::from(String::from("partitionKey")),
                "013AEFCF77FA271571CF665A58C933F1",
            ),
            (
                DriverPartitionKeyValue::from(5.0),
                "19C08621B135968252FB34B4CF66F811",
            ),
            (
                DriverPartitionKeyValue::from(String::from("redmond")),
                "22E342F38A486A088463DFF7838A5963",
            ),
        ];

        for (component, expected) in &cases {
            let actual = get_hashed_partition_key_string(
                std::slice::from_ref(component),
                PartitionKeyKind::Hash,
                2,
            );
            assert_eq!(actual.as_str(), *expected, "Mismatch for V2 component hash");
        }
    }

    #[test]
    fn effective_partition_key_hash_v1_examples() {
        let cases: Vec<(DriverPartitionKeyValue, &str)> = vec![
            (
                DriverPartitionKeyValue::from(String::from("")),
                "05C1CF33970FF80800",
            ),
            (
                DriverPartitionKeyValue::from(String::from("partitionKey")),
                "05C1E1B3D9CD2608716273756A756A706F4C667A00",
            ),
            (DriverPartitionKeyValue::NULL, "05C1ED45D7475601"),
            (DriverPartitionKeyValue::from(true), "05C1D7C5A903D803"),
        ];

        for (component, expected) in &cases {
            let actual = get_hashed_partition_key_string(
                std::slice::from_ref(component),
                PartitionKeyKind::Hash,
                1,
            );
            assert_eq!(actual.as_str(), *expected, "Mismatch for V1 component hash");
        }
    }
}

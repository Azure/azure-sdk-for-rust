// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use serde::{Deserialize, Serialize};
use std::num::NonZeroU16;
use uuid::Uuid;

/// Reserved item property used to persist PATCH tracking entries.
pub const PATCH_TRACKING_PROPERTY: &str = "_azsdkPatchTracking";

/// Time PATCH tracking entries remain protected from age-based pruning.
///
/// A matching entry is honored for as long as it remains on the item, but a
/// later PATCH may prune it after this interval has elapsed or evict it earlier
/// when the marker array reaches capacity.
pub const PATCH_TRACKING_RETENTION: std::time::Duration = std::time::Duration::from_secs(5 * 60);

/// Default maximum number of PATCH tracking entries retained on one item.
/// The oldest entry is evicted when this capacity is reached.
pub const DEFAULT_PATCH_TRACKING_CAPACITY: NonZeroU16 =
    NonZeroU16::new(1024).expect("default PATCH tracking capacity is non-zero");

/// Stable identity for an unsafe PATCH operation.
///
/// Persist and reuse this value when retrying the same logical operation. A
/// newly generated ID identifies a new operation and may apply it again.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PatchTrackingId(Uuid);

impl PatchTrackingId {
    /// Generates a new random, unpredictable tracking ID.
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Returns the underlying UUID.
    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    pub(crate) fn into_driver(self) -> azure_data_cosmos_driver::models::PatchTrackingId {
        self.0.into()
    }

    pub(crate) fn from_driver(value: azure_data_cosmos_driver::models::PatchTrackingId) -> Self {
        Self(value.as_uuid())
    }
}

impl Default for PatchTrackingId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for PatchTrackingId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl std::fmt::Display for PatchTrackingId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(formatter)
    }
}

impl std::str::FromStr for PatchTrackingId {
    type Err = uuid::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        value.parse::<Uuid>().map(Self)
    }
}

impl Serialize for PatchTrackingId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for PatchTrackingId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = String::deserialize(deserializer)?;
        value.parse().map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_round_trip_uses_uuid_text() {
        let text = "7f5241c9-d7c2-4071-97a3-43bdebf6ef8f";
        let tracking_id = text.parse::<PatchTrackingId>().unwrap();

        assert_eq!(
            serde_json::to_string(&tracking_id).unwrap(),
            format!("\"{text}\"")
        );
        assert_eq!(
            serde_json::from_str::<PatchTrackingId>(&format!("\"{text}\""))
                .expect("valid UUID text must deserialize"),
            tracking_id
        );
    }

    #[test]
    fn protocol_constants_match_driver() {
        assert_eq!(
            PATCH_TRACKING_PROPERTY,
            azure_data_cosmos_driver::models::PATCH_TRACKING_PROPERTY
        );
        assert_eq!(
            PATCH_TRACKING_RETENTION,
            azure_data_cosmos_driver::models::PATCH_TRACKING_RETENTION
        );
        assert_eq!(
            DEFAULT_PATCH_TRACKING_CAPACITY,
            azure_data_cosmos_driver::models::DEFAULT_PATCH_TRACKING_CAPACITY
        );
    }
}

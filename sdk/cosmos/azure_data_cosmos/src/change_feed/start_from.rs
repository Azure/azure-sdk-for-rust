// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::time::OffsetDateTime;
use serde::{Deserialize, Serialize};

/// Specifies where to start reading the change feed.
///
/// The change feed is an ordered log of changes. Use this enum to control
/// the starting point for reading changes.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ChangeFeedStartFrom {
    /// Start reading from the beginning of the change feed.
    ///
    /// This reads all available changes from the earliest point in the change feed retention period.
    #[default]
    Beginning,

    /// Start reading from the current point in time.
    ///
    /// Only changes that occur after this point will be returned.
    Now,

    /// Start reading from a specific point in time.
    ///
    /// Changes that occurred at or after this timestamp will be returned.
    /// The timestamp should be in UTC.
    PointInTime(OffsetDateTime),
}

/// Internal representation for serialization in continuation tokens.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ChangeFeedStartFromInternal {
    #[serde(rename = "Type")]
    pub start_type: ChangeFeedStartFromType,

    #[serde(rename = "PointInTimeMs", skip_serializing_if = "Option::is_none")]
    pub point_in_time_ms: Option<i64>,

    #[serde(rename = "Etag", skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,

    #[serde(rename = "FeedRange", skip_serializing_if = "Option::is_none")]
    pub feed_range: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum ChangeFeedStartFromType {
    Beginning,
    Now,
    PointInTime,
    Lease,
}

impl ChangeFeedStartFromInternal {
    pub fn from_public(start_from: &ChangeFeedStartFrom) -> Self {
        match start_from {
            ChangeFeedStartFrom::Beginning => Self {
                start_type: ChangeFeedStartFromType::Beginning,
                point_in_time_ms: None,
                etag: None,
                feed_range: None,
            },
            ChangeFeedStartFrom::Now => Self {
                start_type: ChangeFeedStartFromType::Now,
                point_in_time_ms: None,
                etag: None,
                feed_range: None,
            },
            ChangeFeedStartFrom::PointInTime(dt) => Self {
                start_type: ChangeFeedStartFromType::PointInTime,
                point_in_time_ms: Some(dt.unix_timestamp() * 1000),
                etag: None,
                feed_range: None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_beginning() {
        assert_eq!(
            ChangeFeedStartFrom::default(),
            ChangeFeedStartFrom::Beginning
        );
    }

    #[test]
    fn internal_from_beginning() {
        let internal = ChangeFeedStartFromInternal::from_public(&ChangeFeedStartFrom::Beginning);
        assert_eq!(internal.start_type, ChangeFeedStartFromType::Beginning);
        assert!(internal.point_in_time_ms.is_none());
    }

    #[test]
    fn internal_from_now() {
        let internal = ChangeFeedStartFromInternal::from_public(&ChangeFeedStartFrom::Now);
        assert_eq!(internal.start_type, ChangeFeedStartFromType::Now);
        assert!(internal.point_in_time_ms.is_none());
    }

    #[test]
    fn internal_from_point_in_time() {
        // 2024-01-15 12:00:00 UTC = 1705320000 seconds since Unix epoch
        let dt = OffsetDateTime::from_unix_timestamp(1705320000).unwrap();
        let internal =
            ChangeFeedStartFromInternal::from_public(&ChangeFeedStartFrom::PointInTime(dt));
        assert_eq!(internal.start_type, ChangeFeedStartFromType::PointInTime);
        assert!(internal.point_in_time_ms.is_some());
        // 2024-01-15 12:00:00 UTC in milliseconds
        assert_eq!(internal.point_in_time_ms.unwrap(), 1705320000000);
    }

    #[test]
    fn internal_serialization() {
        let internal = ChangeFeedStartFromInternal::from_public(&ChangeFeedStartFrom::Beginning);
        let json = serde_json::to_string(&internal).unwrap();
        assert!(json.contains("\"Type\":\"Beginning\""));
    }
}

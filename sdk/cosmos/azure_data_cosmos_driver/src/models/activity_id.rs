// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Activity ID for Cosmos DB request correlation.

use std::borrow::Cow;

use serde::{Deserialize, Serialize};

/// A unique identifier for tracking and correlating Cosmos DB operations.
///
/// Activity IDs are used by the Cosmos DB service for request tracing and
/// are essential for troubleshooting. They appear in request and response
/// headers as `x-ms-activity-id`.
///
/// # Creating Activity IDs
///
/// Use [`ActivityId::new_uuid`] to generate a new UUID-based activity ID:
///
/// ```rust
/// use azure_data_cosmos_driver::models::ActivityId;
///
/// let activity_id = ActivityId::new_uuid();
/// println!("Activity ID: {}", activity_id);
/// ```
///
/// Or parse an existing string:
///
/// ```rust
/// use azure_data_cosmos_driver::models::ActivityId;
///
/// let activity_id: ActivityId = "550e8400-e29b-41d4-a716-446655440000".parse().unwrap();
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActivityId(Cow<'static, str>);

impl ActivityId {
    /// Creates a new activity ID with a generated UUID.
    ///
    /// # Example
    ///
    /// ```rust
    /// use azure_data_cosmos_driver::models::ActivityId;
    ///
    /// let id = ActivityId::new_uuid();
    /// assert!(!id.as_str().is_empty());
    /// ```
    pub fn new_uuid() -> Self {
        Self(Cow::Owned(uuid::Uuid::new_v4().to_string()))
    }

    /// Creates an activity ID from an existing string.
    ///
    /// This does not validate the format - it accepts any string value.
    /// Use this when receiving activity IDs from external sources.
    ///
    /// # Example
    ///
    /// ```rust
    /// use azure_data_cosmos_driver::models::ActivityId;
    ///
    /// let id = ActivityId::from_string("my-activity-123".to_string());
    /// assert_eq!(id.as_str(), "my-activity-123");
    /// ```
    pub fn from_string(value: String) -> Self {
        Self(Cow::Owned(value))
    }

    /// Creates an activity ID from a static string.
    ///
    /// This is useful for compile-time known activity IDs in tests.
    ///
    /// # Example
    ///
    /// ```rust
    /// use azure_data_cosmos_driver::models::ActivityId;
    ///
    /// const TEST_ID: ActivityId = ActivityId::from_static("test-activity-id");
    /// assert_eq!(TEST_ID.as_str(), "test-activity-id");
    /// ```
    pub const fn from_static(value: &'static str) -> Self {
        Self(Cow::Borrowed(value))
    }

    /// Returns the activity ID as a string slice.
    ///
    /// # Example
    ///
    /// ```rust
    /// use azure_data_cosmos_driver::models::ActivityId;
    ///
    /// let id = ActivityId::new_uuid();
    /// println!("ID: {}", id.as_str());
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ActivityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::str::FromStr for ActivityId {
    type Err = std::convert::Infallible;

    /// Parses a string into an ActivityId.
    ///
    /// This never fails - any string is accepted.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Cow::Owned(s.to_owned())))
    }
}

impl From<String> for ActivityId {
    fn from(value: String) -> Self {
        Self(Cow::Owned(value))
    }
}

impl From<&str> for ActivityId {
    fn from(value: &str) -> Self {
        Self(Cow::Owned(value.to_owned()))
    }
}

impl AsRef<str> for ActivityId {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_uuid_generates_valid_uuid() {
        let id = ActivityId::new_uuid();
        // UUID v4 format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx
        let uuid_str = id.as_str();
        assert_eq!(uuid_str.len(), 36);
        assert!(uuid::Uuid::parse_str(uuid_str).is_ok());
    }

    #[test]
    fn from_string_preserves_value() {
        let id = ActivityId::from_string("test-123".to_string());
        assert_eq!(id.as_str(), "test-123");
    }

    #[test]
    fn from_static_is_const() {
        const ID: ActivityId = ActivityId::from_static("const-id");
        assert_eq!(ID.as_str(), "const-id");
    }

    #[test]
    fn parse_from_str() {
        let id: ActivityId = "parsed-id".parse().unwrap();
        assert_eq!(id.as_str(), "parsed-id");
    }

    #[test]
    fn display_trait() {
        let id = ActivityId::from_string("display-test".to_string());
        assert_eq!(format!("{}", id), "display-test");
    }

    #[test]
    fn equality() {
        let id1 = ActivityId::from_string("same".to_string());
        let id2 = ActivityId::from_string("same".to_string());
        let id3 = ActivityId::from_string("different".to_string());

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn serialization() {
        let id = ActivityId::from_string("serialize-test".to_string());
        let json = serde_json::to_string(&id).unwrap();
        assert_eq!(json, "\"serialize-test\"");

        let deserialized: ActivityId = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, id);
    }
}

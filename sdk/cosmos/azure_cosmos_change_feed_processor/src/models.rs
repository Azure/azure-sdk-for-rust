// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Rust-facing event models. These are independent of the typed Cosmos SDK.

use std::time::Duration;

use azure_core::fmt::SafeDebug;
use serde::{
    de::{DeserializeOwned, Error as _},
    Deserialize, Deserializer,
};
use serde_json::Value;

/// A partition-local logical sequence number; it is not a global ordering key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize)]
#[serde(transparent)]
pub struct LogicalSequenceNumber(i64);

impl LogicalSequenceNumber {
    /// Returns the service value.
    pub fn value(&self) -> i64 {
        self.0
    }
}
impl From<i64> for LogicalSequenceNumber {
    fn from(value: i64) -> Self {
        Self(value)
    }
}
impl From<LogicalSequenceNumber> for i64 {
    fn from(value: LogicalSequenceNumber) -> Self {
        value.0
    }
}

/// The operation reported by full-fidelity change metadata.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[non_exhaustive]
pub enum ChangeFeedOperationType {
    /// A new item.
    Create,
    /// A replaced item.
    Replace,
    /// A deleted item.
    Delete,
    /// A service operation not recognized by this version.
    #[serde(other)]
    Unknown,
}

fn timestamp<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Duration>, D::Error> {
    let seconds = Option::<i64>::deserialize(deserializer)?;
    Ok(seconds.map(|seconds| Duration::from_secs(seconds.max(0) as u64)))
}

/// Optional metadata supplied for a change.
///
/// LatestVersion may provide only LSN/timestamp metadata. Deleted-item identity
/// is available here even when no document image is present.
#[derive(Clone, Default, SafeDebug, Deserialize)]
pub struct ChangeFeedMetadata {
    #[serde(rename = "operationType", default)]
    operation_type: Option<ChangeFeedOperationType>,
    #[serde(default)]
    lsn: Option<LogicalSequenceNumber>,
    #[serde(rename = "crts", default, deserialize_with = "timestamp")]
    conflict_resolution_timestamp: Option<Duration>,
    #[serde(rename = "previousImageLSN", default)]
    previous_image_lsn: Option<LogicalSequenceNumber>,
    #[serde(rename = "timeToLiveExpired", default)]
    time_to_live_expired: Option<bool>,
    #[serde(default)]
    id: Option<String>,
    #[serde(rename = "partitionKey", default)]
    partition_key: Option<Value>,
}

impl ChangeFeedMetadata {
    /// Returns the reported operation, absent for partial latest-version metadata.
    pub fn operation_type(&self) -> Option<ChangeFeedOperationType> {
        self.operation_type
    }
    /// Returns the partition-local LSN.
    pub fn lsn(&self) -> Option<LogicalSequenceNumber> {
        self.lsn
    }
    /// Returns conflict resolution time since the Unix epoch.
    pub fn conflict_resolution_timestamp(&self) -> Option<Duration> {
        self.conflict_resolution_timestamp
    }
    /// Returns the previous image's LSN, if available.
    pub fn previous_image_lsn(&self) -> Option<LogicalSequenceNumber> {
        self.previous_image_lsn
    }
    /// Returns whether TTL expiration caused a delete, when reported.
    pub fn time_to_live_expired(&self) -> Option<bool> {
        self.time_to_live_expired
    }
    /// Returns the deleted item's ID, when reported.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
    /// Returns the deleted item's wire partition key.
    pub fn partition_key(&self) -> Option<&Value> {
        self.partition_key.as_ref()
    }
    /// Sets the operation.
    pub fn with_operation_type(mut self, value: ChangeFeedOperationType) -> Self {
        self.operation_type = Some(value);
        self
    }
    /// Sets the LSN.
    pub fn with_lsn(mut self, value: LogicalSequenceNumber) -> Self {
        self.lsn = Some(value);
        self
    }
    /// Sets the conflict resolution timestamp.
    pub fn with_conflict_resolution_timestamp(mut self, value: Duration) -> Self {
        self.conflict_resolution_timestamp = Some(value);
        self
    }
    /// Sets the previous image's LSN.
    pub fn with_previous_image_lsn(mut self, value: LogicalSequenceNumber) -> Self {
        self.previous_image_lsn = Some(value);
        self
    }
    /// Sets the TTL-delete flag.
    pub fn with_time_to_live_expired(mut self, value: bool) -> Self {
        self.time_to_live_expired = Some(value);
        self
    }
    /// Sets the deleted-item ID.
    pub fn with_id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }
    /// Sets the deleted-item partition key.
    pub fn with_partition_key(mut self, value: Value) -> Self {
        self.partition_key = Some(value);
        self
    }
}

/// A complete modeled change, including available document images and metadata.
///
/// Deletes may have no current image, or a minimal image your type must tolerate.
/// Previous images exist only when retained by the service. Legacy flat responses
/// become `current`; objects containing only the reserved envelope names remain
/// ambiguous. These types are not interchangeable with the Cosmos SDK's models.
#[derive(Clone, SafeDebug)]
pub struct ChangeFeedItem<T> {
    current: Option<T>,
    previous: Option<T>,
    metadata: Option<ChangeFeedMetadata>,
}

impl<T> Default for ChangeFeedItem<T> {
    fn default() -> Self {
        Self {
            current: None,
            previous: None,
            metadata: None,
        }
    }
}

impl<T> ChangeFeedItem<T> {
    /// Returns the post-change image, if present.
    pub fn current(&self) -> Option<&T> {
        self.current.as_ref()
    }
    /// Returns a retained pre-change image, if available.
    pub fn previous(&self) -> Option<&T> {
        self.previous.as_ref()
    }
    /// Returns partial or full change metadata, if present.
    pub fn metadata(&self) -> Option<&ChangeFeedMetadata> {
        self.metadata.as_ref()
    }
    /// Returns the reported operation.
    pub fn operation_type(&self) -> Option<ChangeFeedOperationType> {
        self.metadata
            .as_ref()
            .and_then(ChangeFeedMetadata::operation_type)
    }
    /// Sets the post-change image.
    pub fn with_current(mut self, value: T) -> Self {
        self.current = Some(value);
        self
    }
    /// Sets the pre-change image.
    pub fn with_previous(mut self, value: T) -> Self {
        self.previous = Some(value);
        self
    }
    /// Sets the metadata.
    pub fn with_metadata(mut self, value: ChangeFeedMetadata) -> Self {
        self.metadata = Some(value);
        self
    }
}

impl<'de, T: DeserializeOwned> Deserialize<'de> for ChangeFeedItem<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        let envelope = value.as_object().is_some_and(|fields| {
            !fields.is_empty()
                && fields
                    .keys()
                    .all(|key| matches!(key.as_str(), "current" | "previous" | "metadata"))
        });
        if !envelope {
            return Ok(Self::default()
                .with_current(serde_json::from_value(value).map_err(D::Error::custom)?));
        }
        #[derive(Deserialize)]
        struct Envelope {
            current: Option<Value>,
            previous: Option<Value>,
            metadata: Option<ChangeFeedMetadata>,
        }
        fn image<T: DeserializeOwned>(
            value: Option<Value>,
        ) -> Result<Option<T>, serde_json::Error> {
            match value {
                None | Some(Value::Null) => Ok(None),
                Some(Value::Object(fields)) if fields.is_empty() => Ok(None),
                Some(value) => serde_json::from_value(value).map(Some),
            }
        }
        let fields: Envelope = serde_json::from_value(value).map_err(D::Error::custom)?;
        Ok(Self {
            current: image(fields.current).map_err(D::Error::custom)?,
            previous: image(fields.previous).map_err(D::Error::custom)?,
            metadata: fields.metadata,
        })
    }
}

#[cfg(test)]
mod tests;

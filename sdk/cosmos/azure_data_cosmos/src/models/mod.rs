// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Model types sent to and received from the Azure Cosmos DB API.

use azure_core::{fmt::SafeDebug, http::Etag, time::OffsetDateTime};
use serde::{Deserialize, Deserializer, Serialize};

mod account_properties;
mod container_properties;
mod cosmos_response;
mod indexing_policy;
mod partition_key_definition;
mod patch_operations;
mod throughput_properties;

pub use account_properties::*;
pub use container_properties::*;
pub use cosmos_response::CosmosResponse;
pub use indexing_policy::*;
pub use partition_key_definition::*;
pub use patch_operations::*;
pub use throughput_properties::*;

fn deserialize_cosmos_timestamp<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let seconds_since_epoch = Option::<i64>::deserialize(deserializer)?;
    match seconds_since_epoch {
        None => Ok(None),
        Some(seconds) => Ok(Some(OffsetDateTime::from_unix_timestamp(seconds).map_err(
            |_| {
                use serde::de::Error;
                D::Error::invalid_value(
                    serde::de::Unexpected::Signed(seconds),
                    &"a valid timestamp",
                )
            },
        )?)),
    }
}

/// Common system properties returned for most Cosmos DB resources.
#[non_exhaustive]
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
pub struct SystemProperties {
    /// The entity tag associated with the resource.
    #[serde(default)]
    #[serde(skip_serializing)]
    #[serde(rename = "_etag")]
    etag: Option<Etag>,

    /// The self-link associated with the resource.
    #[serde(default)]
    #[serde(skip_serializing)]
    #[serde(rename = "_self")]
    self_link: Option<String>,

    /// The system-generated unique identifier associated with the resource.
    #[serde(default)]
    // Some APIs do expect the "_rid" to be provided (Replace Offer, for example), so we do want to serialize it if it's provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_rid")]
    resource_id: Option<String>,

    /// A [`OffsetDateTime`] representing the last modified time of the resource.
    #[serde(default)]
    #[serde(rename = "_ts")]
    #[serde(skip_serializing)]
    #[serde(deserialize_with = "deserialize_cosmos_timestamp")]
    last_modified: Option<OffsetDateTime>,
}

impl SystemProperties {
    /// Gets the entity tag associated with the resource.
    pub fn etag(&self) -> Option<&Etag> {
        self.etag.as_ref()
    }

    /// Gets the self-link associated with the resource.
    pub fn self_link(&self) -> Option<&str> {
        self.self_link.as_deref()
    }

    /// Gets the system-generated unique identifier associated with the resource.
    pub fn resource_id(&self) -> Option<&str> {
        self.resource_id.as_deref()
    }

    /// Gets the last modified time of the resource.
    pub fn last_modified(&self) -> Option<&OffsetDateTime> {
        self.last_modified.as_ref()
    }
}

/// Properties of a Cosmos DB database.
///
/// # Required fields
///
/// * `id` — The unique identifier for the database.
///
/// Use [`DatabaseProperties::new()`] to construct an instance:
///
/// ```rust
/// # use azure_data_cosmos::models::DatabaseProperties;
/// let properties = DatabaseProperties::new("MyDatabase");
/// ```
///
/// Returned by [`DatabaseClient::read()`](crate::clients::DatabaseClient::read()).
#[non_exhaustive]
#[derive(Clone, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
pub struct DatabaseProperties {
    /// The ID of the database.
    id: String,

    /// A [`SystemProperties`] object containing common system properties for the database.
    #[serde(flatten)]
    #[serde(default)]
    system_properties: SystemProperties,
}

impl DatabaseProperties {
    /// Creates a new [`DatabaseProperties`] with the required `id` field.
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            system_properties: SystemProperties::default(),
        }
    }

    /// Gets the ID of the database.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Gets the common system properties for the database.
    pub fn system_properties(&self) -> &SystemProperties {
        &self.system_properties
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use time::{Date, Month, OffsetDateTime, Time};

    #[cfg(test)]
    #[derive(Debug, Deserialize, Serialize)]
    struct TimestampHolder {
        #[serde(default)]
        #[serde(deserialize_with = "super::deserialize_cosmos_timestamp")]
        pub ts: Option<OffsetDateTime>,
    }

    #[test]
    pub fn deserialize_timestamp() {
        let value: TimestampHolder = serde_json::from_str(r#"{"ts":1729036800}"#).unwrap();
        let expected = OffsetDateTime::new_utc(
            Date::from_calendar_date(2024, Month::October, 16).unwrap(), // Can't be a const because Result::unwrap isn't const.
            Time::MIDNIGHT,
        );

        assert_eq!(Some(expected), value.ts);
    }

    #[test]
    pub fn deserialize_missing_timestamp() {
        let value: TimestampHolder = serde_json::from_str(r#"{}"#).unwrap();
        assert_eq!(None, value.ts);
    }

    #[test]
    pub fn deserialize_null_timestamp() {
        let value: TimestampHolder = serde_json::from_str(r#"{"ts":null}"#).unwrap();
        assert_eq!(None, value.ts);
    }
}

// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Model types sent to and received from the Azure Cosmos DB API.

use azure_core::{fmt::SafeDebug, http::Etag, time::OffsetDateTime};
use serde::{Deserialize, Deserializer, Serialize};

mod account_properties;
mod container_properties;
mod indexing_policy;
mod partition_key_definition;
mod patch_operations;
mod throughput_properties;

pub use account_properties::*;
pub use container_properties::*;
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
    pub etag: Option<Etag>,

    /// The self-link associated with the resource.
    #[serde(default)]
    #[serde(skip_serializing)]
    #[serde(rename = "_self")]
    pub self_link: Option<String>,

    /// The system-generated unique identifier associated with the resource.
    #[serde(default)]
    // Some APIs do expect the "_rid" to be provided (Replace Offer, for example), so we do want to serialize it if it's provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "_rid")]
    pub resource_id: Option<String>,

    /// A [`OffsetDateTime`] representing the last modified time of the resource.
    #[serde(default)]
    #[serde(rename = "_ts")]
    #[serde(skip_serializing)]
    #[serde(deserialize_with = "deserialize_cosmos_timestamp")]
    pub last_modified: Option<OffsetDateTime>,
}

/// Properties of a Cosmos DB database.
///
/// Returned by [`DatabaseClient::read()`](crate::clients::DatabaseClient::read()).
#[non_exhaustive]
#[derive(Clone, Default, SafeDebug, Deserialize, Serialize, PartialEq, Eq)]
#[safe(true)]
pub struct DatabaseProperties {
    /// The ID of the database.
    pub id: String,

    /// A [`SystemProperties`] object containing common system properties for the database.
    #[serde(flatten)]
    pub system_properties: SystemProperties,
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

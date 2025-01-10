// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

//! Unix timestamp serde helpers.
use serde::{de, Deserialize, Deserializer, Serializer};
use time::{OffsetDateTime, UtcOffset};

/// Deserialize a Unix timestamp into an [`OffsetDateTime`].
pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let i = i64::deserialize(deserializer)?;
    OffsetDateTime::from_unix_timestamp(i).map_err(de::Error::custom)
}

/// Serialize an [`OffsetDateTime`] to a Unix timestamp.
pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(date.to_offset(UtcOffset::UTC).unix_timestamp())
}

pub mod option {
    use serde::{Deserialize, Deserializer, Serializer};
    use time::{OffsetDateTime, UtcOffset};

    /// Deserialize a Unix timestamp into an optional [`OffsetDateTime`].
    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let i: Option<i64> = Option::deserialize(deserializer)?;
        i.map(|i| OffsetDateTime::from_unix_timestamp(i).map_err(serde::de::Error::custom))
            .transpose()
    }

    /// Serialize an optional [`OffsetDateTime`] to a Unix timestamp.
    pub fn serialize<S>(date: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(date) = date {
            serializer.serialize_i64(date.to_offset(UtcOffset::UTC).unix_timestamp())
        } else {
            serializer.serialize_none()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::json::{from_json, to_json};
    use serde::{Deserialize, Serialize};
    use time::macros::datetime;

    #[derive(Deserialize, Serialize)]
    struct TestType {
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::date::unix_time::option"
        )]
        optional_timestamp: Option<time::OffsetDateTime>,

        #[serde(with = "crate::date::unix_time")]
        required_timestamp: time::OffsetDateTime,
    }

    #[test]
    fn test_deserialize_none() -> crate::Result<()> {
        let json_body = r#"{"required_timestamp":1627904772}"#;
        let test_type: TestType = from_json(json_body)?;
        assert_eq!(test_type.optional_timestamp, None);
        assert_eq!(
            test_type.required_timestamp,
            datetime!(2021-08-02 11:46:12 UTC)
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_null() -> crate::Result<()> {
        let json_body = r#"{"optional_timestamp":null,"required_timestamp":1627904772}"#;
        let test_type: TestType = from_json(json_body)?;
        assert_eq!(test_type.optional_timestamp, None);
        assert_eq!(
            test_type.required_timestamp,
            datetime!(2021-08-02 11:46:12 UTC)
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_some() -> crate::Result<()> {
        let json_body = r#"{"optional_timestamp":1625136302,"required_timestamp":1627904772}"#;
        let test_type: TestType = from_json(json_body)?;
        assert_eq!(
            test_type.optional_timestamp,
            Some(datetime!(2021-07-01 10:45:02 UTC))
        );
        assert_eq!(
            test_type.required_timestamp,
            datetime!(2021-08-02 11:46:12 UTC)
        );
        Ok(())
    }

    #[test]
    fn test_serialize_none() -> crate::Result<()> {
        let test_type = TestType {
            optional_timestamp: None,
            required_timestamp: datetime!(2021-08-02 11:46:12 UTC),
        };
        let json_body = to_json(&test_type)?;
        assert_eq!(json_body, r#"{"required_timestamp":1627904772}"#);
        Ok(())
    }

    #[test]
    fn test_serialize_some() -> crate::Result<()> {
        let test_type = TestType {
            optional_timestamp: Some(datetime!(2021-07-01 10:45:02 UTC)),
            required_timestamp: datetime!(2021-08-02 11:46:12 UTC),
        };
        let json_body = to_json(&test_type)?;
        assert_eq!(
            json_body,
            r#"{"optional_timestamp":1625136302,"required_timestamp":1627904772}"#
        );
        Ok(())
    }
}

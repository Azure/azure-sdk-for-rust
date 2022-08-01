use serde::{self, Deserialize, Deserializer, Serializer};
use time::OffsetDateTime;

use crate::date::*;
use serde::de;

pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_rfc1123(&s).map_err(de::Error::custom)
}

pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&to_rfc1123(date))
}

pub mod option {
    use crate::date::*;
    use serde::{Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        s.map(|s| parse_rfc1123(&s).map_err(serde::de::Error::custom))
            .transpose()
    }

    pub fn serialize<S>(date: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(date) = date {
            serializer.serialize_str(&to_rfc1123(date))
        } else {
            serializer.serialize_none()
        }
    }
}

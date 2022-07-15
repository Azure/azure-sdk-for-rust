//! An assortment of helper utilities.

use chrono::{DateTime, FixedOffset, Utc};
use serde::{
    de::{self, DeserializeOwned, Deserializer},
    Deserialize,
};

pub fn case_insensitive_deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: DeserializeOwned + std::fmt::Debug,
    D: Deserializer<'de>,
{
    let v = String::deserialize(deserializer)?;
    T::deserialize(serde_json::Value::String(v.clone()))
        .or_else(|_| T::deserialize(serde_json::Value::String(v.to_lowercase())))
        .map_err(de::Error::custom)
}

pub fn deserialize_utc_date_from_rfc2822<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    crate::headers::utc_date_from_rfc2822(&s).map_err(serde::de::Error::custom)
}

pub fn deserialize_date_from_rfc3339<'de, D>(
    deserializer: D,
) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)
}

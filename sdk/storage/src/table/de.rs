use chrono::{DateTime, Utc};
use serde::{
    de::{Error, Visitor},
    Deserializer,
};
use std::fmt;

/// Serde visitor to deserialize Timestamp
struct TimestampVisitor;

pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_str(TimestampVisitor)
}

impl<'de> Visitor<'de> for TimestampVisitor {
    type Value = DateTime<Utc>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "a timestamp string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        match value.parse::<DateTime<Utc>>() {
            Ok(date) => Ok(date),
            Err(e) => Err(E::custom(format!("Parse error {} for {}", e, value))),
        }
    }
}

/// Serde visitor to deserialize Option<Timestamp>
struct OptionalTimestampVisitor;

impl<'de> Visitor<'de> for OptionalTimestampVisitor {
    type Value = Option<DateTime<Utc>>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "null or a timestamp string")
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(None)
    }

    fn visit_some<D>(self, d: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Some(d.deserialize_str(TimestampVisitor)?))
    }
}

pub fn optional_timestamp<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    d.deserialize_option(OptionalTimestampVisitor)
}

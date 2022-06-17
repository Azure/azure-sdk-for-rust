//! Parser helper utilities.

use crate::error::{Error, ErrorKind, Result, ResultExt};

pub trait FromStringOptional<T> {
    fn from_str_optional(s: &str) -> Result<T>;
}

impl FromStringOptional<u64> for u64 {
    fn from_str_optional(s: &str) -> Result<u64> {
        s.parse::<u64>().map_kind(ErrorKind::DataConversion)
    }
}

impl FromStringOptional<String> for String {
    fn from_str_optional(s: &str) -> Result<String> {
        Ok(s.to_owned())
    }
}

impl FromStringOptional<bool> for bool {
    fn from_str_optional(s: &str) -> Result<bool> {
        match s {
            "true" => Ok(true),
            "false" => Ok(false),
            _ => Err(Error::with_message(ErrorKind::DataConversion, || {
                "error parsing bool '{s}'"
            })),
        }
    }
}

impl FromStringOptional<chrono::DateTime<chrono::Utc>> for chrono::DateTime<chrono::Utc> {
    fn from_str_optional(s: &str) -> Result<chrono::DateTime<chrono::Utc>> {
        from_azure_time(s).with_context(ErrorKind::DataConversion, || {
            format!("error parsing date time '{s}'")
        })
    }
}

#[inline]
#[cfg(not(feature = "azurite_workaround"))]
pub fn from_azure_time(s: &str) -> Result<chrono::DateTime<chrono::Utc>> {
    let dt = chrono::DateTime::parse_from_rfc2822(s).map_kind(ErrorKind::DataConversion)?;
    let dt_utc: chrono::DateTime<chrono::Utc> = dt.with_timezone(&chrono::Utc);
    Ok(dt_utc)
}

pub mod rfc2822_time_format {
    use super::from_azure_time;
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        from_azure_time(&s).map_err(serde::de::Error::custom)
    }

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&date.to_rfc2822())
    }
}

pub mod rfc2822_time_format_optional {
    use super::from_azure_time;
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        s.map(|s| from_azure_time(&s).map_err(serde::de::Error::custom))
            .transpose()
    }

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(date) = date {
            serializer.serialize_str(&date.to_rfc2822())
        } else {
            serializer.serialize_none()
        }
    }
}

#[inline]
#[cfg(feature = "azurite_workaround")]
pub fn from_azure_time(s: &str) -> Result<chrono::DateTime<chrono::Utc>> {
    if let Ok(dt) = chrono::DateTime::parse_from_rfc2822(s) {
        let dt_utc: chrono::DateTime<chrono::Utc> = dt.with_timezone(&chrono::Utc);
        Ok(dt_utc)
    } else {
        log::warn!("Received an invalid date: {}, returning now()", s);
        Ok(chrono::Utc::now())
    }
}

#[cfg(test)]
mod test {
    use chrono::{Datelike, Timelike};

    #[test]
    fn test_from_azure_time() {
        let t = super::from_azure_time("Sun, 27 Sep 2009 17:26:40 GMT").unwrap();

        assert_eq!(t.day(), 27);
        assert_eq!(t.month(), 9);
        assert_eq!(t.hour(), 17);
        assert_eq!(t.second(), 40);
    }
}

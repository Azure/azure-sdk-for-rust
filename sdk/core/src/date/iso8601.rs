use crate::error::{ErrorKind, ResultExt};
use serde::{self, de, Deserialize, Deserializer, Serializer};
use time::{
    format_description::well_known::{
        iso8601::{Config, EncodedConfig, TimePrecision},
        Iso8601,
    },
    OffsetDateTime, UtcOffset,
};

const SERDE_CONFIG: EncodedConfig = Config::DEFAULT
    .set_year_is_six_digits(false)
    .set_time_precision(TimePrecision::Second {
        decimal_digits: None,
    })
    .encode();

pub fn parse_iso8601(s: &str) -> crate::Result<OffsetDateTime> {
    OffsetDateTime::parse(s, &Iso8601::<SERDE_CONFIG>)
        .with_context(ErrorKind::DataConversion, || {
            format!("unable to parse iso8601 date '{s}")
        })
}

pub fn to_iso8601(date: &OffsetDateTime) -> crate::Result<String> {
    date.format(&Iso8601::<SERDE_CONFIG>)
        .with_context(ErrorKind::DataConversion, || {
            format!("unable to format date '{date:?}")
        })
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<OffsetDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    parse_iso8601(&s).map_err(de::Error::custom)
}

pub fn serialize<S>(date: &OffsetDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    date.to_offset(UtcOffset::UTC);
    let as_str = to_iso8601(date).map_err(serde::ser::Error::custom)?;
    serializer.serialize_str(&as_str)
}

pub mod option {
    use crate::date::iso8601::{parse_iso8601, to_iso8601};
    use serde::{Deserialize, Deserializer, Serializer};
    use time::OffsetDateTime;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        s.map(|s| parse_iso8601(&s).map_err(serde::de::Error::custom))
            .transpose()
    }

    pub fn serialize<S>(date: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(date) = date {
            serializer.serialize_str(&to_iso8601(date).map_err(serde::ser::Error::custom)?)
        } else {
            serializer.serialize_none()
        }
    }
}

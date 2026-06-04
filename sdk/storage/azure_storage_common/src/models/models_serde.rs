pub mod option_offset_date_time_rfc3339 {
    use azure_core::time::{parse_rfc3339, OffsetDateTime};
    use serde::{Deserialize, Deserializer, Serializer};
    use time::format_description::well_known::Rfc3339;

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<OffsetDateTime>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = <Option<String>>::deserialize(deserializer)?;
        match value {
            Some(s) => {
                let dt = parse_rfc3339(&s).map_err(serde::de::Error::custom)?;
                Ok(Some(dt))
            }
            None => Ok(None),
        }
    }

    pub fn serialize<S>(value: &Option<OffsetDateTime>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match value {
            Some(dt) => {
                let s = dt.format(&Rfc3339).map_err(serde::ser::Error::custom)?;
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_none(),
        }
    }
}

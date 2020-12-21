use chrono::{DateTime, SecondsFormat, Utc};
use serde::{self, Deserialize, Deserializer, Serializer};

pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.to_rfc3339_opts(SecondsFormat::Secs, true))
}

#[allow(dead_code)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    DateTime::parse_from_rfc3339(&String::deserialize(deserializer)?)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, NaiveDate, Utc};
    use serde::{self, Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    struct TestEntity {
        #[serde(with = "super")]
        pub datetime: DateTime<Utc>,
    }

    #[test]
    fn serialize_valid_datetime() {
        let entity = TestEntity {
            datetime: DateTime::<Utc>::from_utc(
                NaiveDate::from_ymd(2020, 12, 21).and_hms(14, 53, 41),
                Utc,
            ),
        };

        assert_eq!(
            serde_json::to_string(&entity).unwrap(),
            "{\"datetime\":\"2020-12-21T14:53:41Z\"}"
        );
    }

    #[test]
    fn deserialize_valid_datetime() {
        let timestamp = NaiveDate::from_ymd(2020, 12, 21)
            .and_hms(14, 53, 41)
            .timestamp();
        let entity =
            serde_json::from_str::<TestEntity>("{\"datetime\":\"2020-12-21T14:53:41Z\"}").unwrap();

        assert_eq!(entity.datetime.timestamp(), timestamp);
    }
}

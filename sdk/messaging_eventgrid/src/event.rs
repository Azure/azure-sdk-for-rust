use serde::{self, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// An Event Grid Event, used to create new events that subscribers will receive.
/// In compliance with spec: <https://docs.microsoft.com/azure/event-grid/event-schema>
pub struct Event<T>
where
    T: Serialize,
{
    pub topic: Option<String>,
    pub id: String,
    pub event_type: String,
    pub subject: String,
    #[serde(with = "azure_core::date::rfc3339")]
    pub event_time: OffsetDateTime,
    pub data: Option<T>,
    pub data_version: String,
    pub metadata_version: Option<String>,
}

impl<T> Event<T>
where
    T: Serialize,
{
    /// Create an Event containing the given data with the event time set to now.
    /// If left unspecified, the id will be set to a random v4 uuid.
    /// If left unspecified, the data version will be set to "0.1".
    /// ```
    /// # use azure_messaging_eventgrid::Event;
    /// # use serde::Serialize;
    /// #[derive(Serialize)]
    /// struct Data { number: i32 }
    ///
    /// let event = Event::<Data>::new(None, "ACME.Data.DataPointCreated", "/acme/data", Data { number: 42 }, None);
    /// # assert_eq!(event.event_type, "ACME.Data.DataPointCreated");
    /// # assert_eq!(event.subject, "/acme/data");
    /// # assert_eq!(event.data.unwrap().number, 42);
    /// # assert_eq!(event.data_version, "0.1");
    /// ```
    pub fn new(
        id: Option<String>,
        event_type: &str,
        subject: &str,
        data: T,
        data_version: Option<String>,
    ) -> Event<T> {
        Self {
            id: id.unwrap_or_else(|| Uuid::new_v4().to_string()),
            event_type: event_type.to_owned(),
            subject: subject.to_owned(),
            data_version: data_version.unwrap_or_else(|| String::from("0.1")),
            data: Some(data),
            event_time: OffsetDateTime::now_utc(),
            topic: None,
            metadata_version: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{self, Serialize};
    use time::macros::datetime;

    #[derive(Serialize)]
    struct Data {
        pub number: i32,
    }

    #[test]
    fn create_and_serialize() {
        let mut event = Event::<Data>::new(
            Some(String::from("an id")),
            "ACME.Data.DataPointCreated",
            "/acme/data",
            Data { number: 42 },
            Some(String::from("1.0")),
        );
        event.event_time = datetime!(2020-12-21 14:53:41 UTC);

        assert_eq!(
            serde_json::to_string(&event).unwrap(),
            "{\"topic\":null,\"id\":\"an id\",\"eventType\":\"ACME.Data.DataPointCreated\",\"subject\":\"/acme/data\",\"eventTime\":\"2020-12-21T14:53:41Z\",\"data\":{\"number\":42},\"dataVersion\":\"1.0\",\"metadataVersion\":null}"
        );
    }
}

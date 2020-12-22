use crate::rfc3339_utc_serializer;
use chrono::{DateTime, Utc};
use serde::{self, Serialize};
use uuid::Uuid;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
/// An Event Grid Event, used to create new events that subscribers will receive.
/// In compliance with spec: https://docs.microsoft.com/en-us/azure/event-grid/event-schema
pub struct Event<T>
where
    T: Serialize,
{
    pub topic: Option<String>,
    pub id: String,
    pub event_type: String,
    pub subject: String,
    #[serde(with = "rfc3339_utc_serializer")]
    pub event_time: DateTime<Utc>,
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
    /// # use azure_event_grid::Event;
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
            event_time: Utc::now(),
            topic: None,
            metadata_version: None,
        }
    }
}

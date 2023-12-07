use crate::{event_grid_request_builder::EventGridRequestBuilder, Event};
use azure_core::Url;
use serde::ser::Serialize;

#[derive(Clone)]
pub struct EventGridClient {
    client: hyper::Client<HttpsConnector<HttpConnector>>,
    pub topic_host_name: String,
    pub topic_key: String,
}

impl EventGridClient {
    /// Create an event grid client that can publish events to an event grid topic.
    /// ```
    /// # use azure_messaging_eventgrid::EventGridClient;
    /// let client = EventGridClient::new(String::from("https://name.location.eventgrid.azure.net"), String::from("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="));
    /// # assert_eq!(client.topic_host_name, "https://name.location.eventgrid.azure.net");
    /// # assert_eq!(client.topic_key, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
    /// ```
    pub fn new(topic_host_name: String, topic_key: String) -> Self {
        Self {
            client: hyper::Client::builder().build(HttpsConnector::with_native_roots()),
            topic_host_name,
            topic_key,
        }
    }

    /// Publishes events to a topic.
    /// REST API Spec: https://docs.microsoft.com/rest/api/eventgrid/dataplane/publishevents/publishevents
    pub async fn publish_events<T>(&self, events: &[Event<T>]) -> azure_core::Result<()>
    where
        T: Serialize,
    {
        let body = from_json(&events)?;
        EventGridRequestBuilder::new(Method::Post, &self.events_url()?)
            .sas_key(&self.topic_key)
            .body(Some(&body), Some("application/json"))?
            .request(&self.client)
            .expect(StatusCode::Ok)
            .await?;

        Ok(())
    }

    fn events_url(&self) -> Result<String, azure_core::Error> {
        let mut url = Url::parse(&self.topic_host_name)?;
        url.set_path("/api/events");
        url.set_query(Some("api-version=2018-01-01"));
        Ok(url.to_string())
    }
}

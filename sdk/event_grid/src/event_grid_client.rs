use crate::event_grid_request_builder::EventGridRequestBuilder;
use crate::Event;
use azure_core::errors::AzureError;
use hyper::{self, client::HttpConnector, Method, StatusCode};
use hyper_rustls::HttpsConnector;
use serde::ser::Serialize;
use url::Url;

#[derive(Clone)]
pub struct EventGridClient<'a, 'b> {
    client: hyper::Client<HttpsConnector<HttpConnector>>,
    pub topic_host_name: &'a str,
    pub topic_key: &'b str,
}

impl<'a, 'b> EventGridClient<'a, 'b> {
    /// Create an event grid client that can publish events to an event grid topic.
    /// ```
    /// # use azure_event_grid::EventGridClient;
    /// let client = EventGridClient::new("https://name.location.eventgrid.azure.net", "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
    /// # assert_eq!(client.topic_host_name, "https://name.location.eventgrid.azure.net");
    /// # assert_eq!(client.topic_key, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
    /// ```
    pub fn new(topic_host_name: &'a str, topic_key: &'b str) -> EventGridClient<'a, 'b> {
        EventGridClient {
            client: hyper::Client::builder().build(HttpsConnector::new()),
            topic_host_name,
            topic_key,
        }
    }

    /// Publishes events to a topic.
    /// REST API Spec: https://docs.microsoft.com/en-us/rest/api/eventgrid/dataplane/publishevents/publishevents
    pub async fn publish_events<T>(&self, events: &[Event<T>]) -> Result<(), AzureError>
    where
        T: Serialize,
    {
        let body = serde_json::to_string(&events).unwrap();
        EventGridRequestBuilder::new(Method::POST, &self.events_url()?)
            .sas_key(self.topic_key)
            .body(Some(&body), Some("application/json"))?
            .request(&self.client)
            .expect(StatusCode::OK)
            .await?;

        Ok(())
    }

    fn events_url(&self) -> Result<String, AzureError> {
        let mut url = Url::parse(self.topic_host_name)?;
        url.set_path("/api/events");
        url.set_query(Some("api-version=2018-01-01"));
        Ok(url.to_string())
    }
}

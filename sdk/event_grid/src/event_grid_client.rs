use crate::Event;
use azure_core::{errors::AzureError, HttpClient};
use bytes::Bytes;
use http::{
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    Method, StatusCode,
};
use serde::ser::Serialize;
use std::sync::Arc;
use url::Url;

#[derive(Clone)]
pub struct EventGridClient {
    client: Arc<Box<dyn HttpClient>>,
    pub topic_host_name: String,
    pub topic_key: String,
}

impl EventGridClient {
    /// Create an event grid client that can publish events to an event grid topic.
    /// ```
    /// # use azure_event_grid::EventGridClient;
    /// let client = EventGridClient::new(String::from("https://name.location.eventgrid.azure.net"), String::from("AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA="));
    /// # assert_eq!(client.topic_host_name, "https://name.location.eventgrid.azure.net");
    /// # assert_eq!(client.topic_key, "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=");
    /// ```
    pub fn new(
        topic_host_name: String,
        topic_key: String,
        client: Arc<Box<dyn HttpClient>>,
    ) -> Self {
        Self {
            client,
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
        let body = Bytes::from(serde_json::to_vec(&events)?);
        let req = http::request::Builder::new()
            .uri(&self.events_url()?)
            .method(Method::POST)
            .header("aeg-sas-key", &self.topic_key)
            .header(CONTENT_TYPE, "application/json")
            .header(CONTENT_LENGTH, &body.len().to_string())
            .body(body)?;

        let _res = self
            .client
            .execute_request_check_status(req, StatusCode::OK)
            .await?;
        // EventGridRequestBuilder::new(Method::POST, &self.events_url()?)
        //     .sas_key(&self.topic_key)
        //     .body(Some(&body), Some("application/json"))?
        //     .request(&self.client)
        //     .expect(StatusCode::OK)
        //     .await?;
        Ok(())
    }

    fn events_url(&self) -> Result<String, AzureError> {
        let mut url = Url::parse(&self.topic_host_name)?;
        url.set_path("/api/events");
        url.set_query(Some("api-version=2018-01-01"));
        Ok(url.to_string())
    }
}

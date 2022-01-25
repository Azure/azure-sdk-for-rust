#![allow(missing_docs)]

use std::convert::TryInto;

use azure_core::prelude::*;
use azure_core::setters;
use http::{Method, StatusCode};
use serde::Serialize;

use crate::service::{responses::QueryResponse, ServiceClient, API_VERSION};

/// Body for the Query request
#[derive(Serialize, Debug)]
struct QueryBody {
    query: String,
}

/// Builder for creating queries
pub struct QueryBuilder<'a, 'b> {
    service_client: &'a ServiceClient,
    max_item_count: MaxItemCount,
    continuation: Option<Continuation<'b>>,
}

impl<'a, 'b> QueryBuilder<'a, 'b> {
    /// Create a new query struct
    pub(crate) fn new(service_client: &'a ServiceClient) -> Self {
        Self {
            service_client,
            max_item_count: MaxItemCount::new(-1),
            continuation: None,
        }
    }

    azure_core::setters! {
        continuation: &'b str => Some(Continuation::new(continuation)),
        max_item_count: i32 => MaxItemCount::new(max_item_count),
    }

    /// Invoke a qiven query on the IoT Hub
    ///
    /// ```
    /// use std::sync::Arc;
    /// use azure_core::HttpClient;
    /// use azure_iot_hub::service::ServiceClient;
    ///
    /// # let http_client = azure_core::new_http_client();
    /// # let connection_string = "HostName=cool-iot-hub.azure-devices.net;SharedAccessKeyName=iot_hubowner;SharedAccessKey=YSB2ZXJ5IHNlY3VyZSBrZXkgaXMgaW1wb3J0YW50Cg==";
    /// let iot_hub = ServiceClient::from_connection_string(http_client, connection_string, 3600).expect("Failed to create the ServiceClient!");
    /// let query_builder = iot_hub.query().max_item_count(1).continuation("some_token").execute("SELECT * FROM devices");
    /// ```
    pub async fn execute<S>(self, query: S) -> crate::Result<QueryResponse>
    where
        S: Into<String>,
    {
        let uri = format!(
            "https://{}.azure-devices.net/devices/query?api-version={}",
            self.service_client.iot_hub_name, API_VERSION
        );

        let query_body = QueryBody {
            query: query.into(),
        };
        let body = azure_core::to_json(&query_body)?;

        let request = self.service_client.prepare_request(&uri, Method::POST);
        let request = azure_core::headers::add_optional_header(&self.continuation, request);
        let request = azure_core::headers::add_mandatory_header(&self.max_item_count, request);
        let request = request.body(body)?;

        Ok(self
            .service_client
            .http_client()
            .execute_request_check_status(request, StatusCode::OK)
            .await?
            .try_into()?)
    }
}

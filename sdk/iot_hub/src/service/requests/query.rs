#![allow(missing_docs)]

use crate::service::{responses::QueryResponse, ServiceClient, API_VERSION};
use azure_core::prelude::*;
use azure_core::Method;
use serde::Serialize;
use std::convert::TryInto;

/// Body for the Query request
#[derive(Serialize, Debug)]
struct QueryBody {
    query: String,
}

azure_core::operation! {
    /// Builder for creating queries
    Query,
    client: ServiceClient,
    query: String,
    ?max_item_count: MaxItemCount,
    ?continuation: Continuation
}

impl QueryBuilder {
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
    pub fn into_future(self) -> Query {
        Box::pin(async move {
            let uri = format!(
                "https://{}.azure-devices.net/devices/query?api-version={}",
                self.client.iot_hub_name, API_VERSION
            );

            let query_body = QueryBody {
                query: self.query.into(),
            };
            let body = azure_core::to_json(&query_body)?;

            let mut request = self.client.finalize_request(&uri, Method::Post)?;
            request.add_optional_header(&self.continuation);
            request.add_mandatory_header(&self.max_item_count.unwrap_or_default());
            request.set_body(body);

            self.client
                .http_client()
                .execute_request_check_status(&request)
                .await?
                .try_into()
        })
    }
}

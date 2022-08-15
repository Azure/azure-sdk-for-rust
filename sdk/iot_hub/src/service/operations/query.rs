#![allow(missing_docs)]

use crate::service::{responses::QueryResponse, ServiceClient, API_VERSION};
use azure_core::prelude::*;
use azure_core::Method;
use serde::Serialize;

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
    pub fn into_future(mut self) -> Query {
        Box::pin(async move {
            let uri = format!(
                "https://{}.azure-devices.net/devices/query?api-version={}",
                self.client.iot_hub_name, API_VERSION
            );

            let query_body = QueryBody { query: self.query };
            let body = azure_core::to_json(&query_body)?;

            let mut request = self.client.finalize_request(&uri, Method::Post)?;
            request.add_optional_header(&self.continuation);
            request.add_mandatory_header(&self.max_item_count.unwrap_or_default());
            request.set_body(body);

            let response = self.client.send(&mut self.context, &mut request).await?;

            QueryResponse::try_from(response).await
        })
    }
}

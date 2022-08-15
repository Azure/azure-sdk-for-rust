use crate::QueueServiceClient;
use azure_core::{
    error::Error, headers::Headers, prelude::*, Method, Pageable, Response as AzureResponse,
};
use azure_storage::{core::headers::CommonStorageResponseHeaders, xml::read_xml};
use std::convert::TryInto;

operation! {
    #[stream]
    ListQueues,
    client: QueueServiceClient,
    ?prefix: Prefix,
    ?max_results: MaxResults,
    ?include_metadata: bool,
}

impl ListQueuesBuilder {
    pub fn into_stream(self) -> Pageable<ListQueuesResponse, Error> {
        let make_request = move |continuation: Option<NextMarker>| {
            let mut this = self.clone();
            async move {
                let mut url = this.client.storage_client.queue_storage_url().to_owned();

                url.query_pairs_mut().append_pair("comp", "list");

                this.prefix.append_to_url_query(&mut url);

                if let Some(next_marker) = continuation {
                    next_marker.append_to_url_query(&mut url);
                }

                this.max_results.append_to_url_query(&mut url);

                if this.include_metadata.unwrap_or(false) {
                    url.query_pairs_mut().append_pair("include", "metadata");
                }

                let mut request = this.client.storage_client.finalize_request(
                    url,
                    Method::Get,
                    Headers::new(),
                    None,
                )?;

                let response = this.client.send(&mut this.context, &mut request).await?;

                ListQueuesResponse::try_from(response).await
            }
        };

        Pageable::new(make_request)
    }
}

#[derive(Debug, Clone)]
pub struct ListQueuesResponse {
    pub common_storage_response_headers: CommonStorageResponseHeaders,
    pub service_endpoint: String,
    pub prefix: Option<String>,
    // this seems duplicate :S
    pub marker: Option<String>,
    pub max_results: Option<u32>,
    pub queues: Vec<Queue>,
    pub next_marker: Option<NextMarker>,
}

impl Continuable for ListQueuesResponse {
    type Continuation = NextMarker;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.next_marker.clone()
    }
}

impl ListQueuesResponse {
    async fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let body = body.collect().await?;

        let mut response: ListQueuesResponseInternal = read_xml(&body)?;

        if let Some("") = response.next_marker.as_deref() {
            response.next_marker = None;
        }

        Ok(ListQueuesResponse {
            common_storage_response_headers: (&headers).try_into()?,
            service_endpoint: response.service_endpoint,
            prefix: response.prefix,
            marker: response.marker,
            max_results: response.max_results,
            queues: response.queues.queues,
            next_marker: response.next_marker.map(|nm| nm.into()),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ListQueuesResponseInternal {
    pub service_endpoint: String,
    pub prefix: Option<String>,
    pub marker: Option<String>,
    pub max_results: Option<u32>,
    pub queues: Queues,
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Queues {
    #[serde(rename = "Queue", default)]
    pub queues: Vec<Queue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Queue {
    pub name: String,
    #[serde(default)]
    pub metadata: std::collections::HashMap<String, String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_parse() {
        let range = "<?xml version=\"1.0\" encoding=\"utf-8\"?><EnumerationResults ServiceEndpoint=\"https://azureskdforrust.queue.core.windows.net/\"><Prefix>a</Prefix><MaxResults>2</MaxResults><Queues><Queue><Name>azureiscool</Name></Queue><Queue><Name>azurerocks</Name></Queue></Queues><NextMarker /></EnumerationResults>";

        let response: ListQueuesResponseInternal = read_xml(range.as_bytes()).unwrap();

        assert_eq!(response.queues.queues.len(), 2);
    }
}

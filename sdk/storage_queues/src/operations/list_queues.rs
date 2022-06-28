use crate::QueueServiceClient;
use azure_core::{
    collect_pinned_stream, error::Error, headers::Headers, prelude::*, Context, Method, Pageable,
    Response as AzureResponse,
};
use azure_storage::{core::headers::CommonStorageResponseHeaders, xml::read_xml};
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListQueuesBuilder {
    service_client: QueueServiceClient,
    prefix: Option<Prefix>,
    max_results: Option<MaxResults>,
    include_metadata: bool,
    timeout: Option<Timeout>,
    context: Context,
}

impl ListQueuesBuilder {
    pub(crate) fn new(service_client: QueueServiceClient) -> Self {
        Self {
            service_client,
            prefix: None,
            max_results: None,
            include_metadata: false,
            timeout: None,
            context: Context::new(),
        }
    }

    setters! {
        prefix: Prefix => Some(prefix),
        max_results: MaxResults => Some(max_results),
        include_metadata: bool => include_metadata,
        timeout: Timeout => Some(timeout),
        context: Context => context,
    }

    pub fn into_stream(self) -> Pageable<ListQueuesResponse, Error> {
        let make_request = move |continuation: Option<Continuation>| {
            let mut this = self.clone();
            async move {
                let mut url = this
                    .service_client
                    .storage_client
                    .storage_account_client()
                    .queue_storage_url()
                    .to_owned();

                url.query_pairs_mut().append_pair("comp", "list");

                this.prefix.append_to_url_query(&mut url);

                if let Some(continuation) = continuation {
                    url.query_pairs_mut()
                        .append_pair("marker", &continuation.as_string());
                }

                this.max_results.append_to_url_query(&mut url);

                if this.include_metadata {
                    url.query_pairs_mut().append_pair("include", "metadata");
                }

                this.timeout.append_to_url_query(&mut url);
                AppendToUrlQuery::append_to_url_query(&this.timeout, &mut url);

                let mut request = this.service_client.storage_client.prepare_request(
                    url,
                    Method::Get,
                    Headers::new(),
                    None,
                )?;

                let response = this
                    .service_client
                    .send(&mut this.context, &mut request)
                    .await?;

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
    fn continuation(&self) -> Option<Continuation> {
        self.next_marker.clone().map(Continuation::from)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ListQueuesResponseInternal {
    #[serde(rename = "ServiceEndpoint")]
    pub service_endpoint: String,
    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,
    #[serde(rename = "Marker")]
    pub marker: Option<String>,
    #[serde(rename = "MaxResults")]
    pub max_results: Option<u32>,

    #[serde(rename = "Queues")]
    pub queues: Queues,

    #[serde(rename = "NextMarker")]
    pub next_marker: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Queues {
    #[serde(rename = "Queue")]
    pub queues: Option<Vec<Queue>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Queue {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Metadata")]
    pub metadata: Option<std::collections::HashMap<String, String>>,
}

impl ListQueuesResponse {
    async fn try_from(response: AzureResponse) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let body = collect_pinned_stream(body).await?;

        let mut response: ListQueuesResponseInternal = read_xml(&body)?;

        // get rid of the ugly Some("") empty string
        // we use None instead
        if let Some(next_marker) = &response.next_marker {
            if next_marker.is_empty() {
                response.next_marker = None;
            }
        }

        Ok(ListQueuesResponse {
            common_storage_response_headers: (&headers).try_into()?,
            service_endpoint: response.service_endpoint,
            prefix: response.prefix,
            marker: response.marker,
            max_results: response.max_results,
            queues: response.queues.queues.unwrap_or_default(),
            next_marker: response.next_marker.map(|nm| nm.into()),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn try_parse() {
        let range = "<?xml version=\"1.0\" encoding=\"utf-8\"?><EnumerationResults ServiceEndpoint=\"https://azureskdforrust.queue.core.windows.net/\"><Prefix>a</Prefix><MaxResults>2</MaxResults><Queues><Queue><Name>azureiscool</Name></Queue><Queue><Name>azurerocks</Name></Queue></Queues><NextMarker /></EnumerationResults>";

        let response: ListQueuesResponseInternal = serde_xml_rs::from_str(range).unwrap();

        assert_eq!(response.queues.queues.unwrap().len(), 2);
    }
}

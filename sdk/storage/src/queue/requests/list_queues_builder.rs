use crate::core::prelude::*;
use crate::queue::clients::QueueAccountClient;
use crate::queue::responses::*;
use crate::queue::HasStorageClient;
use azure_core::errors::AzureError;
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    queue_service: &'a QueueAccountClient<C>,
    prefix: Option<&'a str>,
    next_marker: Option<&'a str>,
    max_results: Option<u32>,
    include_metadata: bool,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    pub(crate) fn new(queue_service: &'a QueueAccountClient<C>) -> Self {
        ListQueuesBuilder {
            queue_service,
            prefix: None,
            next_marker: None,
            max_results: None,
            include_metadata: false,
            timeout: None,
            client_request_id: None,
        }
    }
}

//set mandatory no traits methods
impl<'a, C> PrefixOption<'a> for ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    fn prefix(&self) -> Option<&'a str> {
        self.prefix
    }
}

impl<'a, C> NextMarkerOption<'a> for ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    fn next_marker(&self) -> Option<&'a str> {
        self.next_marker
    }
}

impl<'a, C> MaxResultsOption for ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    fn max_results(&self) -> Option<u32> {
        self.max_results
    }
}

impl<'a, C> IncludeMetadataOption for ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    fn include_metadata(&self) -> bool {
        self.include_metadata
    }
}

impl<'a, C> TimeoutOption for ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C> ClientRequestIdOption<'a> for ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C> PrefixSupport<'a> for ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = Self;

    fn with_prefix(self, prefix: &'a str) -> Self::O {
        ListQueuesBuilder {
            queue_service: self.queue_service,
            prefix: Some(prefix),
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_metadata: self.include_metadata,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> NextMarkerSupport<'a> for ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = Self;

    fn with_next_marker(self, next_marker: &'a str) -> Self::O {
        ListQueuesBuilder {
            queue_service: self.queue_service,
            prefix: self.prefix,
            next_marker: Some(next_marker),
            max_results: self.max_results,
            include_metadata: self.include_metadata,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> MaxResultsSupport for ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = Self;

    fn with_max_results(self, max_results: u32) -> Self::O {
        ListQueuesBuilder {
            queue_service: self.queue_service,
            prefix: self.prefix,
            next_marker: self.next_marker,
            max_results: Some(max_results),
            include_metadata: self.include_metadata,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> IncludeMetadataSupport for ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = Self;

    fn with_include_metadata(self) -> Self::O {
        ListQueuesBuilder {
            queue_service: self.queue_service,
            prefix: self.prefix,
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_metadata: true,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> TimeoutSupport for ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = Self;

    fn with_timeout(self, timeout: u64) -> Self::O {
        ListQueuesBuilder {
            queue_service: self.queue_service,
            prefix: self.prefix,
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_metadata: self.include_metadata,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C> ClientRequestIdSupport<'a> for ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = Self;

    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        ListQueuesBuilder {
            queue_service: self.queue_service,
            prefix: self.prefix,
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_metadata: self.include_metadata,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, C> ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    pub fn queue_service(&self) -> &'a QueueAccountClient<C> {
        self.queue_service
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    pub async fn execute(self) -> Result<ListQueuesResponse, AzureError> {
        let mut url = url::Url::parse(self.queue_service.storage_client().queue_uri())?;

        url.query_pairs_mut().append_pair("comp", "list");

        IncludeMetadataOption::append_pair(&self, &mut url);
        TimeoutOption::append_pair(&self, &mut url);
        MaxResultsOption::append_pair(&self, &mut url);
        NextMarkerOption::append_pair(&self, &mut url);
        PrefixOption::append_pair(&self, &mut url);

        debug!("url == {}", url);

        let perform_request_response = self.queue_service.storage_client().perform_request(
            url.as_str(),
            &http::Method::GET,
            &|mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            Some(&[]),
        )?;

        let (headers, body) = perform_request_response
            .check_status_extract_headers_and_body(StatusCode::OK)
            .await?;

        (&headers, &body as &[u8]).try_into()
    }
}

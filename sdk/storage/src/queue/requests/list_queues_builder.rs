use crate::core::prelude::*;
use crate::queue::clients::QueueAccountClient;
use crate::queue::responses::*;
use crate::queue::HasStorageClient;
use azure_core::errors::AzureError;
use azure_core::headers::add_optional_header;
use azure_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    queue_account_client: &'a QueueAccountClient<C>,
    prefix: Option<&'a str>,
    next_marker: Option<&'a str>,
    max_results: Option<u32>,
    include_metadata: bool,
    timeout: Option<Timeout>,
    client_request_id: Option<ClientRequestId<'a>>,
}

impl<'a, C> ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    pub(crate) fn new(queue_account_client: &'a QueueAccountClient<C>) -> Self {
        ListQueuesBuilder {
            queue_account_client,
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

impl<'a, C> PrefixSupport<'a> for ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    type O = Self;

    fn with_prefix(self, prefix: &'a str) -> Self::O {
        ListQueuesBuilder {
            queue_account_client: self.queue_account_client,
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
            queue_account_client: self.queue_account_client,
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
            queue_account_client: self.queue_account_client,
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
            queue_account_client: self.queue_account_client,
            prefix: self.prefix,
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_metadata: true,
            timeout: self.timeout,
            client_request_id: self.client_request_id,
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> ListQueuesBuilder<'a, C>
where
    C: Client + Clone,
{
    pub fn queue_account_client(&self) -> &'a QueueAccountClient<C> {
        self.queue_account_client
    }

    pub fn timeout(&self) -> &Option<Timeout> {
        &self.timeout
    }

    pub fn with_timeout(self, timeout: Timeout) -> Self {
        Self {
            queue_account_client: self.queue_account_client,
            prefix: self.prefix,
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_metadata: self.include_metadata,
            timeout: Some(timeout),
            client_request_id: self.client_request_id,
        }
    }

    pub fn client_request_id(&self) -> &Option<ClientRequestId<'a>> {
        &self.client_request_id
    }

    pub fn with_client_request_id(self, client_request_id: ClientRequestId<'a>) -> Self {
        Self {
            queue_account_client: self.queue_account_client,
            prefix: self.prefix,
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_metadata: self.include_metadata,
            timeout: self.timeout,
            client_request_id: Some(client_request_id),
        }
    }

    pub async fn execute(self) -> Result<ListQueuesResponse, AzureError> {
        let mut url = url::Url::parse(self.queue_account_client.storage_client().queue_uri())?;

        url.query_pairs_mut().append_pair("comp", "list");

        IncludeMetadataOption::append_to_url(&self, &mut url);
        MaxResultsOption::append_to_url(&self, &mut url);
        NextMarkerOption::append_to_url(&self, &mut url);
        PrefixOption::append_to_url(&self, &mut url);

        AppendToUrlQuery::append_to_url_query(self.timeout(), &mut url);

        debug!("url == {}", url);

        let perform_request_response = self.queue_account_client.storage_client().perform_request(
            url.as_str(),
            &http::Method::GET,
            &|mut request| {
                request = add_optional_header(self.client_request_id(), request);
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

use crate::core::prelude::*;
use crate::queue::prelude::*;
use crate::queue::responses::*;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::prelude::*;
use hyper::StatusCode;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct ListQueuesBuilder<'a, 'b, C>
where
    C: Client,
{
    queue_service: &'a dyn QueueService<StorageClient = C>,
    prefix: Option<&'b str>,
    next_marker: Option<&'b str>,
    max_results: Option<u32>,
    include_metadata: bool,
    timeout: Option<u64>,
    client_request_id: Option<&'a str>,
}

impl<'a, 'b, C> ListQueuesBuilder<'a, 'b, C>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(
        queue_service: &'a dyn QueueService<StorageClient = C>,
    ) -> ListQueuesBuilder<'a, 'b, C> {
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
impl<'a, 'b, C> PrefixOption<'b> for ListQueuesBuilder<'a, 'b, C>
where
    C: Client,
{
    #[inline]
    fn prefix(&self) -> Option<&'b str> {
        self.prefix
    }
}

impl<'a, 'b, C> NextMarkerOption<'b> for ListQueuesBuilder<'a, 'b, C>
where
    C: Client,
{
    #[inline]
    fn next_marker(&self) -> Option<&'b str> {
        self.next_marker
    }
}

impl<'a, 'b, C> MaxResultsOption for ListQueuesBuilder<'a, 'b, C>
where
    C: Client,
{
    #[inline]
    fn max_results(&self) -> Option<u32> {
        self.max_results
    }
}

impl<'a, 'b, C> IncludeMetadataOption for ListQueuesBuilder<'a, 'b, C>
where
    C: Client,
{
    #[inline]
    fn include_metadata(&self) -> bool {
        self.include_metadata
    }
}

impl<'a, 'b, C> TimeoutOption for ListQueuesBuilder<'a, 'b, C>
where
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, 'b, C> ClientRequestIdOption<'a> for ListQueuesBuilder<'a, 'b, C>
where
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, 'b, C> PrefixSupport<'b> for ListQueuesBuilder<'a, 'b, C>
where
    C: Client,
{
    type O = ListQueuesBuilder<'a, 'b, C>;

    #[inline]
    fn with_prefix(self, prefix: &'b str) -> Self::O {
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

impl<'a, 'b, C> NextMarkerSupport<'b> for ListQueuesBuilder<'a, 'b, C>
where
    C: Client,
{
    type O = ListQueuesBuilder<'a, 'b, C>;

    #[inline]
    fn with_next_marker(self, next_marker: &'b str) -> Self::O {
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

impl<'a, 'b, C> MaxResultsSupport for ListQueuesBuilder<'a, 'b, C>
where
    C: Client,
{
    type O = ListQueuesBuilder<'a, 'b, C>;

    #[inline]
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

impl<'a, 'b, C> IncludeMetadataSupport for ListQueuesBuilder<'a, 'b, C>
where
    C: Client,
{
    type O = ListQueuesBuilder<'a, 'b, C>;

    #[inline]
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

impl<'a, 'b, C> TimeoutSupport for ListQueuesBuilder<'a, 'b, C>
where
    C: Client,
{
    type O = ListQueuesBuilder<'a, 'b, C>;

    #[inline]
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

impl<'a, 'b, C> ClientRequestIdSupport<'a> for ListQueuesBuilder<'a, 'b, C>
where
    C: Client,
{
    type O = ListQueuesBuilder<'a, 'b, C>;

    #[inline]
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
impl<'a, 'b, C> ListQueuesBuilder<'a, 'b, C>
where
    C: Client,
{
    pub fn queue_service(&self) -> &'a dyn QueueService<StorageClient = C> {
        self.queue_service
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, 'b, C> ListQueuesBuilder<'a, 'b, C>
where
    C: Client,
{
    pub async fn execute(self) -> Result<ListQueuesResponse, AzureError> {
        let mut uri = format!(
            "{}?comp=list",
            self.queue_service.storage_client().queue_uri()
        );

        if let Some(nm) = IncludeMetadataOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }
        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }
        if let Some(nm) = MaxResultsOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }
        if let Some(nm) = NextMarkerOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }
        if let Some(nm) = PrefixOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        debug!("uri == {}", uri);

        let future_response = self.queue_service.storage_client().perform_request(
            &uri,
            &http::Method::GET,
            &|mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            Some(&[]),
        )?;

        let (headers, body) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;

        (&headers, &body as &[u8]).try_into()
    }
}

use crate::container::incomplete_vector_from_container_response;
use crate::container::responses::ListContainersResponse;
use crate::core::prelude::*;
use azure_core::errors::AzureError;
use azure_core::headers::request_id_from_headers;
use azure_core::prelude::*;
use hyper::{Method, StatusCode};

#[derive(Debug, Clone)]
pub struct ListBuilder<'a, C>
where
    C: Client,
{
    client: &'a C,
    prefix: Option<&'a str>,
    next_marker: Option<&'a str>,
    include_metadata: bool,
    max_results: Option<u32>,
    client_request_id: Option<&'a str>,
    timeout: Option<u64>,
}

impl<'a, C> ListBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> ListBuilder<'a, C> {
        ListBuilder {
            client,
            prefix: None,
            next_marker: None,
            include_metadata: false,
            max_results: None,
            client_request_id: None,
            timeout: None,
        }
    }
}

impl<'a, C> ClientRequired<'a, C> for ListBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C> PrefixOption<'a> for ListBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn prefix(&self) -> Option<&'a str> {
        self.prefix
    }
}

impl<'a, C> NextMarkerOption<'a> for ListBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn next_marker(&self) -> Option<&'a str> {
        self.next_marker
    }
}

impl<'a, C> IncludeMetadataOption for ListBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn include_metadata(&self) -> bool {
        self.include_metadata
    }
}

impl<'a, C> MaxResultsOption for ListBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn max_results(&self) -> Option<u32> {
        self.max_results
    }
}

impl<'a, C> ClientRequestIdOption<'a> for ListBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C> TimeoutOption for ListBuilder<'a, C>
where
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C> PrefixSupport<'a> for ListBuilder<'a, C>
where
    C: Client,
{
    type O = ListBuilder<'a, C>;

    #[inline]
    fn with_prefix(self, prefix: &'a str) -> Self::O {
        ListBuilder {
            client: self.client,
            prefix: Some(prefix),
            next_marker: self.next_marker,
            include_metadata: self.include_metadata,
            max_results: self.max_results,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, C> NextMarkerSupport<'a> for ListBuilder<'a, C>
where
    C: Client,
{
    type O = ListBuilder<'a, C>;

    #[inline]
    fn with_next_marker(self, next_marker: &'a str) -> Self::O {
        ListBuilder {
            client: self.client,
            prefix: self.prefix,
            next_marker: Some(next_marker),
            include_metadata: self.include_metadata,
            max_results: self.max_results,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, C> IncludeMetadataSupport for ListBuilder<'a, C>
where
    C: Client,
{
    type O = ListBuilder<'a, C>;

    #[inline]
    fn with_include_metadata(self) -> Self::O {
        ListBuilder {
            client: self.client,
            prefix: self.prefix,
            next_marker: self.next_marker,
            include_metadata: true,
            max_results: self.max_results,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, C> MaxResultsSupport for ListBuilder<'a, C>
where
    C: Client,
{
    type O = ListBuilder<'a, C>;

    #[inline]
    fn with_max_results(self, max_results: u32) -> Self::O {
        ListBuilder {
            client: self.client,
            prefix: self.prefix,
            next_marker: self.next_marker,
            include_metadata: self.include_metadata,
            max_results: Some(max_results),
            client_request_id: self.client_request_id,
            timeout: self.timeout,
        }
    }
}

impl<'a, C> ClientRequestIdSupport<'a> for ListBuilder<'a, C>
where
    C: Client,
{
    type O = ListBuilder<'a, C>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        ListBuilder {
            client: self.client,
            prefix: self.prefix,
            next_marker: self.next_marker,
            include_metadata: self.include_metadata,
            max_results: self.max_results,
            client_request_id: Some(client_request_id),
            timeout: self.timeout,
        }
    }
}

impl<'a, C> TimeoutSupport for ListBuilder<'a, C>
where
    C: Client,
{
    type O = ListBuilder<'a, C>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        ListBuilder {
            client: self.client,
            prefix: self.prefix,
            next_marker: self.next_marker,
            include_metadata: self.include_metadata,
            max_results: self.max_results,
            client_request_id: self.client_request_id,
            timeout: Some(timeout),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> ListBuilder<'a, C>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<ListContainersResponse, AzureError> {
        let mut uri = format!("{}?comp=list", self.client().blob_uri());

        if self.include_metadata() {
            uri = format!("{}&include=metadata", uri);
        }

        if let Some(mr) = MaxResultsOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, mr);
        }
        if let Some(nm) = NextMarkerOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }
        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }
        if let Some(nm) = PrefixOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        debug!("generated uri = {}", uri);

        let perform_request_response = self.client().perform_request(
            &uri,
            &Method::GET,
            &|request| ClientRequestIdOption::add_header(&self, request),
            None,
        )?;

        let (headers, body) = perform_request_response
            .check_status_extract_headers_and_body(StatusCode::OK)
            .await?;
        let body = std::str::from_utf8(&body)?;
        let incomplete_vector = incomplete_vector_from_container_response(&body)?;
        let request_id = request_id_from_headers(&headers)?;
        Ok(ListContainersResponse {
            incomplete_vector,
            request_id,
        })
    }
}

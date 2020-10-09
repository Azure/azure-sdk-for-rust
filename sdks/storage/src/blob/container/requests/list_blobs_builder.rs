use crate::blob::blob::responses::ListBlobsResponse;
use crate::blob::container::generate_container_uri;
use crate::core::prelude::*;
use azure_sdk_core::errors::{check_status_extract_headers_and_body_as_string, AzureError};
use azure_sdk_core::prelude::*;
use azure_sdk_core::{No, ToAssign, Yes};
use futures::stream::{unfold, Stream};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    container_name: Option<&'a str>,
    client_request_id: Option<&'a str>,
    timeout: Option<u64>,
    prefix: Option<&'a str>,
    delimiter: Option<&'a str>,
    next_marker: Option<&'a str>,
    max_results: Option<u32>,
    include_snapshots: bool,
    include_metadata: bool,
    include_uncommitted_blobs: bool,
    include_copy: bool,
    include_deleted: bool,
}

impl<'a, C> ListBlobBuilder<'a, C, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> ListBlobBuilder<'a, C, No> {
        ListBlobBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            client_request_id: None,
            timeout: None,
            prefix: None,
            delimiter: None,
            next_marker: None,
            max_results: None,
            include_snapshots: false,
            include_metadata: false,
            include_uncommitted_blobs: false,
            include_copy: false,
            include_deleted: false,
        }
    }
}

impl<'a, C, ContainerNameSet> ClientRequired<'a, C> for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C> ContainerNameRequired<'a> for ListBlobBuilder<'a, C, Yes>
where
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet> ClientRequestIdOption<'a> for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, ContainerNameSet> TimeoutOption for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet> PrefixOption<'a> for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn prefix(&self) -> Option<&'a str> {
        self.prefix
    }
}

impl<'a, C, ContainerNameSet> DelimiterOption<'a> for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn delimiter(&self) -> Option<&'a str> {
        self.delimiter
    }
}

impl<'a, C, ContainerNameSet> NextMarkerOption<'a> for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn next_marker(&self) -> Option<&'a str> {
        self.next_marker
    }
}

impl<'a, C, ContainerNameSet> MaxResultsOption for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn max_results(&self) -> Option<u32> {
        self.max_results
    }
}

impl<'a, C, ContainerNameSet> IncludeSnapshotsOption for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn include_snapshots(&self) -> bool {
        self.include_snapshots
    }
}

impl<'a, C, ContainerNameSet> IncludeMetadataOption for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn include_metadata(&self) -> bool {
        self.include_metadata
    }
}

impl<'a, C, ContainerNameSet> IncludeUncommittedBlobsOption
    for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn include_uncommitted_blobs(&self) -> bool {
        self.include_uncommitted_blobs
    }
}

impl<'a, C, ContainerNameSet> IncludeCopyOption for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn include_copy(&self) -> bool {
        self.include_copy
    }
}

impl<'a, C, ContainerNameSet> IncludeDeletedOption for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn include_deleted(&self) -> bool {
        self.include_deleted
    }
}

impl<'a, C> ContainerNameSupport<'a> for ListBlobBuilder<'a, C, No>
where
    C: Client,
{
    type O = ListBlobBuilder<'a, C, Yes>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        ListBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: Some(container_name),
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: self.delimiter,
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, C, ContainerNameSet> ClientRequestIdSupport<'a>
    for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = ListBlobBuilder<'a, C, ContainerNameSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        ListBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: Some(client_request_id),
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: self.delimiter,
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, C, ContainerNameSet> TimeoutSupport for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = ListBlobBuilder<'a, C, ContainerNameSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        ListBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: Some(timeout),
            prefix: self.prefix,
            delimiter: self.delimiter,
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, C, ContainerNameSet> PrefixSupport<'a> for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = ListBlobBuilder<'a, C, ContainerNameSet>;

    #[inline]
    fn with_prefix(self, prefix: &'a str) -> Self::O {
        ListBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: Some(prefix),
            delimiter: self.delimiter,
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, C, ContainerNameSet> DelimiterSupport<'a> for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = ListBlobBuilder<'a, C, ContainerNameSet>;

    #[inline]
    fn with_delimiter(self, delimiter: &'a str) -> Self::O {
        ListBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: Some(delimiter),
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, C, ContainerNameSet> NextMarkerSupport<'a> for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = ListBlobBuilder<'a, C, ContainerNameSet>;

    #[inline]
    fn with_next_marker(self, next_marker: &'a str) -> Self::O {
        ListBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: self.delimiter,
            next_marker: Some(next_marker),
            max_results: self.max_results,
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, C, ContainerNameSet> MaxResultsSupport for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = ListBlobBuilder<'a, C, ContainerNameSet>;

    #[inline]
    fn with_max_results(self, max_results: u32) -> Self::O {
        ListBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: self.delimiter,
            next_marker: self.next_marker,
            max_results: Some(max_results),
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, C, ContainerNameSet> IncludeSnapshotsSupport for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = ListBlobBuilder<'a, C, ContainerNameSet>;

    #[inline]
    fn with_include_snapshots(self) -> Self::O {
        ListBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: self.delimiter,
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_snapshots: true,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, C, ContainerNameSet> IncludeMetadataSupport for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = ListBlobBuilder<'a, C, ContainerNameSet>;

    #[inline]
    fn with_include_metadata(self) -> Self::O {
        ListBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: self.delimiter,
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_snapshots: self.include_snapshots,
            include_metadata: true,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, C, ContainerNameSet> IncludeUncommittedBlobsSupport
    for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = ListBlobBuilder<'a, C, ContainerNameSet>;

    #[inline]
    fn with_include_uncommitted_blobs(self) -> Self::O {
        ListBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: self.delimiter,
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: true,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, C, ContainerNameSet> IncludeCopySupport for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = ListBlobBuilder<'a, C, ContainerNameSet>;

    #[inline]
    fn with_include_copy(self) -> Self::O {
        ListBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: self.delimiter,
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: true,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, C, ContainerNameSet> IncludeDeletedSupport for ListBlobBuilder<'a, C, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = ListBlobBuilder<'a, C, ContainerNameSet>;

    #[inline]
    fn with_include_deleted(self) -> Self::O {
        ListBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: self.delimiter,
            next_marker: self.next_marker,
            max_results: self.max_results,
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: true,
        }
    }
}

impl<'a, C> IncludeListOptions for ListBlobBuilder<'a, C, Yes> where C: Client {}

impl<'a, C> ListBlobBuilder<'a, C, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<ListBlobsResponse, AzureError> {
        // we create a copy to move into the future's closure.
        // We need to do this since the closure only accepts
        // 'static lifetimes.
        let container_name = self.container_name().to_owned();

        let mut uri = generate_container_uri(
            self.client(),
            self.container_name(),
            Some("restype=container&comp=list"),
        );

        if let Some(mr) = MaxResultsOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, mr);
        }
        if let Some(mr) = NextMarkerOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, mr);
        }
        if let Some(mr) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, mr);
        }
        if let Some(mr) = PrefixOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, mr);
        }
        if let Some(mr) = IncludeListOptions::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, mr);
        }

        trace!("list blob uri = {}", uri);

        let future_response =
            self.client()
                .perform_request(&uri, &Method::GET, &|request| request, None)?;

        let (headers, body_as_str) =
            check_status_extract_headers_and_body_as_string(future_response, StatusCode::OK)
                .await?;
        ListBlobsResponse::from_response(&container_name, &headers, &body_as_str)
    }
}

impl<'a, C> ListBlobBuilder<'a, C, Yes>
where
    C: Client + Clone,
{
    pub fn stream(self) -> impl Stream<Item = Result<ListBlobsResponse, AzureError>> + 'a {
        #[derive(Debug, Clone, PartialEq)]
        enum States {
            Init,
            NextMarker(String),
        };

        unfold(Some(States::Init), move |next_marker: Option<States>| {
            let req = self.clone();
            async move {
                debug!("next_marker == {:?}", &next_marker);
                let response = match next_marker {
                    Some(States::Init) => req.finalize().await,
                    Some(States::NextMarker(next_marker)) => {
                        req.with_next_marker(&next_marker).finalize().await
                    }
                    None => return None,
                };

                // the ? operator does not work in async move (yet?)
                // so we have to resort to this boilerplate
                let response = match response {
                    Ok(response) => response,
                    Err(err) => return Some((Err(err), None)),
                };

                let next_marker = match response.incomplete_vector.token() {
                    Some(ct) => Some(States::NextMarker(ct.to_owned())),
                    None => None,
                };

                Some((Ok(response), next_marker))
            }
        })
    }
}

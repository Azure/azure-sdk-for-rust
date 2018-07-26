use azure::core::errors::{check_status_extract_headers_and_body_as_string, AzureError};
use azure::core::{
    request_id_from_headers, ClientRequestIdOption, ClientRequestIdSupport, ClientRequired, ContainerNameRequired, ContainerNameSupport,
    DelimiterOption, DelimiterSupport, IncludeCopyOption, IncludeCopySupport, IncludeDeletedOption, IncludeDeletedSupport,
    IncludeListOptions, IncludeMetadataOption, IncludeMetadataSupport, IncludeSnapshotsOption, IncludeSnapshotsSupport,
    IncludeUncommittedBlobsOption, IncludeUncommittedBlobsSupport, MaxResultsOption, MaxResultsSupport, NextMarkerOption,
    NextMarkerSupport, No, PrefixOption, PrefixSupport, TimeoutOption, TimeoutSupport, ToAssign, Yes,
};
use azure::storage::blob::incomplete_vector_from_response;
use azure::storage::blob::responses::ListBlobsResponse;
use azure::storage::client::Client;
use futures::future::done;
use futures::prelude::*;
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    client: &'a Client,
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

impl<'a> ListBlobBuilder<'a, No> {
    pub(crate) fn new(client: &'a Client) -> ListBlobBuilder<'a, No> {
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

impl<'a, ContainerNameSet> ClientRequired<'a> for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a> ContainerNameRequired<'a> for ListBlobBuilder<'a, Yes> {
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet> ClientRequestIdOption<'a> for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet> TimeoutOption for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet> PrefixOption<'a> for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn prefix(&self) -> Option<&'a str> {
        self.prefix
    }
}

impl<'a, ContainerNameSet> DelimiterOption<'a> for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn delimiter(&self) -> Option<&'a str> {
        self.delimiter
    }
}

impl<'a, ContainerNameSet> NextMarkerOption<'a> for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn next_marker(&self) -> Option<&'a str> {
        self.next_marker
    }
}

impl<'a, ContainerNameSet> MaxResultsOption for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn max_results(&self) -> Option<u32> {
        self.max_results
    }
}

impl<'a, ContainerNameSet> IncludeSnapshotsOption for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn include_snapshots(&self) -> bool {
        self.include_snapshots
    }
}

impl<'a, ContainerNameSet> IncludeMetadataOption for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn include_metadata(&self) -> bool {
        self.include_metadata
    }
}

impl<'a, ContainerNameSet> IncludeUncommittedBlobsOption for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn include_uncommitted_blobs(&self) -> bool {
        self.include_uncommitted_blobs
    }
}

impl<'a, ContainerNameSet> IncludeCopyOption for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn include_copy(&self) -> bool {
        self.include_copy
    }
}

impl<'a, ContainerNameSet> IncludeDeletedOption for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    fn include_deleted(&self) -> bool {
        self.include_deleted
    }
}

impl<'a, ContainerNameSet> ContainerNameSupport<'a> for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobBuilder<'a, Yes>;

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

impl<'a, ContainerNameSet> ClientRequestIdSupport<'a> for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobBuilder<'a, ContainerNameSet>;

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

impl<'a, ContainerNameSet> TimeoutSupport for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobBuilder<'a, ContainerNameSet>;

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

impl<'a, ContainerNameSet> PrefixSupport<'a> for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobBuilder<'a, ContainerNameSet>;

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

impl<'a, ContainerNameSet> DelimiterSupport<'a> for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobBuilder<'a, ContainerNameSet>;

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

impl<'a, ContainerNameSet> NextMarkerSupport<'a> for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobBuilder<'a, ContainerNameSet>;

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

impl<'a, ContainerNameSet> MaxResultsSupport for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobBuilder<'a, ContainerNameSet>;

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

impl<'a, ContainerNameSet> IncludeSnapshotsSupport for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobBuilder<'a, ContainerNameSet>;

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

impl<'a, ContainerNameSet> IncludeMetadataSupport for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobBuilder<'a, ContainerNameSet>;

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

impl<'a, ContainerNameSet> IncludeUncommittedBlobsSupport for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobBuilder<'a, ContainerNameSet>;

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

impl<'a, ContainerNameSet> IncludeCopySupport for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobBuilder<'a, ContainerNameSet>;

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

impl<'a, ContainerNameSet> IncludeDeletedSupport for ListBlobBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobBuilder<'a, ContainerNameSet>;

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

// methods callable regardless
impl<'a, ContainerNameSet> ListBlobBuilder<'a, ContainerNameSet> where ContainerNameSet: ToAssign {}

// this empty trait is required in order to use IncludeListOptions methods. No duck typing, sorry
// :(
impl<'a> IncludeListOptions for ListBlobBuilder<'a, Yes> {}

impl<'a> ListBlobBuilder<'a, Yes> {
    pub fn finalize(self) -> impl Future<Item = ListBlobsResponse, Error = AzureError> {
        // we create a copy to move into the future's closure.
        // We need to do this since the closure only accepts
        // 'static lifetimes.
        let container_name = self.container_name().to_owned();

        let mut uri = format!(
            "https://{}.blob.core.windows.\
             net/{}?restype=container&comp=list",
            self.client().account(),
            container_name
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

        let req = self.client().perform_request(&uri, Method::GET, |_| {}, None);

        done(req).from_err().and_then(move |future_response| {
            check_status_extract_headers_and_body_as_string(future_response, StatusCode::OK).and_then(move |(headers, body)| {
                done(incomplete_vector_from_response(&body, &container_name)).and_then(move |incomplete_vector| {
                    done(request_id_from_headers(&headers).map(move |request_id| ListBlobsResponse {
                        incomplete_vector,
                        request_id,
                    }))
                })
            })
        })
    }
}

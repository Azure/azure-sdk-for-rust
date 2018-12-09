use crate::azure::core::errors::AzureError;
use crate::azure::core::incompletevector::IncompleteVector;
use crate::azure::core::{
    ClientRequestIdOption, ClientRequestIdSupport, ClientRequired, ContainerNameRequired, ContainerNameSupport, DelimiterOption,
    DelimiterSupport, IncludeCopyOption, IncludeCopySupport, IncludeDeletedOption, IncludeDeletedSupport, IncludeListOptions,
    IncludeMetadataOption, IncludeMetadataSupport, IncludeSnapshotsOption, IncludeSnapshotsSupport, IncludeUncommittedBlobsOption,
    IncludeUncommittedBlobsSupport, NextMarkerSupport, No, PrefixOption, PrefixSupport, TimeoutOption, TimeoutSupport, ToAssign, Yes,
};
use crate::azure::storage::blob::requests::ListBlobBuilder;
use crate::azure::storage::blob::Blob;
use crate::azure::storage::client::Client;
use futures::prelude::*;
use futures::stream;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct ListBlobStreamBuilder<'a, ContainerNameSet>
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
    include_snapshots: bool,
    include_metadata: bool,
    include_uncommitted_blobs: bool,
    include_copy: bool,
    include_deleted: bool,
}

impl<'a> ListBlobStreamBuilder<'a, No> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> ListBlobStreamBuilder<'a, No> {
        ListBlobStreamBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            client_request_id: None,
            timeout: None,
            prefix: None,
            delimiter: None,
            include_snapshots: false,
            include_metadata: false,
            include_uncommitted_blobs: false,
            include_copy: false,
            include_deleted: false,
        }
    }
}

impl<'a, ContainerNameSet> ClientRequired<'a> for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a> ContainerNameRequired<'a> for ListBlobStreamBuilder<'a, Yes> {
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet> ClientRequestIdOption<'a> for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet> TimeoutOption for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet> PrefixOption<'a> for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    #[inline]
    fn prefix(&self) -> Option<&'a str> {
        self.prefix
    }
}

impl<'a, ContainerNameSet> DelimiterOption<'a> for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    #[inline]
    fn delimiter(&self) -> Option<&'a str> {
        self.delimiter
    }
}

impl<'a, ContainerNameSet> IncludeSnapshotsOption for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    #[inline]
    fn include_snapshots(&self) -> bool {
        self.include_snapshots
    }
}

impl<'a, ContainerNameSet> IncludeMetadataOption for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    #[inline]
    fn include_metadata(&self) -> bool {
        self.include_metadata
    }
}

impl<'a, ContainerNameSet> IncludeUncommittedBlobsOption for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    #[inline]
    fn include_uncommitted_blobs(&self) -> bool {
        self.include_uncommitted_blobs
    }
}

impl<'a, ContainerNameSet> IncludeCopyOption for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    #[inline]
    fn include_copy(&self) -> bool {
        self.include_copy
    }
}

impl<'a, ContainerNameSet> IncludeDeletedOption for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    #[inline]
    fn include_deleted(&self) -> bool {
        self.include_deleted
    }
}

impl<'a, ContainerNameSet> ContainerNameSupport<'a> for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobStreamBuilder<'a, Yes>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        ListBlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: Some(container_name),
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: self.delimiter,
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, ContainerNameSet> ClientRequestIdSupport<'a> for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobStreamBuilder<'a, ContainerNameSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        ListBlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: Some(client_request_id),
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: self.delimiter,
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, ContainerNameSet> TimeoutSupport for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobStreamBuilder<'a, ContainerNameSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        ListBlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: Some(timeout),
            prefix: self.prefix,
            delimiter: self.delimiter,
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, ContainerNameSet> PrefixSupport<'a> for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobStreamBuilder<'a, ContainerNameSet>;

    #[inline]
    fn with_prefix(self, prefix: &'a str) -> Self::O {
        ListBlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: Some(prefix),
            delimiter: self.delimiter,
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, ContainerNameSet> DelimiterSupport<'a> for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobStreamBuilder<'a, ContainerNameSet>;

    #[inline]
    fn with_delimiter(self, delimiter: &'a str) -> Self::O {
        ListBlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: Some(delimiter),
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, ContainerNameSet> IncludeSnapshotsSupport for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobStreamBuilder<'a, ContainerNameSet>;

    #[inline]
    fn with_include_snapshots(self) -> Self::O {
        ListBlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: self.delimiter,
            include_snapshots: true,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, ContainerNameSet> IncludeMetadataSupport for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobStreamBuilder<'a, ContainerNameSet>;

    #[inline]
    fn with_include_metadata(self) -> Self::O {
        ListBlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: self.delimiter,
            include_snapshots: self.include_snapshots,
            include_metadata: true,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, ContainerNameSet> IncludeUncommittedBlobsSupport for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobStreamBuilder<'a, ContainerNameSet>;

    #[inline]
    fn with_include_uncommitted_blobs(self) -> Self::O {
        ListBlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: self.delimiter,
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: true,
            include_copy: self.include_copy,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, ContainerNameSet> IncludeCopySupport for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobStreamBuilder<'a, ContainerNameSet>;

    #[inline]
    fn with_include_copy(self) -> Self::O {
        ListBlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: self.delimiter,
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: true,
            include_deleted: self.include_deleted,
        }
    }
}

impl<'a, ContainerNameSet> IncludeDeletedSupport for ListBlobStreamBuilder<'a, ContainerNameSet>
where
    ContainerNameSet: ToAssign,
{
    type O = ListBlobStreamBuilder<'a, ContainerNameSet>;

    #[inline]
    fn with_include_deleted(self) -> Self::O {
        ListBlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            container_name: self.container_name,
            client_request_id: self.client_request_id,
            timeout: self.timeout,
            prefix: self.prefix,
            delimiter: self.delimiter,
            include_snapshots: self.include_snapshots,
            include_metadata: self.include_metadata,
            include_uncommitted_blobs: self.include_uncommitted_blobs,
            include_copy: self.include_copy,
            include_deleted: true,
        }
    }
}

// methods callable regardless
impl<'a, ContainerNameSet> ListBlobStreamBuilder<'a, ContainerNameSet> where ContainerNameSet: ToAssign {}

// this empty trait is required in order to use IncludeListOptions methods. No duck typing, sorry
// :(
impl<'a> IncludeListOptions for ListBlobStreamBuilder<'a, Yes> {}

enum ContinuationState {
    Start,
    Next(Option<String>),
}

impl<'a> ListBlobStreamBuilder<'a, Yes> {
    #[inline]
    pub fn finalize(self) -> impl Stream<Item = Blob, Error = AzureError> {
        let container_name = self.container_name().to_owned();

        let client_request_id = self.client_request_id.map(|v| v.to_owned());
        let timeout = self.timeout.to_owned();
        let prefix = self.prefix.map(|v| v.to_owned());
        let delimiter = self.delimiter.map(|v| v.to_owned());
        let include_snapshots = self.include_snapshots;
        let include_metadata = self.include_metadata;
        let include_uncommitted_blobs = self.include_uncommitted_blobs;
        let include_copy = self.include_copy;
        let include_deleted = self.include_deleted;

        let client = self.client().clone();

        stream::unfold(ContinuationState::Start, move |cont_token| {
            let marker = match cont_token {
                ContinuationState::Start => None,
                ContinuationState::Next(Some(marker)) => Some(marker),
                ContinuationState::Next(None) => return None,
            };

            let mut req = ListBlobBuilder::new(&client).with_container_name(&container_name);

            if let Some(ref marker) = &marker {
                req = req.with_next_marker(&*marker);
            }
            if let Some(ref client_request_id) = &client_request_id {
                req = req.with_client_request_id(client_request_id);
            }
            if let Some(timeout) = timeout {
                req = req.with_timeout(timeout);
            }
            if let Some(ref prefix) = &prefix {
                req = req.with_prefix(prefix);
            }
            if let Some(ref delimiter) = &delimiter {
                req = req.with_delimiter(delimiter);
            }

            if include_snapshots {
                req = req.with_include_snapshots();
            }
            if include_metadata {
                req = req.with_include_metadata();
            }
            if include_uncommitted_blobs {
                req = req.with_include_uncommitted_blobs();
            }
            if include_copy {
                req = req.with_include_copy();
            }
            if include_deleted {
                req = req.with_include_deleted();
            }

            let req = req.finalize();
            Some(req.map(move |response| {
                let IncompleteVector { token, vector } = response.incomplete_vector;
                (stream::iter_ok(vector), ContinuationState::Next(token))
            }))
        })
        .flatten()
    }
}

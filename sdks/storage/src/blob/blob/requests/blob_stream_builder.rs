use crate::blob::blob::requests::GetBlobBuilder;
use crate::core::prelude::*;
use azure_core::errors::AzureError;
use azure_core::lease::LeaseId;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use chrono::{DateTime, Utc};
use futures::stream::Stream;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_range: PhantomData<RangeSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    range: Option<&'a Range>,
    snapshot: Option<DateTime<Utc>>,
    timeout: Option<u64>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<&'a str>,
    chunk_size: u64,
}

impl<'a, C> BlobStreamBuilder<'a, C, No, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> BlobStreamBuilder<'a, C, No, No, No> {
        BlobStreamBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_range: PhantomData {},
            range: None,
            snapshot: None,
            timeout: None,
            lease_id: None,
            client_request_id: None,
            chunk_size: 1048576,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, RangeSet> ClientRequired<'a, C>
    for BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, BlobNameSet, RangeSet> ContainerNameRequired<'a>
    for BlobStreamBuilder<'a, C, Yes, BlobNameSet, RangeSet>
where
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, RangeSet> BlobNameRequired<'a>
    for BlobStreamBuilder<'a, C, ContainerNameSet, Yes, RangeSet>
where
    ContainerNameSet: ToAssign,
    RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> RangeRequired<'a>
    for BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn range(&self) -> &'a Range {
        self.range.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, RangeSet> SnapshotOption
    for BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn snapshot(&self) -> Option<DateTime<Utc>> {
        self.snapshot
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, RangeSet> TimeoutOption
    for BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, RangeSet> LeaseIdOption<'a>
    for BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, RangeSet> ClientRequestIdOption<'a>
    for BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, RangeSet> ChunkSizeOption
    for BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
    C: Client,
{
    #[inline]
    fn chunk_size(&self) -> u64 {
        self.chunk_size
    }
}

impl<'a, C, BlobNameSet, RangeSet> ContainerNameSupport<'a>
    for BlobStreamBuilder<'a, C, No, BlobNameSet, RangeSet>
where
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
    C: Client,
{
    type O = BlobStreamBuilder<'a, C, Yes, BlobNameSet, RangeSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        BlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_range: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            range: self.range,
            snapshot: self.snapshot,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
            chunk_size: self.chunk_size,
        }
    }
}

impl<'a, C, ContainerNameSet, RangeSet> BlobNameSupport<'a>
    for BlobStreamBuilder<'a, C, ContainerNameSet, No, RangeSet>
where
    ContainerNameSet: ToAssign,
    RangeSet: ToAssign,
    C: Client,
{
    type O = BlobStreamBuilder<'a, C, ContainerNameSet, Yes, RangeSet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        BlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_range: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            range: self.range,
            snapshot: self.snapshot,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
            chunk_size: self.chunk_size,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> RangeSupport<'a>
    for BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, No>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>;

    #[inline]
    fn with_range(self, range: &'a Range) -> Self::O {
        BlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            range: Some(range),
            snapshot: self.snapshot,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
            chunk_size: self.chunk_size,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, RangeSet> SnapshotSupport
    for BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
    C: Client,
{
    type O = BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>;

    #[inline]
    fn with_snapshot(self, snapshot: DateTime<Utc>) -> Self::O {
        BlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            range: self.range,
            snapshot: Some(snapshot),
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
            chunk_size: self.chunk_size,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, RangeSet> TimeoutSupport
    for BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
    C: Client,
{
    type O = BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        BlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            range: self.range,
            snapshot: self.snapshot,
            timeout: Some(timeout),
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
            chunk_size: self.chunk_size,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, RangeSet> LeaseIdSupport<'a>
    for BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
    C: Client,
{
    type O = BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        BlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            range: self.range,
            snapshot: self.snapshot,
            timeout: self.timeout,
            lease_id: Some(lease_id),
            client_request_id: self.client_request_id,
            chunk_size: self.chunk_size,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, RangeSet> ClientRequestIdSupport<'a>
    for BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
    C: Client,
{
    type O = BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        BlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            range: self.range,
            snapshot: self.snapshot,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: Some(client_request_id),
            chunk_size: self.chunk_size,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, RangeSet> ChunkSizeSupport
    for BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
    C: Client,
{
    type O = BlobStreamBuilder<'a, C, ContainerNameSet, BlobNameSet, RangeSet>;

    #[inline]
    fn with_chunk_size(self, chunk_size: u64) -> Self::O {
        BlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            range: self.range,
            snapshot: self.snapshot,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
            chunk_size,
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> BlobStreamBuilder<'a, C, Yes, Yes, Yes>
where
    C: Client + Clone,
{
    #[inline]
    pub fn finalize(self) -> impl Stream<Item = Result<Vec<u8>, AzureError>> + 'a {
        let client = self.client().clone();
        let container_name = self.container_name();
        let client_request_id = self.client_request_id.map(|v| v.to_owned());
        let blob_name = self.blob_name();
        let range = self.range().to_owned();
        let snapshot = self.snapshot.to_owned();
        let timeout = self.timeout.to_owned();
        let lease_id = self.lease_id.cloned();
        let chunk_size = self.chunk_size;

        futures::stream::unfold(Some(range), move |remaining| {
            let client = client.clone();
            let client_request_id = client_request_id.clone();

            async move {
                let remaining = match remaining {
                    Some(range) => range,
                    None => return None,
                };

                let range = if remaining.start + chunk_size > remaining.end {
                    Range::new(remaining.start, remaining.end)
                } else {
                    Range::new(remaining.start, remaining.start + chunk_size)
                };

                let mut req = GetBlobBuilder::new(&client)
                    .with_container_name(&container_name)
                    .with_blob_name(&blob_name)
                    .with_range(&range);

                if let Some(snapshot) = snapshot {
                    req = req.with_snapshot(snapshot);
                }
                if let Some(timeout) = timeout {
                    req = req.with_timeout(timeout);
                }
                if let Some(ref lease_id) = &lease_id {
                    req = req.with_lease_id(lease_id);
                }
                if let Some(ref client_request_id) = &client_request_id {
                    req = req.with_client_request_id(client_request_id);
                }

                let response = match req.finalize().await {
                    Ok(response) => response,
                    Err(err) => return Some((Err(err), None)),
                };

                Some((
                    Ok(response.data),
                    if remaining.end > range.end {
                        Some(Range::new(range.end, remaining.end))
                    } else {
                        None
                    },
                ))
            }
        })
    }
}

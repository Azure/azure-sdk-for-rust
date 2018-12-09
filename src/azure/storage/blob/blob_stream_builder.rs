use crate::azure::core::errors::AzureError;
use crate::azure::core::lease::LeaseId;
use crate::azure::core::range::Range;
use crate::azure::core::{
    BlobNameRequired, BlobNameSupport, ClientRequestIdOption, ClientRequestIdSupport, ClientRequired, ContainerNameRequired,
    ContainerNameSupport, LeaseIdOption, LeaseIdSupport, No, RangeRequired, RangeSupport, SnapshotOption, SnapshotSupport, TimeoutOption,
    TimeoutSupport, ToAssign, Yes,
};
use crate::azure::storage::blob::requests::GetBlobBuilder;
use crate::azure::storage::client::Client;
use chrono::{DateTime, Utc};
use futures::prelude::*;
use futures::stream;
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_range: PhantomData<RangeSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    snapshot: Option<DateTime<Utc>>,
    timeout: Option<u64>,
    range: Option<&'a Range>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<&'a str>,
    increment: u64,
}

impl<'a> BlobStreamBuilder<'a, No, No, No> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> BlobStreamBuilder<'a, No, No, No> {
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
            increment: 1024 * 1024,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, RangeSet> ClientRequired<'a> for BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet, RangeSet> ContainerNameRequired<'a> for BlobStreamBuilder<'a, Yes, BlobNameSet, RangeSet>
where
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, RangeSet> BlobNameRequired<'a> for BlobStreamBuilder<'a, ContainerNameSet, Yes, RangeSet>
where
    ContainerNameSet: ToAssign,
    RangeSet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> RangeRequired<'a> for BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn range(&self) -> &'a Range {
        self.range.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet, RangeSet> SnapshotOption for BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
{
    #[inline]
    fn snapshot(&self) -> Option<DateTime<Utc>> {
        self.snapshot
    }
}

impl<'a, ContainerNameSet, BlobNameSet, RangeSet> TimeoutOption for BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet, RangeSet> LeaseIdOption<'a> for BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, RangeSet> ClientRequestIdOption<'a>
    for BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, RangeSet> ContainerNameSupport<'a>
    for BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
{
    type O = BlobStreamBuilder<'a, Yes, BlobNameSet, RangeSet>;

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
            increment: self.increment,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, RangeSet> BlobNameSupport<'a> for BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
{
    type O = BlobStreamBuilder<'a, ContainerNameSet, Yes, RangeSet>;

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
            increment: self.increment,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, RangeSet> RangeSupport<'a> for BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
{
    type O = BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, Yes>;

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
            increment: self.increment,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, RangeSet> SnapshotSupport for BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
{
    type O = BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>;

    #[inline]
    fn with_snapshot(self, snapshot: DateTime<Utc>) -> Self::O {
        BlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: Some(snapshot),
            timeout: self.timeout,
            range: self.range,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
            increment: self.increment,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, RangeSet> TimeoutSupport for BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
{
    type O = BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        BlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: Some(timeout),
            range: self.range,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
            increment: self.increment,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, RangeSet> LeaseIdSupport<'a> for BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
{
    type O = BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        BlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: self.timeout,
            range: self.range,
            lease_id: Some(lease_id),
            client_request_id: self.client_request_id,
            increment: self.increment,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, RangeSet> ClientRequestIdSupport<'a>
    for BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
{
    type O = BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        BlobStreamBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_range: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: self.timeout,
            range: self.range,
            lease_id: self.lease_id,
            client_request_id: Some(client_request_id),
            increment: self.increment,
        }
    }
}

// methods callable regardless
impl<'a, ContainerNameSet, BlobNameSet, RangeSet> BlobStreamBuilder<'a, ContainerNameSet, BlobNameSet, RangeSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    RangeSet: ToAssign,
{
}

impl<'a> BlobStreamBuilder<'a, Yes, Yes, Yes> {
    #[inline]
    pub fn finalize(self) -> impl Stream<Item = Vec<u8>, Error = AzureError> {
        let container_name = self.container_name().to_owned();
        let blob_name = self.blob_name().to_owned();
        let range = self.range().to_owned();

        let snapshot = self.snapshot.to_owned();
        let timeout = self.timeout.to_owned();
        let lease_id = self.lease_id.cloned();
        let client_request_id = self.client_request_id.map(|v| v.to_owned());
        let increment = self.increment;

        let client = self.client().clone();

        stream::unfold(Some(range), move |remaining| {
            let remaining = match remaining {
                Some(range) => range,
                None => return None,
            };

            let range = if remaining.start + increment > remaining.end {
                Range::new(remaining.start, remaining.end)
            } else {
                Range::new(remaining.start, remaining.start + increment)
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

            let req = req.finalize();
            Some(req.map(move |response| {
                (
                    response.data,
                    if remaining.end > range.end {
                        Some(Range::new(range.end, remaining.end))
                    } else {
                        None
                    },
                )
            }))
        })
    }
}

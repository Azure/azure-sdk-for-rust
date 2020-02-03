use crate::blob::generate_blob_uri;
use crate::blob::responses::DeleteBlobResponse;
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::{
    BlobNameRequired, BlobNameSupport, ClientRequestIdOption, ClientRequestIdSupport,
    ContainerNameRequired, ContainerNameSupport, LeaseIdOption, LeaseIdSupport, SnapshotRequired,
    SnapshotSupport, TimeoutOption, TimeoutSupport,
};
use azure_sdk_core::{No, ToAssign, Yes};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use chrono::{DateTime, Utc};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
{
    client: &'a Client,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_snapshot: PhantomData<SnapshotSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    snapshot: Option<DateTime<Utc>>,
    timeout: Option<u64>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<&'a str>,
}

impl<'a> DeleteBlobSnapshotBuilder<'a, No, No, No> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> DeleteBlobSnapshotBuilder<'a, No, No, No> {
        DeleteBlobSnapshotBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_snapshot: PhantomData {},
            snapshot: None,
            timeout: None,
            lease_id: None,
            client_request_id: None,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SnapshotSet> ClientRequired<'a>
    for DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet, SnapshotSet> ContainerNameRequired<'a>
    for DeleteBlobSnapshotBuilder<'a, Yes, BlobNameSet, SnapshotSet>
where
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet, SnapshotSet> BlobNameRequired<'a>
    for DeleteBlobSnapshotBuilder<'a, ContainerNameSet, Yes, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    SnapshotSet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> SnapshotRequired
    for DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn snapshot(&self) -> DateTime<Utc> {
        self.snapshot.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SnapshotSet> TimeoutOption
    for DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SnapshotSet> LeaseIdOption<'a>
    for DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SnapshotSet> ClientRequestIdOption<'a>
    for DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SnapshotSet> ContainerNameSupport<'a>
    for DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
{
    type O = DeleteBlobSnapshotBuilder<'a, Yes, BlobNameSet, SnapshotSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        DeleteBlobSnapshotBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_snapshot: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SnapshotSet> BlobNameSupport<'a>
    for DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
{
    type O = DeleteBlobSnapshotBuilder<'a, ContainerNameSet, Yes, SnapshotSet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        DeleteBlobSnapshotBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_snapshot: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            snapshot: self.snapshot,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SnapshotSet> SnapshotSupport
    for DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
{
    type O = DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, Yes>;

    #[inline]
    fn with_snapshot(self, snapshot: DateTime<Utc>) -> Self::O {
        DeleteBlobSnapshotBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_snapshot: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: Some(snapshot),
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SnapshotSet> TimeoutSupport
    for DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
{
    type O = DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, SnapshotSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        DeleteBlobSnapshotBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_snapshot: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: Some(timeout),
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SnapshotSet> LeaseIdSupport<'a>
    for DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
{
    type O = DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, SnapshotSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        DeleteBlobSnapshotBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_snapshot: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: self.timeout,
            lease_id: Some(lease_id),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, ContainerNameSet, BlobNameSet, SnapshotSet> ClientRequestIdSupport<'a>
    for DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
{
    type O = DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, SnapshotSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        DeleteBlobSnapshotBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_snapshot: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable regardless
impl<'a, ContainerNameSet, BlobNameSet, SnapshotSet>
    DeleteBlobSnapshotBuilder<'a, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
{
}

impl<'a> DeleteBlobSnapshotBuilder<'a, Yes, Yes, Yes> {
    pub async fn finalize(self) -> Result<DeleteBlobResponse, AzureError> {
        let mut uri = generate_blob_uri(&self, Some(&SnapshotRequired::to_uri_parameter(&self)));

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let future_response = self.client().perform_request(
            &uri,
            &Method::DELETE,
            |mut request| {
                request = LeaseIdOption::add_header(&self, request);
                request = ClientRequestIdOption::add_header(&self, request);
                request
            },
            None,
        )?;

        let (headers, _body) =
            check_status_extract_headers_and_body(future_response, StatusCode::ACCEPTED).await?;
        DeleteBlobResponse::from_headers(&headers)
    }
}

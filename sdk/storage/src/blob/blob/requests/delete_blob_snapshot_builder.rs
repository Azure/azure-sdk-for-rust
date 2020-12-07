use crate::blob::blob::generate_blob_uri;
use crate::blob::blob::responses::DeleteBlobResponse;
use crate::core::prelude::*;
use azure_core::errors::AzureError;
use azure_core::lease::LeaseId;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use chrono::{DateTime, Utc};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
    C: Client,
{
    client: &'a C,
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

impl<'a, C> DeleteBlobSnapshotBuilder<'a, C, No, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> DeleteBlobSnapshotBuilder<'a, C, No, No, No> {
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

impl<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet> ClientRequired<'a, C>
    for DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, BlobNameSet, SnapshotSet> ContainerNameRequired<'a>
    for DeleteBlobSnapshotBuilder<'a, C, Yes, BlobNameSet, SnapshotSet>
where
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, SnapshotSet> BlobNameRequired<'a>
    for DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, Yes, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    SnapshotSet: ToAssign,
    C: Client,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> SnapshotRequired
    for DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn snapshot(&self) -> DateTime<Utc> {
        self.snapshot.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet> TimeoutOption
    for DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet> LeaseIdOption<'a>
    for DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet> ClientRequestIdOption<'a>
    for DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, BlobNameSet, SnapshotSet> ContainerNameSupport<'a>
    for DeleteBlobSnapshotBuilder<'a, C, No, BlobNameSet, SnapshotSet>
where
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
    C: Client,
{
    type O = DeleteBlobSnapshotBuilder<'a, C, Yes, BlobNameSet, SnapshotSet>;

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

impl<'a, C, ContainerNameSet, SnapshotSet> BlobNameSupport<'a>
    for DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, No, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    SnapshotSet: ToAssign,
    C: Client,
{
    type O = DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, Yes, SnapshotSet>;

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

impl<'a, C, ContainerNameSet, BlobNameSet> SnapshotSupport
    for DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, BlobNameSet, No>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>;

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

impl<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet> TimeoutSupport
    for DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
    C: Client,
{
    type O = DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet>;

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

impl<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet> LeaseIdSupport<'a>
    for DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
    C: Client,
{
    type O = DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet>;

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

impl<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet> ClientRequestIdSupport<'a>
    for DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    SnapshotSet: ToAssign,
    C: Client,
{
    type O = DeleteBlobSnapshotBuilder<'a, C, ContainerNameSet, BlobNameSet, SnapshotSet>;

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

// methods callable only when every mandatory field has been filled
impl<'a, C> DeleteBlobSnapshotBuilder<'a, C, Yes, Yes, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<DeleteBlobResponse, AzureError> {
        let mut uri = generate_blob_uri(
            self.client(),
            self.container_name(),
            self.blob_name(),
            Some(&SnapshotRequired::to_uri_parameter(&self)),
        );

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}&{}", uri, nm);
        }

        let perform_request_response = self.client().perform_request(
            &uri,
            &Method::DELETE,
            &|mut request| {
                request = LeaseIdOption::add_optional_header(&self, request);
                request = ClientRequestIdOption::add_optional_header(&self, request);
                request
            },
            None,
        )?;

        let (headers, _body) = perform_request_response
            .check_status_extract_headers_and_body(StatusCode::ACCEPTED)
            .await?;
        DeleteBlobResponse::from_headers(&headers)
    }
}

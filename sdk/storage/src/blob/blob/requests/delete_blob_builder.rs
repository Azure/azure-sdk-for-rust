use crate::blob::blob::generate_blob_uri;
use crate::blob::blob::responses::DeleteBlobResponse;
use crate::core::prelude::*;
use azure_core::errors::AzureError;
use azure_core::lease::LeaseId;
use azure_core::prelude::*;
use azure_core::{DeleteSnapshotsMethod, No, ToAssign, Yes};
use azure_core::{DeleteSnapshotsMethodRequired, DeleteSnapshotsMethodSupport};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct DeleteBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    p_delete_snapshots_method: PhantomData<DeleteSnapshotMethodSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    delete_snapshots_method: DeleteSnapshotsMethod,
    timeout: Option<u64>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> DeleteBlobBuilder<'a, C, No, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> DeleteBlobBuilder<'a, C, No, No, No> {
        DeleteBlobBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            p_delete_snapshots_method: PhantomData {},
            delete_snapshots_method: DeleteSnapshotsMethod::Include,
            timeout: None,
            lease_id: None,
            client_request_id: None,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> ClientRequired<'a, C>
    for DeleteBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, BlobNameSet, DeleteSnapshotMethodSet> ContainerNameRequired<'a>
    for DeleteBlobBuilder<'a, C, Yes, BlobNameSet, DeleteSnapshotMethodSet>
where
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, DeleteSnapshotMethodSet> BlobNameRequired<'a>
    for DeleteBlobBuilder<'a, C, ContainerNameSet, Yes, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
    C: Client,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> DeleteSnapshotsMethodRequired
    for DeleteBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn delete_snapshots_method(&self) -> DeleteSnapshotsMethod {
        self.delete_snapshots_method
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> TimeoutOption
    for DeleteBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> LeaseIdOption<'a>
    for DeleteBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> ClientRequestIdOption<'a>
    for DeleteBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, BlobNameSet, DeleteSnapshotMethodSet> ContainerNameSupport<'a>
    for DeleteBlobBuilder<'a, C, No, BlobNameSet, DeleteSnapshotMethodSet>
where
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
    C: Client,
{
    type O = DeleteBlobBuilder<'a, C, Yes, BlobNameSet, DeleteSnapshotMethodSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        DeleteBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_delete_snapshots_method: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            delete_snapshots_method: self.delete_snapshots_method,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, DeleteSnapshotMethodSet> BlobNameSupport<'a>
    for DeleteBlobBuilder<'a, C, ContainerNameSet, No, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
    C: Client,
{
    type O = DeleteBlobBuilder<'a, C, ContainerNameSet, Yes, DeleteSnapshotMethodSet>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        DeleteBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_delete_snapshots_method: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            delete_snapshots_method: self.delete_snapshots_method,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> DeleteSnapshotsMethodSupport
    for DeleteBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, No>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = DeleteBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, Yes>;

    #[inline]
    fn with_delete_snapshots_method(
        self,
        delete_snapshots_method: DeleteSnapshotsMethod,
    ) -> Self::O {
        DeleteBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_delete_snapshots_method: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            delete_snapshots_method,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> TimeoutSupport
    for DeleteBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
    C: Client,
{
    type O = DeleteBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        DeleteBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_delete_snapshots_method: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            delete_snapshots_method: self.delete_snapshots_method,
            timeout: Some(timeout),
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> LeaseIdSupport<'a>
    for DeleteBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
    C: Client,
{
    type O = DeleteBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        DeleteBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_delete_snapshots_method: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            delete_snapshots_method: self.delete_snapshots_method,
            timeout: self.timeout,
            lease_id: Some(lease_id),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet> ClientRequestIdSupport<'a>
    for DeleteBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    DeleteSnapshotMethodSet: ToAssign,
    C: Client,
{
    type O = DeleteBlobBuilder<'a, C, ContainerNameSet, BlobNameSet, DeleteSnapshotMethodSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        DeleteBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            p_delete_snapshots_method: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            delete_snapshots_method: self.delete_snapshots_method,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> DeleteBlobBuilder<'a, C, Yes, Yes, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<DeleteBlobResponse, AzureError> {
        let mut uri =
            generate_blob_uri(self.client(), self.container_name(), self.blob_name(), None);

        if let Some(nm) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}?{}", uri, nm);
        }

        trace!("delete_blob uri == {:?}", uri);

        let perform_request_response = self.client().perform_request(
            &uri,
            &Method::DELETE,
            &|mut request| {
                request = DeleteSnapshotsMethodRequired::add_header(&self, request);
                request = LeaseIdOption::add_header(&self, request);
                request = ClientRequestIdOption::add_header(&self, request);
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

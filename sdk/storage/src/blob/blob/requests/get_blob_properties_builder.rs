use crate::blob::blob::responses::GetBlobPropertiesResponse;
use crate::blob::blob::{generate_blob_uri, Blob};
use crate::core::prelude::*;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::lease::LeaseId;
use azure_core::prelude::*;
use azure_core::{No, ToAssign, Yes};
use chrono::{DateTime, Utc};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct GetBlobPropertiesBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    client: &'a C,
    p_container_name: PhantomData<ContainerNameSet>,
    p_blob_name: PhantomData<BlobNameSet>,
    container_name: Option<&'a str>,
    blob_name: Option<&'a str>,
    snapshot: Option<DateTime<Utc>>,
    timeout: Option<u64>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> GetBlobPropertiesBuilder<'a, C, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> GetBlobPropertiesBuilder<'a, C, No, No> {
        GetBlobPropertiesBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            snapshot: None,
            timeout: None,
            lease_id: None,
            client_request_id: None,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> ClientRequired<'a, C>
    for GetBlobPropertiesBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client(&self) -> &'a C {
        self.client
    }
}

//get mandatory no traits methods

//set mandatory no traits methods
impl<'a, C, BlobNameSet> ContainerNameRequired<'a>
    for GetBlobPropertiesBuilder<'a, C, Yes, BlobNameSet>
where
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet> BlobNameRequired<'a>
    for GetBlobPropertiesBuilder<'a, C, ContainerNameSet, Yes>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> SnapshotOption
    for GetBlobPropertiesBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn snapshot(&self) -> Option<DateTime<Utc>> {
        self.snapshot
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> TimeoutOption
    for GetBlobPropertiesBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> LeaseIdOption<'a>
    for GetBlobPropertiesBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> ClientRequestIdOption<'a>
    for GetBlobPropertiesBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, C, BlobNameSet> ContainerNameSupport<'a>
    for GetBlobPropertiesBuilder<'a, C, No, BlobNameSet>
where
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = GetBlobPropertiesBuilder<'a, C, Yes, BlobNameSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        GetBlobPropertiesBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet> BlobNameSupport<'a>
    for GetBlobPropertiesBuilder<'a, C, ContainerNameSet, No>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = GetBlobPropertiesBuilder<'a, C, ContainerNameSet, Yes>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        GetBlobPropertiesBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
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
    for GetBlobPropertiesBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = GetBlobPropertiesBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_snapshot(self, snapshot: DateTime<Utc>) -> Self::O {
        GetBlobPropertiesBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: Some(snapshot),
            timeout: self.timeout,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> TimeoutSupport
    for GetBlobPropertiesBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = GetBlobPropertiesBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        GetBlobPropertiesBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: Some(timeout),
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> LeaseIdSupport<'a>
    for GetBlobPropertiesBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = GetBlobPropertiesBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        GetBlobPropertiesBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: self.timeout,
            lease_id: Some(lease_id),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> ClientRequestIdSupport<'a>
    for GetBlobPropertiesBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = GetBlobPropertiesBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        GetBlobPropertiesBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
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
impl<'a, C> GetBlobPropertiesBuilder<'a, C, Yes, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<GetBlobPropertiesResponse, AzureError> {
        let container_name = self.container_name().to_owned();
        let blob_name = self.blob_name().to_owned();
        let snapshot_time = self.snapshot();

        let mut uri =
            generate_blob_uri(self.client(), self.container_name(), self.blob_name(), None);

        let mut f_first = true;
        if let Some(snapshot) = SnapshotOption::to_uri_parameter(&self) {
            uri = format!("{}?{}", uri, snapshot);
            f_first = false;
        }
        if let Some(timeout) = TimeoutOption::to_uri_parameter(&self) {
            uri = format!("{}{}{}", uri, if f_first { "?" } else { "&" }, timeout);
        }

        trace!("uri == {:?}", uri);

        let future_response = self.client().perform_request(
            &uri,
            &Method::HEAD,
            &|mut request| {
                request = ClientRequestIdOption::add_header(&self, request);
                request = LeaseIdOption::add_header(&self, request);
                request
            },
            None,
        )?;

        let (headers, _) =
            check_status_extract_headers_and_body(future_response, StatusCode::OK).await?;
        let blob = Blob::from_headers(&blob_name, &container_name, snapshot_time, &headers)?;
        GetBlobPropertiesResponse::from_response(&headers, blob)
    }
}

use crate::blob::blob::responses::GetBlobResponse;
use crate::blob::blob::{generate_blob_uri, Blob};
use crate::core::prelude::*;
use azure_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_core::headers::RANGE_GET_CONTENT_MD5;
use azure_core::lease::LeaseId;
use azure_core::prelude::*;
use azure_core::range::Range;
use azure_core::util::RequestBuilderExt;
use azure_core::{No, ToAssign, Yes};
use chrono::{DateTime, Utc};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
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
    range: Option<&'a Range>,
    lease_id: Option<&'a LeaseId>,
    client_request_id: Option<&'a str>,
}

impl<'a, C> GetBlobBuilder<'a, C, No, No>
where
    C: Client,
{
    #[inline]
    pub(crate) fn new(client: &'a C) -> GetBlobBuilder<'a, C, No, No> {
        GetBlobBuilder {
            client,
            p_container_name: PhantomData {},
            container_name: None,
            p_blob_name: PhantomData {},
            blob_name: None,
            snapshot: None,
            timeout: None,
            range: None,
            lease_id: None,
            client_request_id: None,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> ClientRequired<'a, C>
    for GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
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
impl<'a, C, BlobNameSet> ContainerNameRequired<'a> for GetBlobBuilder<'a, C, Yes, BlobNameSet>
where
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, C, ContainerNameSet> BlobNameRequired<'a> for GetBlobBuilder<'a, C, ContainerNameSet, Yes>
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
    for GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
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
    for GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
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

impl<'a, C, ContainerNameSet, BlobNameSet> RangeOption<'a>
    for GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    #[inline]
    fn range(&self) -> Option<&'a Range> {
        self.range
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> LeaseIdOption<'a>
    for GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
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
    for GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
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

impl<'a, C, BlobNameSet> ContainerNameSupport<'a> for GetBlobBuilder<'a, C, No, BlobNameSet>
where
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = GetBlobBuilder<'a, C, Yes, BlobNameSet>;

    #[inline]
    fn with_container_name(self, container_name: &'a str) -> Self::O {
        GetBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: Some(container_name),
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: self.timeout,
            range: self.range,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet> BlobNameSupport<'a> for GetBlobBuilder<'a, C, ContainerNameSet, No>
where
    ContainerNameSet: ToAssign,
    C: Client,
{
    type O = GetBlobBuilder<'a, C, ContainerNameSet, Yes>;

    #[inline]
    fn with_blob_name(self, blob_name: &'a str) -> Self::O {
        GetBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: Some(blob_name),
            snapshot: self.snapshot,
            timeout: self.timeout,
            range: self.range,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> SnapshotSupport
    for GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_snapshot(self, snapshot: DateTime<Utc>) -> Self::O {
        GetBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: Some(snapshot),
            timeout: self.timeout,
            range: self.range,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> TimeoutSupport
    for GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_timeout(self, timeout: u64) -> Self::O {
        GetBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: Some(timeout),
            range: self.range,
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> RangeSupport<'a>
    for GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_range(self, range: &'a Range) -> Self::O {
        GetBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: self.timeout,
            range: Some(range),
            lease_id: self.lease_id,
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> LeaseIdSupport<'a>
    for GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_lease_id(self, lease_id: &'a LeaseId) -> Self::O {
        GetBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: self.timeout,
            range: self.range,
            lease_id: Some(lease_id),
            client_request_id: self.client_request_id,
        }
    }
}

impl<'a, C, ContainerNameSet, BlobNameSet> ClientRequestIdSupport<'a>
    for GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
    C: Client,
{
    type O = GetBlobBuilder<'a, C, ContainerNameSet, BlobNameSet>;

    #[inline]
    fn with_client_request_id(self, client_request_id: &'a str) -> Self::O {
        GetBlobBuilder {
            client: self.client,
            p_container_name: PhantomData {},
            p_blob_name: PhantomData {},
            container_name: self.container_name,
            blob_name: self.blob_name,
            snapshot: self.snapshot,
            timeout: self.timeout,
            range: self.range,
            lease_id: self.lease_id,
            client_request_id: Some(client_request_id),
        }
    }
}

// methods callable only when every mandatory field has been filled
impl<'a, C> GetBlobBuilder<'a, C, Yes, Yes>
where
    C: Client,
{
    pub async fn finalize(self) -> Result<GetBlobResponse, AzureError> {
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

        let perform_request_response = self.client().perform_request(
            &uri,
            &Method::GET,
            &|mut request| {
                if let Some(r) = self.range() {
                    request = LeaseIdOption::add_header(&self, request);
                    request = RangeOption::add_header(&self, request);

                    if r.len() <= 4 * 1024 * 1024 {
                        request = request.header_static(RANGE_GET_CONTENT_MD5, "true");
                    }
                }
                request
            },
            None,
        )?;

        let expected_status_code = if self.range().is_some() {
            StatusCode::PARTIAL_CONTENT
        } else {
            StatusCode::OK
        };

        let (headers, body) = check_status_extract_headers_and_body(
            perform_request_response.response_future,
            expected_status_code,
        )
        .await?;
        let blob = Blob::from_headers(&blob_name, &container_name, snapshot_time, &headers)?;
        GetBlobResponse::from_response(&headers, blob, &body)
    }
}

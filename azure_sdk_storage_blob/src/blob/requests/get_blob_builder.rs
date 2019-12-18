use crate::blob::responses::GetBlobResponse;
use crate::blob::{generate_blob_uri, Blob};
use azure_sdk_core::errors::{check_status_extract_headers_and_body, AzureError};
use azure_sdk_core::headers::RANGE_GET_CONTENT_MD5;
use azure_sdk_core::lease::LeaseId;
use azure_sdk_core::range::Range;
use azure_sdk_core::util::RequestBuilderExt;
use azure_sdk_core::{
    BlobNameRequired, BlobNameSupport, ClientRequestIdOption, ClientRequestIdSupport,
    ContainerNameRequired, ContainerNameSupport, LeaseIdOption, LeaseIdSupport, No, RangeOption,
    RangeSupport, SnapshotOption, SnapshotSupport, TimeoutOption, TimeoutSupport, ToAssign, Yes,
};
use azure_sdk_storage_core::client::Client;
use azure_sdk_storage_core::ClientRequired;
use chrono::{DateTime, Utc};
use hyper::{Method, StatusCode};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    client: &'a Client,
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

impl<'a> GetBlobBuilder<'a, No, No> {
    #[inline]
    pub(crate) fn new(client: &'a Client) -> GetBlobBuilder<'a, No, No> {
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

impl<'a, ContainerNameSet, BlobNameSet> ClientRequired<'a>
    for GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn client(&self) -> &'a Client {
        self.client
    }
}

impl<'a, BlobNameSet> ContainerNameRequired<'a> for GetBlobBuilder<'a, Yes, BlobNameSet>
where
    BlobNameSet: ToAssign,
{
    #[inline]
    fn container_name(&self) -> &'a str {
        self.container_name.unwrap()
    }
}

impl<'a, ContainerNameSet> BlobNameRequired<'a> for GetBlobBuilder<'a, ContainerNameSet, Yes>
where
    ContainerNameSet: ToAssign,
{
    #[inline]
    fn blob_name(&self) -> &'a str {
        self.blob_name.unwrap()
    }
}

impl<'a, ContainerNameSet, BlobNameSet> SnapshotOption
    for GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn snapshot(&self) -> Option<DateTime<Utc>> {
        self.snapshot
    }
}

impl<'a, ContainerNameSet, BlobNameSet> TimeoutOption
    for GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn timeout(&self) -> Option<u64> {
        self.timeout
    }
}

impl<'a, ContainerNameSet, BlobNameSet> RangeOption<'a>
    for GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn range(&self) -> Option<&'a Range> {
        self.range
    }
}

impl<'a, ContainerNameSet, BlobNameSet> LeaseIdOption<'a>
    for GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn lease_id(&self) -> Option<&'a LeaseId> {
        self.lease_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ClientRequestIdOption<'a>
    for GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    #[inline]
    fn client_request_id(&self) -> Option<&'a str> {
        self.client_request_id
    }
}

impl<'a, ContainerNameSet, BlobNameSet> ContainerNameSupport<'a>
    for GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = GetBlobBuilder<'a, Yes, BlobNameSet>;

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

impl<'a, ContainerNameSet, BlobNameSet> BlobNameSupport<'a>
    for GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = GetBlobBuilder<'a, ContainerNameSet, Yes>;

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

impl<'a, ContainerNameSet, BlobNameSet> SnapshotSupport
    for GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>;

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

impl<'a, ContainerNameSet, BlobNameSet> TimeoutSupport
    for GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>;

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

impl<'a, ContainerNameSet, BlobNameSet> RangeSupport<'a>
    for GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>;

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

impl<'a, ContainerNameSet, BlobNameSet> LeaseIdSupport<'a>
    for GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>;

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

impl<'a, ContainerNameSet, BlobNameSet> ClientRequestIdSupport<'a>
    for GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
    type O = GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>;

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

// methods callable regardless
impl<'a, ContainerNameSet, BlobNameSet> GetBlobBuilder<'a, ContainerNameSet, BlobNameSet>
where
    ContainerNameSet: ToAssign,
    BlobNameSet: ToAssign,
{
}

impl<'a> GetBlobBuilder<'a, Yes, Yes> {
    #[inline]
    pub async fn finalize(self) -> Result<GetBlobResponse, AzureError> {
        let container_name = self.container_name().to_owned();
        let blob_name = self.blob_name().to_owned();
        let snapshot_time = self.snapshot();

        let mut uri = generate_blob_uri(&self, None);

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
            &Method::GET,
            |mut request| {
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

        let (headers, body) =
            check_status_extract_headers_and_body(future_response, expected_status_code).await?;
        let blob = Blob::from_headers(&blob_name, &container_name, snapshot_time, &headers)?;
        GetBlobResponse::from_response(&headers, blob, &body)
    }
}

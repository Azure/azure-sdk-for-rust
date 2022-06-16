use crate::{blob::responses::GetBlobResponse, prelude::*, Header};
use azure_core::{
    error::Result,
    headers::{add_optional_header, add_optional_header_ref, AsHeaders, CLIENT_REQUEST_ID},
    prelude::*,
};
use futures::stream::Stream;
use std::convert::TryInto;

// TODO Copy trait is required for current stream approach, once migrated to new approach with
// pipelines, we can use the ClientRequestId from core again which does not implement Copy.
#[derive(Debug, Clone, Copy)]
pub struct ClientRequestId<'a>(&'a str);

impl<'a> ClientRequestId<'a> {
    pub fn new(client_request_id: &'a str) -> Self {
        Self(client_request_id)
    }
}

impl<'a> From<&'a str> for ClientRequestId<'a> {
    fn from(client_request_id: &'a str) -> Self {
        Self::new(client_request_id)
    }
}

impl<'a> Header for ClientRequestId<'a> {
    fn name(&self) -> azure_core::headers::HeaderName {
        CLIENT_REQUEST_ID.into()
    }

    fn value(&self) -> azure_core::headers::HeaderValue {
        self.0.to_owned().into()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GetBlobBuilder<'a> {
    blob_client: &'a BlobClient,
    range: Option<Range>,
    blob_versioning: Option<&'a BlobVersioning>,
    client_request_id: Option<ClientRequestId<'a>>,
    timeout: Option<Timeout>,
    lease_id: Option<&'a LeaseId>,
}

impl<'a> GetBlobBuilder<'a> {
    pub(crate) fn new(blob_client: &'a BlobClient) -> Self {
        Self {
            blob_client,
            blob_versioning: None,
            timeout: None,
            range: None,
            lease_id: None,
            client_request_id: None,
        }
    }

    setters! {
        range: Range => Some(range),
        blob_versioning: &'a BlobVersioning => Some(blob_versioning),
        client_request_id: ClientRequestId<'a> => Some(client_request_id),
        timeout: Timeout => Some(timeout),
        lease_id: &'a LeaseId => Some(lease_id),
    }

    pub async fn execute(&self) -> Result<GetBlobResponse> {
        let mut url = self.blob_client.url_with_segments(None)?;

        self.blob_versioning.append_to_url_query(&mut url);
        self.timeout.append_to_url_query(&mut url);

        trace!("url == {:?}", url);

        let (request, _url) = self.blob_client.prepare_request(
            url.as_str(),
            &http::Method::GET,
            &|mut request| {
                if let Some(item) = &self.range {
                    for (name, value) in item.as_headers() {
                        request = request.header(name.as_str(), value.as_str())
                    }
                }
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request
            },
            None,
        )?;

        debug!("request == {:#?}", request);

        let expected_status_code = if self.range.is_some() {
            http::StatusCode::PARTIAL_CONTENT
        } else {
            http::StatusCode::OK
        };

        let response = self
            .blob_client
            .http_client()
            .execute_request_check_status(request, expected_status_code)
            .await?;

        Ok((self.blob_client.blob_name(), response).try_into()?)
    }

    pub fn stream(self, chunk_size: u64) -> impl Stream<Item = Result<GetBlobResponse>> + 'a {
        enum States {
            Init,
            Progress(Range),
            End,
        }

        // this can either be the range requested by the caller or the complete file.
        let requested_range = self.range.unwrap_or_else(|| Range::new(0, u64::MAX));

        futures::stream::unfold(States::Init, move |state| async move {
            let mut remaining = match state {
                States::Init => requested_range,
                States::Progress(range) => range,
                States::End => return None,
            };

            debug!(
                "remaining.start == {}, chunk_size == {}, remaining.end == {}",
                remaining.start, chunk_size, remaining.end
            );

            let range = if remaining.start + chunk_size > remaining.end {
                Range::new(remaining.start, remaining.end)
            } else {
                Range::new(remaining.start, remaining.start + chunk_size)
            };

            let req = self.range(range);

            let response = match req.execute().await {
                Ok(response) => response,
                Err(err) => return Some((Err(err), States::End)),
            };

            debug!("response.content_range == {:?}", response.content_range);

            // now that we know what the remote blob size is, let's update the
            // boundary. We do this only if it's smaller than the requested size because the could
            // have specified a smaller range.
            remaining.end = match response.content_range {
                None => requested_range.end,
                Some(content_range) => {
                    std::cmp::min(requested_range.end, content_range.total_length())
                }
            };

            let next_state = if remaining.end > range.end {
                States::Progress(Range::new(range.end, remaining.end))
            } else {
                States::End
            };

            Some((Ok(response), next_state))
        })
    }
}

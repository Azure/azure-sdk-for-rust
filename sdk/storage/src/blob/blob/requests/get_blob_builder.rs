use crate::blob::blob::responses::GetBlobResponse;
use crate::blob::prelude::*;
use crate::clients::BlobClient;
use azure_core::headers::{add_optional_header, add_optional_header_ref};
use azure_core::prelude::*;
use futures::stream::Stream;

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

    pub async fn execute(
        &self,
    ) -> Result<GetBlobResponse, Box<dyn std::error::Error + Send + Sync>> {
        let mut url = self
            .blob_client
            .storage_account_client()
            .blob_storage_url()
            .to_owned();
        url.path_segments_mut()
            .map_err(|_| "Invalid blob URL")?
            .push(self.blob_client.container_client().container_name())
            .push(self.blob_client.blob_name());

        self.blob_versioning.append_to_url_query(&mut url);
        self.timeout.append_to_url_query(&mut url);

        trace!("url == {:?}", url);

        let (request, _url) = self.blob_client.prepare_request(
            url.as_str(),
            &http::Method::GET,
            &|mut request| {
                request = add_optional_header(&self.range, request);
                request = add_optional_header(&self.client_request_id, request);
                request = add_optional_header_ref(&self.lease_id, request);
                request
            },
            None,
        )?;

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

        debug!("response.headers() == {:#?}", response.headers());

        let blob = Blob::from_headers(self.blob_client.blob_name(), response.headers())?;
        Ok(GetBlobResponse::from_response(
            response.headers(),
            blob,
            response.body(),
        )?)
    }

    pub fn stream(
        self,
        chunk_size: u64,
    ) -> impl Stream<Item = Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>>> + 'a {
        enum States {
            Init,
            Progress(Range),
            End,
        }

        let complete_range = Range::new(0, u64::MAX);

        futures::stream::unfold(States::Init, move |state| async move {
            let remaining = match state {
                States::Init => self.range.unwrap_or(complete_range),
                States::Progress(range) => range,
                States::End => return None,
            };

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

            Some((
                Ok(response.data),
                if remaining.end > range.end {
                    if self.range.is_some() {
                        States::Progress(Range::new(range.end, remaining.end))
                    } else {
                        // if we are here it means the user have not specified a
                        // range and we didn't get the whole blob in one passing.
                        // We specified u64::MAX as the first range but now
                        // we need to find the correct size to avoid requesting data
                        // outside the valid range.
                        debug!("content-range == {:?}", response.content_range);
                        // this unwrap should always be safe since we did not
                        // get the whole blob in the previous call.
                        let content_range = response.content_range.unwrap();
                        let ridx =
                            match content_range.find('/') {
                                Some(ridx) => ridx,
                                None => return Some((
                                    Err("The returned content-range is invalid: / is not present"
                                        .into()),
                                    States::End,
                                )),
                            };
                        let total =
                            match str::parse(&content_range[ridx + 1..]) {
                                Ok(total) => total,
                                Err(_err) => return Some((
                                    Err("The returned content-range is invalid: after / there is a non valid number"
                                        .into()),
                                    States::End,
                                )),
                            };

                        States::Progress(Range::new(range.end, total))
                    }
                } else {
                    States::End
                },
            ))
        })
    }
}

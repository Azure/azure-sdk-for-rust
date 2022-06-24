use crate::{blob::Blob, prelude::*};
use azure_core::{
    collect_pinned_stream,
    error::{Error, ErrorKind, ResultExt},
    headers::*,
    prelude::*,
    Context, Pageable, RequestId, Response as AzureResponse,
};
use bytes::Bytes;
use chrono::{DateTime, Utc};
use std::str::FromStr;

const DEFAULT_CHUNK_SIZE: u64 = 0x1000 * 0x1000;

#[derive(Clone)]
pub struct GetBlobBuilder {
    blob_client: BlobClient,
    range: Option<Range>,
    blob_versioning: Option<BlobVersioning>,
    timeout: Option<Timeout>,
    lease_id: Option<LeaseId>,
    chunk_size: u64,
    context: Context,
}

fn initial_range(chunk_size: u64, request_range: Option<Range>) -> Range {
    match request_range {
        Some(range) => {
            let len = std::cmp::min(range.len(), chunk_size);
            Range::new(range.start, range.start + len)
        }
        None => Range::new(0, chunk_size),
    }
}

fn remaining_range(
    chunk_size: u64,
    content_range: Option<ContentRange>,
    base_range: Option<Range>,
) -> Option<Range> {
    // if there was no content range in the response, assume the entire blob was
    // returned.
    let content_range = content_range?;

    // if the next byte is at or past the total length, then we're done.
    if content_range.end() + 1 >= content_range.total_length() {
        return None;
    }

    // if the user didn't specify a range, assume the entire size
    let base_range = base_range.unwrap_or_else(|| Range::new(0, std::u64::MAX));

    // if the response said the end of the blob was downloaded, we're done
    if content_range.end() >= base_range.end {
        return None;
    }
    // if the user specified range is smaller than the blob, truncate the
    // requested range.  Note, we add + 1, as we don't need to re-fetch the last
    // byte of the previous request.
    let left = Range::new(
        content_range.end() + 1,
        std::cmp::min(base_range.end, content_range.total_length()),
    );

    if left.len() > chunk_size {
        Some(Range::new(left.start, left.start + chunk_size))
    } else {
        Some(left)
    }
}

impl GetBlobBuilder {
    pub(crate) fn new(blob_client: BlobClient) -> Self {
        Self {
            blob_client,
            blob_versioning: None,
            timeout: None,
            range: None,
            lease_id: None,
            chunk_size: DEFAULT_CHUNK_SIZE,
            context: Context::new(),
        }
    }

    setters! {
        range: Range => Some(range),
        chunk_size: u64 => chunk_size,
        blob_versioning: BlobVersioning => Some(blob_versioning),
        timeout: Timeout => Some(timeout),
        lease_id: LeaseId => Some(lease_id),
    }

    pub fn into_stream(self) -> Pageable<GetBlobResponse, Error> {
        let make_request = move |continuation: Option<Continuation>| {
            let this = self.clone();
            let mut ctx = self.context.clone();
            async move {
                let mut url = this.blob_client.url_with_segments(None)?;

                let range = match continuation {
                    Some(Continuation::String(_)) => {
                        panic!("unexpected contination type")
                    }
                    Some(Continuation::Range(range)) => range.into(),
                    None => initial_range(this.chunk_size, this.range),
                };

                this.blob_versioning.append_to_url_query(&mut url);
                this.timeout.append_to_url_query(&mut url);

                let mut request =
                    this.blob_client
                        .prepare_request(url.as_str(), http::Method::GET, None)?;

                for (name, value) in range.as_headers() {
                    request.insert_header(name, value);
                }

                request.add_optional_header(&this.lease_id);

                let response = this.blob_client.send(&mut ctx, &mut request).await?;

                GetBlobResponse::try_from(this, response).await
            }
        };
        Pageable::new(make_request)
    }
}

#[derive(Debug, Clone)]
pub struct GetBlobResponse {
    pub request_id: RequestId,
    pub blob: Blob,
    pub data: Bytes,
    pub date: DateTime<Utc>,
    pub remaining_range: Option<Range>,
}

impl GetBlobResponse {
    async fn try_from(
        request: GetBlobBuilder,
        response: AzureResponse,
    ) -> azure_core::Result<Self> {
        let (_, headers, body) = response.deconstruct();
        let data = collect_pinned_stream(body).await?;

        let request_id = request_id_from_headers(&headers)?;
        let date = date_from_headers(&headers)?;

        let content_range_header = headers.get(&CONTENT_RANGE);
        let content_range = match content_range_header {
            Some(hv) => {
                Some(ContentRange::from_str(hv.as_str()).map_kind(ErrorKind::DataConversion)?)
            }
            None => None,
        };

        let remaining_range = remaining_range(request.chunk_size, content_range, request.range);

        Ok(Self {
            request_id,
            blob: Blob::from_headers(request.blob_client.blob_name(), &headers)?,
            data,
            date,
            remaining_range,
        })
    }
}

impl Continuable for GetBlobResponse {
    fn continuation(&self) -> Option<Continuation> {
        self.remaining_range.map(Continuation::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_range() -> azure_core::Result<()> {
        let result = initial_range(3, Some(Range::new(0, 10)));
        let expected = Range::new(0, 3);
        assert_eq!(result, expected);

        let result = initial_range(3, Some(Range::new(3, 10)));
        let expected = Range::new(3, 6);
        assert_eq!(result, expected);

        let result = initial_range(3, None);
        let expected = Range::new(0, 3);
        assert_eq!(result, expected);
        Ok(())
    }
    #[test]
    fn test_remaining_range() -> azure_core::Result<()> {
        let result = remaining_range(3, None, None);
        assert!(result.is_none());

        let result = remaining_range(3, None, Some(Range::new(0, 10)));
        assert!(result.is_none());

        let result = remaining_range(
            3,
            Some(ContentRange::new(0, 3, 10)),
            Some(Range::new(0, 10)),
        );
        assert_eq!(result, Some(Range::new(4, 7)));

        let result = remaining_range(
            3,
            Some(ContentRange::new(0, 10, 10)),
            Some(Range::new(0, 10)),
        );
        assert!(result.is_none());

        let result = remaining_range(3, Some(ContentRange::new(0, 10, 10)), None);
        assert!(result.is_none());

        let result = remaining_range(3, Some(ContentRange::new(0, 10, 20)), None);
        assert_eq!(result, Some(Range::new(11, 14)));

        Ok(())
    }
}

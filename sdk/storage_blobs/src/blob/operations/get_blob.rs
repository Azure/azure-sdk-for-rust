use crate::{blob::Blob, prelude::*};
use azure_core::{
    error::Error, headers::*, prelude::*, Pageable, RequestId, Response as AzureResponse,
    ResponseBody,
};
use time::OffsetDateTime;

const DEFAULT_CHUNK_SIZE: u64 = 0x1000 * 0x1000;

operation! {
    #[stream]
    GetBlob,
    client: BlobClient,
    ?range: Range,
    ?blob_versioning: BlobVersioning,
    ?lease_id: LeaseId,
    ?chunk_size: u64,
    ?if_modified_since: IfModifiedSinceCondition,
    ?if_match: IfMatchCondition,
    ?if_tags: IfTags,
}

impl GetBlobBuilder {
    pub fn into_stream(self) -> Pageable<GetBlobResponse, Error> {
        let make_request = move |continuation: Option<Range>| {
            let this = self.clone();
            let mut ctx = self.context.clone();
            async move {
                let mut url = this.client.url()?;

                let range = match continuation {
                    Some(range) => range,
                    None => {
                        initial_range(this.chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE), this.range)
                    }
                };

                this.blob_versioning.append_to_url_query(&mut url);

                let mut headers = Headers::new();
                for (name, value) in range.as_headers() {
                    headers.insert(name, value);
                }

                headers.add(this.lease_id);
                headers.add(this.if_modified_since);
                headers.add(this.if_match.clone());
                headers.add(this.if_tags.clone());

                let mut request =
                    this.client
                        .finalize_request(url, azure_core::Method::Get, headers, None)?;

                let response = this.client.send(&mut ctx, &mut request).await?;

                GetBlobResponse::try_from(this, response).await
            }
        };
        Pageable::new(make_request)
    }
}

#[derive(Debug)]
pub struct GetBlobResponse {
    pub request_id: RequestId,
    pub blob: Blob,
    pub data: ResponseBody,
    pub date: OffsetDateTime,
    pub remaining_range: Option<Range>,
}

impl GetBlobResponse {
    async fn try_from(
        request: GetBlobBuilder,
        response: AzureResponse,
    ) -> azure_core::Result<Self> {
        let headers = response.headers();

        let request_id = request_id_from_headers(headers)?;
        let date = date_from_headers(headers)?;

        let content_range = headers.get_optional_as(&CONTENT_RANGE)?;

        let remaining_range = remaining_range(
            request.chunk_size.unwrap_or(DEFAULT_CHUNK_SIZE),
            request.range,
            content_range,
        );
        let blob = Blob::from_headers(request.client.blob_name(), headers)?;
        let data = response.into_body();

        Ok(Self {
            request_id,
            blob,
            data,
            date,
            remaining_range,
        })
    }
}

impl Continuable for GetBlobResponse {
    type Continuation = Range;
    fn continuation(&self) -> Option<Self::Continuation> {
        self.remaining_range
    }
}

// caclculate the first Range for use at the beginning of the Pageable.
fn initial_range(chunk_size: u64, request_range: Option<Range>) -> Range {
    match request_range {
        Some(range) => {
            let len = std::cmp::min(range.len(), chunk_size);
            Range::new(range.start, range.start + len)
        }
        None => Range::new(0, chunk_size),
    }
}

// After each request, calculate how much data is left to be read based on the
// requested chunk size, requested range, and Content-Range header from the response.
//
// The Content-Range response is authoritative for the current size of the blob,
// which we use that to determine the next chunk size.  If the Content-Range is
// missing from the response, we assume the response had the entire blob.
//
// If the Content-Range indicates the response was at the end of the blob or
// user's requested slice, we return None to indicate the response is complete.
//
// The remaining range is calculated from immediately after the response until
// the end of the requested range or chunk size, which ever is smaller.
fn remaining_range(
    chunk_size: u64,
    base_range: Option<Range>,
    content_range: Option<ContentRange>,
) -> Option<Range> {
    // if there was no content range in the response, assume the entire blob was
    // returned.
    let content_range = content_range?;

    // if the next byte is at or past the total length, then we're done.
    if content_range.end() + 1 >= content_range.total_length() {
        return None;
    }

    // if the user didn't specify a range, assume the entire size
    let requested_range = base_range.unwrap_or_else(|| Range::new(0, content_range.total_length()));

    // if the response said the end of the blob was downloaded, we're done
    // Note, we add + 1, as we don't need to re-fetch the last
    // byte of the previous request.
    if content_range.end() + 1 >= requested_range.end {
        return None;
    }

    // if the user specified range is smaller than the blob, truncate the
    // requested range.  Note, we add + 1, as we don't need to re-fetch the last
    // byte of the previous request.
    let start = content_range.end() + 1;
    let remaining_size = requested_range.end - start;

    let size = std::cmp::min(remaining_size, chunk_size);

    Some(Range::new(start, start + size))
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

        let result = remaining_range(3, Some(Range::new(0, 10)), None);
        assert!(result.is_none());

        let result = remaining_range(
            3,
            Some(Range::new(0, 10)),
            Some(ContentRange::new(0, 3, 10)),
        );
        assert_eq!(result, Some(Range::new(4, 7)));

        let result = remaining_range(
            3,
            Some(Range::new(0, 10)),
            Some(ContentRange::new(0, 10, 10)),
        );
        assert!(result.is_none());

        let result = remaining_range(3, None, Some(ContentRange::new(0, 10, 10)));
        assert!(result.is_none());

        let result = remaining_range(3, None, Some(ContentRange::new(0, 10, 20)));
        assert_eq!(result, Some(Range::new(11, 14)));

        let result = remaining_range(
            20,
            Some(Range::new(5, 15)),
            Some(ContentRange::new(5, 14, 20)),
        );
        assert_eq!(result, None);

        Ok(())
    }
}

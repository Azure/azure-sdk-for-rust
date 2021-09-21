#[cfg(feature = "mock_transport_framework")]
use crate::{collect_pinned_stream, BytesStream, Response, StreamError};
use bytes::Bytes;
use http::{header, HeaderMap, StatusCode};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BytesResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: Bytes,
}

impl BytesResponse {
    pub(crate) fn new(status: StatusCode, headers: HeaderMap, body: Bytes) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    #[cfg(feature = "mock_transport_framework")]
    pub(crate) async fn duplicate(response: Response) -> Result<(Response, Self), StreamError> {
        let (status_code, header_map, pinned_stream) = response.deconstruct();
        let response_bytes = collect_pinned_stream(pinned_stream).await?;

        let response = Response::new(
            status_code,
            header_map.clone(),
            Box::pin(BytesStream::new(response_bytes.clone())),
        );
        let bytes_response = BytesResponse::new(status_code, header_map, response_bytes);

        Ok((response, bytes_response))
    }

    pub(crate) fn deconstruct(self) -> (StatusCode, HeaderMap, Bytes) {
        (self.status, self.headers, self.body)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SerializedBytesResponse<'a> {
    status: u16,
    #[serde(borrow)]
    headers: HashMap<&'a str, &'a str>,
    body: String,
}

impl<'a> From<&'a BytesResponse> for SerializedBytesResponse<'a> {
    fn from(r: &'a BytesResponse) -> Self {
        let mut headers = HashMap::new();
        for (h, v) in r.headers.iter() {
            headers.insert(h.as_str(), v.to_str().unwrap());
        }
        let status = r.status.as_u16();
        let body = base64::encode(&r.body as &[u8]);
        Self {
            status,
            headers,
            body,
        }
    }
}

impl std::convert::TryFrom<SerializedBytesResponse<'_>> for BytesResponse {
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn try_from(r: SerializedBytesResponse<'_>) -> Result<Self, Self::Error> {
        let mut headers = HeaderMap::new();
        for (&n, &v) in r.headers.iter() {
            let name = header::HeaderName::from_lowercase(n.as_bytes())?;
            let value = header::HeaderValue::from_str(v)?;
            headers.insert(name, value);
        }
        let body = Bytes::from(base64::decode(r.body)?);
        let status = StatusCode::from_u16(r.status)?;

        Ok(Self::new(status, headers, body))
    }
}

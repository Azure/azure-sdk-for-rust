use bytes::Bytes;
use http::{header, HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{collect_pinned_stream, BytesStream, Response};

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MockResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: Bytes,
}

impl From<MockResponse> for Response {
    fn from(mock_response: MockResponse) -> Self {
        let bytes_stream: crate::BytesStream = mock_response.body.into();

        Self::new(
            mock_response.status,
            mock_response.headers,
            Box::pin(bytes_stream),
        )
    }
}

impl MockResponse {
    pub(crate) fn new(status: StatusCode, headers: HeaderMap, body: Bytes) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    pub(crate) async fn duplicate(
        response: crate::Response,
    ) -> Result<(crate::Response, Self), crate::StreamError> {
        let (status_code, header_map, pinned_stream) = response.deconstruct();
        let response_bytes = collect_pinned_stream(pinned_stream).await?;

        let response = Response::new(
            status_code,
            header_map.clone(),
            Box::pin(BytesStream::new(response_bytes.clone())),
        );
        let mock_response = MockResponse::new(status_code, header_map, response_bytes);

        Ok((response, mock_response))
    }
}

impl<'de> Deserialize<'de> for MockResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use serde::de::Error;
        let r = SerializedMockResponse::deserialize(deserializer)?;
        let mut headers = HeaderMap::new();
        for (n, v) in r.headers.iter() {
            let name = header::HeaderName::from_lowercase(n.as_bytes()).map_err(Error::custom)?;
            let value = header::HeaderValue::from_str(&v).map_err(Error::custom)?;
            headers.insert(name, value);
        }
        let body = Bytes::from(base64::decode(r.body).map_err(Error::custom)?);
        let status = StatusCode::from_u16(r.status).map_err(Error::custom)?;

        Ok(Self::new(status, headers, body))
    }
}

impl Serialize for MockResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut headers = BTreeMap::new();
        for (h, v) in self.headers.iter() {
            headers.insert(h.as_str().into(), v.to_str().unwrap().into());
        }
        let status = self.status.as_u16();
        let body = base64::encode(&self.body as &[u8]);
        let s = SerializedMockResponse {
            status,
            headers,
            body,
        };
        s.serialize(serializer)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SerializedMockResponse {
    status: u16,
    headers: BTreeMap<String, String>,
    body: String,
}

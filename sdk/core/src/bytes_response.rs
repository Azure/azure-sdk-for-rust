#[cfg(feature = "mock_transport_framework")]
use crate::{collect_pinned_stream, BytesStream, Response, StreamError};
use bytes::Bytes;
use http::{header, HeaderMap, StatusCode};
use serde::{Deserialize, Serialize};
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

impl<'de> Deserialize<'de> for BytesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        use serde::de::Error;
        let r = SerializedBytesResponse::deserialize(deserializer)?;
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

impl Serialize for BytesResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        let mut headers = HashMap::new();
        for (h, v) in self.headers.iter() {
            headers.insert(h.as_str().into(), v.to_str().unwrap().into());
        }
        let status = self.status.as_u16();
        let body = base64::encode(&self.body as &[u8]);
        let s = SerializedBytesResponse {
            status,
            headers,
            body,
        };
        s.serialize(serializer)
    }
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SerializedBytesResponse {
    status: u16,
    headers: HashMap<String, String>,
    body: String,
}
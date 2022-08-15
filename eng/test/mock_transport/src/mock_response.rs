use azure_core::{
    error,
    headers::{HeaderName, HeaderValue, Headers},
    BytesStream, Response, StatusCode,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MockResponse {
    status: StatusCode,
    headers: Headers,
    body: Bytes,
}

impl From<MockResponse> for Response {
    fn from(mock_response: MockResponse) -> Self {
        let bytes_stream: azure_core::BytesStream = mock_response.body.into();

        Self::new(
            mock_response.status,
            mock_response.headers,
            Box::pin(bytes_stream),
        )
    }
}

impl MockResponse {
    pub(crate) fn new(status: StatusCode, headers: Headers, body: Bytes) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    pub(crate) async fn duplicate(response: Response) -> error::Result<(Response, Self)> {
        use error::ResultExt;
        let (status_code, header_map, body) = response.deconstruct();
        let response_bytes = body.collect().await.context(
            error::ErrorKind::Io,
            "an error occurred fetching the next part of the byte stream",
        )?;

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
        let mut headers = Headers::new();
        for (n, v) in r.headers.iter() {
            let name = HeaderName::from(n.to_owned());
            let value = HeaderValue::from(v.to_owned());
            headers.insert(name, value);
        }
        let body = Bytes::from(base64::decode(r.body).map_err(Error::custom)?);
        let status = StatusCode::try_from(r.status)
            .map_err(|_| Error::custom(format!("invalid status code {}", r.status)))?;

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
            headers.insert(h.as_str().into(), v.as_str().into());
        }
        let status = self.status as u16;
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

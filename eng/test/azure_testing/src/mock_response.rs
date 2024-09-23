// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{
    base64, error,
    headers::{self, HeaderName, HeaderValue, Headers},
    BytesStream, Response, StatusCode,
};
use bytes::Bytes;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{is_json_content_type, is_utf8_safe_content_type, BodyEncoding, SerializedBody};

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
        for (n, v) in &r.headers {
            let name = HeaderName::from(n.to_owned());
            let value = HeaderValue::from(v.to_owned());
            headers.insert(name, value);
        }
        let body = match r.body.encoding {
            BodyEncoding::Empty => Bytes::new(),
            BodyEncoding::Utf8 => Bytes::copy_from_slice(
                r.body
                    .content
                    .as_str()
                    .ok_or(Error::custom(
                        "expected a string for content when body is UTF-8 encoded",
                    ))?
                    .as_bytes(),
            ),
            BodyEncoding::Json => serde_json::to_string(&r.body.content)
                .map_err(|_| Error::custom("invalid JSON in JSON body"))?
                .into(),
            BodyEncoding::Base64 => base64::decode(r.body.content.as_str().ok_or(
                Error::custom("expected a string for content when body is UTF-8 encoded"),
            )?)
            .map_err(|_| Error::custom("invalid base64 in JSON body"))?
            .into(),
        };
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
            let v = crate::sanitation::sanitize_header(h, v);
            headers.insert(h.as_str().into(), v.as_str().into());
        }
        let status = self.status as u16;

        let body = if self.body.is_empty() {
            SerializedBody {
                encoding: BodyEncoding::Empty,
                content: serde_json::Value::Null,
            }
        } else if is_json_content_type(self.headers.get_optional_str(&headers::CONTENT_TYPE)) {
            let bytes = Vec::from(self.body.clone());
            SerializedBody {
                encoding: BodyEncoding::Json,
                content: serde_json::from_slice::<serde_json::Value>(&bytes)
                    .map_err(|_| serde::ser::Error::custom("invalid utf-8 in JSON body"))?,
            }
        } else if is_utf8_safe_content_type(self.headers.get_optional_str(&headers::CONTENT_TYPE)) {
            let bytes = Vec::from(self.body.clone());
            SerializedBody {
                encoding: BodyEncoding::Utf8,
                content: serde_json::Value::String(
                    String::from_utf8(bytes)
                        .map_err(|_| serde::ser::Error::custom("invalid utf-8 in JSON body"))?,
                ),
            }
        } else {
            SerializedBody {
                encoding: BodyEncoding::Base64,
                content: serde_json::Value::String(base64::encode(&self.body)),
            }
        };

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
    body: SerializedBody,
}

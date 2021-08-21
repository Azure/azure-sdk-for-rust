#[cfg(feature = "mock_transport_generate")]
use crate::{collect_pinned_stream, BytesStream, Response, StreamError};
use bytes::Bytes;
use http::{HeaderMap, StatusCode};
use serde::de::Visitor;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

const FIELDS: &'static [&'static str] = &["status", "headers", "body"];

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BytesResponse {
    status: StatusCode,
    headers: HeaderMap,
    body: Bytes,
}

impl Serialize for BytesResponse {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hm = HashMap::new();
        for (h, v) in self.headers.iter() {
            hm.insert(h.to_string(), v.to_str().unwrap());
        }

        let mut state = serializer.serialize_struct("Response", 3)?;
        state.serialize_field(FIELDS[0], &self.status.as_u16())?;
        state.serialize_field(FIELDS[1], &hm)?;
        state.serialize_field(FIELDS[2], &self.body as &[u8])?;

        state.end()
    }
}

impl<'de> Deserialize<'de> for BytesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Response", FIELDS, ByteResponseVisitor)
    }
}

struct ByteResponseVisitor;

impl<'de> Visitor<'de> for ByteResponseVisitor {
    type Value = BytesResponse;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("struct BytesResponse")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let status: (&str, u16) = match map.next_entry()? {
            Some((a, b)) => (a, b),
            None => return Err(serde::de::Error::custom("missing status code")),
        };

        if status.0 != FIELDS[0] {
            return Err(serde::de::Error::custom(format!(
                "unexpected field {}, expected {}",
                status.0, FIELDS[0]
            )));
        }

        let headers: (&str, HashMap<&str, String>) = match map.next_entry()? {
            Some((a, b)) => (a, b),
            None => return Err(serde::de::Error::custom("missing header map")),
        };
        if headers.0 != FIELDS[1] {
            return Err(serde::de::Error::custom(format!(
                "unexpected field {}, expected {}",
                headers.0, FIELDS[1]
            )));
        }

        let body: (&str, Vec<u8>) = match map.next_entry()? {
            Some((a, b)) => (a, b),
            None => return Err(serde::de::Error::custom("missing body")),
        };
        if body.0 != FIELDS[2] {
            return Err(serde::de::Error::custom(format!(
                "unexpected field {}, expected {}",
                body.0, FIELDS[2]
            )));
        }

        let mut hm = HeaderMap::new();
        for (k, v) in headers.1.into_iter() {
            hm.append(
                http::header::HeaderName::from_lowercase(k.as_bytes())
                    .map_err(serde::de::Error::custom)?,
                http::HeaderValue::from_str(&v).map_err(serde::de::Error::custom)?,
            );
        }

        Ok(Self::Value::new(
            StatusCode::from_u16(status.1).map_err(serde::de::Error::custom)?,
            hm,
            Bytes::from(body.1),
        ))
    }
}

impl BytesResponse {
    pub(crate) fn new(status: StatusCode, headers: HeaderMap, body: Bytes) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }

    #[cfg(feature = "mock_transport_generate")]
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

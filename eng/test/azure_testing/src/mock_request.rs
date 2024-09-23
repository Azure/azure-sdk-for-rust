// Copyright (c) Microsoft Corporation. All rights reserved.
// Licensed under the MIT License.

use azure_core::{base64, headers, Body, Method, Request, Url};
use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};
use std::collections::BTreeMap;
use std::str::FromStr;

use crate::{is_utf8_safe_content_type, sanitation, BodyEncoding, SerializedBody};

#[derive(Serialize, Deserialize)]
pub(crate) struct SerializedMockRequest {
    pub url: String,
    pub method: String,
    pub headers: BTreeMap<String, String>,
    pub body: SerializedBody,
}

pub struct RequestSerializer<'a>(&'a Request);

impl<'a> RequestSerializer<'a> {
    pub fn new(req: &'a Request) -> Self {
        Self(req)
    }
}

pub struct RequestDeserializer(Request);

impl RequestDeserializer {
    pub fn into_inner(self) -> Request {
        self.0
    }
}

impl<'de> Deserialize<'de> for RequestDeserializer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let serialized_request = SerializedMockRequest::deserialize(deserializer)?;

        let body =
            match serialized_request.body.encoding {
                BodyEncoding::Empty => Vec::new(),
                BodyEncoding::Json => serde_json::to_string(&serialized_request.body.content)
                    .map_err(|_| Error::custom("invalid JSON in JSON body"))?
                    .into(),
                BodyEncoding::Base64 => {
                    base64::decode(serialized_request.body.content.as_str().ok_or(
                        Error::custom("expected a string for content when body is UTF-8 encoded"),
                    )?)
                    .map_err(serde::de::Error::custom)?
                }
                BodyEncoding::Utf8 => serialized_request
                    .body
                    .content
                    .as_str()
                    .ok_or(Error::custom(
                        "expected a string for content when body is UTF-8 encoded",
                    ))?
                    .into(),
            };

        // `url` cannot be relative
        let url = Url::parse("http://example.com").unwrap();
        let url = url
            .join(&serialized_request.url)
            .expect("expected a valid uri");
        let mut req = Request::new(
            url,
            Method::from_str(&serialized_request.method).expect("expected a valid HTTP method"),
        );
        for (k, v) in serialized_request.headers {
            req.insert_header(k.to_owned(), v);
        }

        req.set_body(bytes::Bytes::from(body));
        Ok(Self(req))
    }
}

impl<'a> Serialize for RequestSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hm = std::collections::BTreeMap::new();
        for (h, v) in self.0.headers().iter() {
            let v = crate::sanitation::sanitize_header(h, v);
            hm.insert(h.as_str().to_string(), v.as_str().to_string());
        }

        let body_bytes = match self.0.body() {
            Body::Bytes(bytes) => bytes.clone(),
            _ => todo!("only bytes body is supported"),
        };

        // When serializing requests, we don't support using the JSON encoding type.
        // This is because when running in replay mode, we need to be able to compare the request against the recorded request.
        // We do this at a byte level, so we need to match it exactly.
        // TODO: We could probably do something optimistic here and use the JSON encoding type if we detect that the incoming request body looks identical to a standard-serialized serde_json::Value.
        let body = if body_bytes.len() == 0 {
            SerializedBody {
                encoding: BodyEncoding::Empty,
                content: serde_json::Value::Null,
            }
        } else if is_utf8_safe_content_type(
            self.0.headers().get_optional_str(&headers::CONTENT_TYPE),
        ) {
            let bytes = Vec::from(body_bytes);
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
                content: serde_json::Value::String(base64::encode(&body_bytes)),
            }
        };

        let serialized_request = SerializedMockRequest {
            url: sanitation::sanitize_url(self.0.url().as_str()),
            method: self.0.method().to_string(),
            headers: hm,
            body,
        };

        serialized_request.serialize(serializer)
    }
}

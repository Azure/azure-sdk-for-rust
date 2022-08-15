use azure_core::{Body, Method, Request};
use serde::de::Visitor;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::str::FromStr;
use url::Url;

const FIELDS: &[&str] = &["uri", "method", "headers", "body"];

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
        Ok(RequestDeserializer(deserializer.deserialize_struct(
            "Request",
            FIELDS,
            RequestVisitor,
        )?))
    }
}

struct RequestVisitor;

impl<'de> Visitor<'de> for RequestVisitor {
    type Value = Request;

    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("struct Request")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let uri: (&str, &str) = match map.next_entry()? {
            Some((a, b)) => (a, b),
            None => return Err(serde::de::Error::custom("missing uri")),
        };

        if uri.0 != FIELDS[0] {
            return Err(serde::de::Error::custom(format!(
                "unexpected field {}, expected {}",
                uri.0, FIELDS[0]
            )));
        }

        let method: (&str, &str) = match map.next_entry()? {
            Some((a, b)) => (a, b),
            None => return Err(serde::de::Error::custom("missing method")),
        };

        if method.0 != FIELDS[1] {
            return Err(serde::de::Error::custom(format!(
                "unexpected field {}, expected {}",
                method.0, FIELDS[1]
            )));
        }

        let headers: (&str, HashMap<&str, String>) = match map.next_entry()? {
            Some((a, b)) => (a, b),
            None => return Err(serde::de::Error::custom("missing header map")),
        };
        if headers.0 != FIELDS[2] {
            return Err(serde::de::Error::custom(format!(
                "unexpected field {}, expected {}",
                headers.0, FIELDS[2]
            )));
        }

        let body: (&str, String) = match map.next_entry()? {
            Some((a, b)) => (a, b),
            None => return Err(serde::de::Error::custom("missing body")),
        };
        if body.0 != FIELDS[3] {
            return Err(serde::de::Error::custom(format!(
                "unexpected field {}, expected {}",
                body.0, FIELDS[3]
            )));
        }

        let body = base64::decode(&body.1).map_err(serde::de::Error::custom)?;

        // `url` cannot be relative
        let url = Url::parse("http://example.com").unwrap();
        let url = url.join(uri.1).expect("expected a valid uri");
        let mut req = Self::Value::new(
            url,
            Method::from_str(method.1).expect("expected a valid HTTP method"),
        );
        for (k, v) in headers.1.into_iter() {
            req.insert_header(k.to_owned(), v);
        }

        req.set_body(bytes::Bytes::from(body));
        Ok(req)
    }
}

impl<'a> Serialize for RequestSerializer<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut hm = std::collections::BTreeMap::new();
        for (h, v) in self.0.headers().iter() {
            if h.as_str().to_lowercase() == "authorization" {
                hm.insert(h.as_str(), "<<STRIPPED>>");
            } else {
                hm.insert(h.as_str(), v.as_str());
            }
        }

        let mut state = serializer.serialize_struct("Request", 4)?;
        state.serialize_field(FIELDS[0], &self.0.path_and_query())?;
        state.serialize_field(FIELDS[1], &self.0.method().to_string())?;
        state.serialize_field(FIELDS[2], &hm)?;
        state.serialize_field(
            FIELDS[3],
            &match &self.0.body() {
                Body::Bytes(bytes) => base64::encode(bytes as &[u8]),
                Body::SeekableStream(_) => unimplemented!(),
            },
        )?;

        state.end()
    }
}

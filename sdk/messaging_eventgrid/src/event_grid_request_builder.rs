use crate::event_grid_request::EventGridRequest;
use http::{
    header::{HeaderName, HeaderValue, CONTENT_LENGTH, CONTENT_TYPE},
    request::Builder,
    Method, Request,
};
use std::convert::TryFrom;

pub struct EventGridRequestBuilder {
    builder: Builder,
}

impl EventGridRequestBuilder {
    pub fn new(method: Method, uri: &str) -> Self {
        EventGridRequestBuilder {
            builder: Request::builder().method(method).uri(uri),
        }
    }

    pub fn sas_key(self, key: &str) -> Self {
        self.optional_header("aeg-sas-key", Some(key))
    }

    pub fn body(
        self,
        body: Option<&str>,
        content_type: Option<&str>,
    ) -> Result<EventGridRequest, http::Error> {
        Ok(self
            .optional_header(CONTENT_TYPE, content_type)
            .optional_header(CONTENT_LENGTH, body.map(|b| b.len()))
            .builder
            .body(
                body.map(|b| Vec::from(b.as_bytes()).into())
                    .unwrap_or_default(),
            )?
            .into())
    }

    pub fn optional_header<K, V>(self, key: K, value: Option<V>) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        EventGridRequestBuilder {
            builder: match value {
                Some(value) => self.builder.header(key, value),
                None => self.builder,
            },
        }
    }
}

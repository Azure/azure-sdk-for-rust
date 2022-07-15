use crate::headers::{self, Header, HeaderValue};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContentType(HeaderValue);

impl ContentType {
    pub const APPLICATION_JSON: ContentType =
        ContentType(HeaderValue::from_static("application/json"));

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl From<&'static str> for ContentType {
    fn from(s: &'static str) -> Self {
        Self(s.into())
    }
}

impl From<String> for ContentType {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

impl From<&String> for ContentType {
    fn from(s: &String) -> Self {
        Self(s.into())
    }
}

impl ContentType {
    pub fn new(s: impl Into<ContentType>) -> Self {
        s.into()
    }
}

impl Header for ContentType {
    fn name(&self) -> headers::HeaderName {
        headers::CONTENT_TYPE
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.clone()
    }
}

use crate::headers::{self, Header};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContentType(std::borrow::Cow<'static, str>);

impl ContentType {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<&'static str> for ContentType {
    fn from(s: &'static str) -> Self {
        Self(std::borrow::Cow::Borrowed(s))
    }
}

impl From<String> for ContentType {
    fn from(s: String) -> Self {
        Self(std::borrow::Cow::Owned(s))
    }
}

impl From<&String> for ContentType {
    fn from(s: &String) -> Self {
        Self(std::borrow::Cow::Owned(s.clone()))
    }
}

impl ContentType {
    pub fn new(s: impl Into<ContentType>) -> Self {
        s.into()
    }
}

impl<'a> Header for ContentType {
    fn name(&self) -> headers::HeaderName {
        http::header::CONTENT_TYPE.into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_string().into()
    }
}
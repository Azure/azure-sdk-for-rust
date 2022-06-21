use crate::headers::{self, Header};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ContentEncoding(std::borrow::Cow<'static, str>);

impl ContentEncoding {
    pub fn as_str(&self) -> &str {
        self.0.as_ref()
    }

    const fn from_static(s: &'static str) -> Self {
        Self(std::borrow::Cow::Borrowed(s))
    }
}

impl From<&'static str> for ContentEncoding {
    fn from(s: &'static str) -> Self {
        Self::from_static(s)
    }
}

impl From<String> for ContentEncoding {
    fn from(s: String) -> Self {
        Self(std::borrow::Cow::Owned(s))
    }
}

impl From<&String> for ContentEncoding {
    fn from(s: &String) -> Self {
        Self(std::borrow::Cow::Owned(s.clone()))
    }
}

impl ContentEncoding {
    pub fn new(s: impl Into<ContentEncoding>) -> Self {
        s.into()
    }
}

impl<'a> Header for ContentEncoding {
    fn name(&self) -> headers::HeaderName {
        http::header::CONTENT_TYPE.into()
    }

    fn value(&self) -> headers::HeaderValue {
        self.0.to_string().into()
    }
}
